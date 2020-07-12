#![deny(missing_debug_implementations, missing_docs, clippy::all)]
//! Minimalist speedrun timer

mod cfg;
mod graphics;

use {
    cfg::Cfg,
    clap::{load_yaml, App},
    graphics::init_vk_window,
    std::path::Path,
    vulkano::{
        buffer::{BufferUsage, CpuAccessibleBuffer},
        command_buffer::{AutoCommandBufferBuilder, CommandBuffer},
        device::{Device, DeviceExtensions, Features},
        instance::{Instance, InstanceExtensions, PhysicalDevice},
        sync::GpuFuture,
    },
    vulkano_win::VkSurfaceBuild,
    winit::{event_loop::EventLoop, window::WindowBuilder},
};

fn main() {
    // Parse command line args
    let cli_argfile = load_yaml!("../res/sys/cli.yaml");
    let _matches = App::from(cli_argfile).get_matches();

    let _cfg = Cfg::init_cfg(Path::new("./res/cfg/cfg.toml"));

    let events_loop = EventLoop::new();

    // Create Vulkano Instance
    // TODO Nicer no vulkan output
    let instance =
        Instance::new(None, &InstanceExtensions::none(), None).expect("Failed to create instance");

    // Setup vulkan window
    //let (event_loop, surface) = init_vk_window(&EventLoop::new(), instance.clone());

    
    let event_loop = EventLoop::new();
    let surface = WindowBuilder::new()
        .build_vk_surface(&events_loop, instance.clone())
        .expect("Failed to create Vulkan window");
    

    // TODO allow user to select device
    let physical = PhysicalDevice::enumerate(&instance)
        .next()
        .expect("No device available");

    // List ques
    for family in physical.queue_families() {
        println!(
            "Found a queue family with {:?} queue(s)",
            family.queues_count()
        );
    }

    // Get que family that supports Graphics
    let queue_family = physical
        .queue_families()
        .find(|&q| q.supports_graphics())
        .expect("couldn't find a graphical queue family");

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

    /*
    // Enter main loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
    */
}
