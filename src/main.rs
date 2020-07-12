#![deny(missing_debug_implementations, missing_docs, clippy::all)]
//! Minimalist speedrun timer

mod cfg;
mod gfx;

use {
    cfg::Cfg,
    clap::{load_yaml, App},
    gfx::{shaders, util},
    std::{path::Path, sync::Arc},
    vulkano::{
        buffer::{BufferUsage, CpuAccessibleBuffer},
        command_buffer::{AutoCommandBufferBuilder, CommandBuffer},
        descriptor::{descriptor_set::PersistentDescriptorSet, PipelineLayoutAbstract},
        device::{Device, DeviceExtensions, Features},
        instance::{Instance, InstanceExtensions, PhysicalDevice},
        pipeline::ComputePipeline,
        sync::GpuFuture,
    },
    vulkano_win::VkSurfaceBuild,
    winit::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    },
};

fn main() {
    // Parse command line args
    let cli_argfile = load_yaml!("../res/sys/cli.yaml");
    let _matches = App::from(cli_argfile).get_matches();

    let cfg = Cfg::init_cfg(Path::new("./res/cfg/cfg.toml"));

    // Setup vulkan window
    let mut app = util::PSApp::init(&cfg);

    // Get Queue to render to
    let queue = app.queues.next().unwrap();

    // Load basic compute shader
    let shader = shaders::basic_cmp::Shader::load(app.device.clone())
        .expect("Failed to create shader module");

    // Create content to buffer
    let data_iter = 0..65536;
    let data_buffer =
        CpuAccessibleBuffer::from_iter(app.device.clone(), BufferUsage::all(), false, data_iter)
            .expect("failed to create buffer");

    let compute_pipeline = Arc::new(
        ComputePipeline::new(app.device.clone(), &shader.main_entry_point(), &())
            .expect("failed to create compute pipeline"),
    );

    let layout = compute_pipeline.layout().descriptor_set_layout(0).unwrap();
    let set = Arc::new(
        PersistentDescriptorSet::start(layout.clone())
            .add_buffer(data_buffer.clone())
            .unwrap()
            .build()
            .unwrap(),
    );


    // Build compute command
    let mut builder = AutoCommandBufferBuilder::new(app.device.clone(), queue.family()).unwrap();
    builder
        .dispatch([1024, 1, 1], compute_pipeline.clone(), set.clone(), ())
        .unwrap();
    let command_buffer = builder.build().unwrap();

    // Wait for command to finish
    let finished = command_buffer.execute(queue.clone()).unwrap();

    // Check compute operation completed successfully
    finished
        .then_signal_fence_and_flush()
        .unwrap()
        .wait(None)
        .unwrap();

    let content = data_buffer.read().unwrap();
    for (n, val) in content.iter().enumerate() {
        assert_eq!(*val, n as u32 * 12);
    }

    println!("Everything succeeded!");

    // Main program loop
    // TODO handle other winit events
    #[allow(clippy::single_match)]
    app.event_loop.run(|event, _, control_flow| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => {
            *control_flow = ControlFlow::Exit;
        }
        _ => (),
    });
}
