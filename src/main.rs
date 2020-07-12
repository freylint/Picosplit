#![deny(missing_debug_implementations, missing_docs, clippy::all)]
//! Minimalist speedrun timer

mod cfg;
mod gfx;

use {
    cfg::Cfg,
    clap::{load_yaml, App},
    gfx::{shaders, util},
    std::path::Path,
    vulkano::{
        buffer::{BufferUsage, CpuAccessibleBuffer},
        command_buffer::{AutoCommandBufferBuilder, CommandBuffer},
        device::{Device, DeviceExtensions, Features},
        instance::{Instance, InstanceExtensions, PhysicalDevice},
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
            &DeviceExtensions::none(),
            [(queue_family, 0.5)].iter().cloned(),
        )
        .expect("failed to create device")
    };

    // Get Queue to render to
    let queue = queues.next().unwrap();

    let shader =
        shaders::basic_cmp::Shader::load(device.clone()).expect("failed to create shader module");

    // Create content to buffer.
    let data_iter = 0..65536;
    let data_buffer =
        CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), false, data_iter)
            .expect("failed to create buffer");

    // Create command buffer
    let mut builder = AutoCommandBufferBuilder::new(device.clone(), queue.family()).unwrap();

    // Copy data into buffer
    builder
        .copy_buffer(data_buffer.clone(), data_buffer)
        .unwrap();

    // Build buffer
    let command_buffer = builder.build().unwrap();

    // Execute command buffer
    let finished = command_buffer.execute(queue.clone()).unwrap();

    // Check compute operation completed successfully
    finished
        .then_signal_fence_and_flush()
        .unwrap()
        .wait(None)
        .unwrap();

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
