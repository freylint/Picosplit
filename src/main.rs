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

    let _cfg = Cfg::init_cfg(Path::new("./res/cfg/cfg.toml"));

    // Create Vulkano Instance
    // TODO Nicer no vulkan output
    let instance = {
        let extensions = vulkano_win::required_extensions();
        Instance::new(None, &extensions, None).expect("failed to create Vulkan instance")
    };

    // Setup vulkan
    let (events_loop, surface) = util::init_vk_window(&EventLoop::new(), instance.clone());
    let physical = util::get_vk_physical_device(&instance);
    util::print_vk_ques(&physical);
    let queue_family = util::get_graphics_capable_que_family(&physical);

    // Get device and queues to render to
    let (device, mut queues) = {
        Device::new(
            physical,
            &Features::none(),
            &DeviceExtensions::supported_by_device(physical),
            [(queue_family, 0.5)].iter().cloned(),
        )
        .expect("Failed to create device")
    };

    // Get Queue to render to
    let queue = queues.next().unwrap();

    let shader =
        shaders::basic_cmp::Shader::load(device.clone()).expect("Failed to create shader module");

    // Create content to buffer.
    let data_iter = 0..65536;
    let data_buffer =
        CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), false, data_iter)
            .expect("failed to create buffer");

    let compute_pipeline = Arc::new(
        ComputePipeline::new(device.clone(), &shader.main_entry_point(), &())
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

    // Create compute command
    let mut builder = AutoCommandBufferBuilder::new(device.clone(), queue.family()).unwrap();

    // Build compute command
    let mut builder = AutoCommandBufferBuilder::new(device.clone(), queue.family()).unwrap();
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
    events_loop.run(|event, _, control_flow| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => {
            *control_flow = ControlFlow::Exit;
        }
        _ => (),
    });
}
