#![deny(missing_debug_implementations, missing_docs, clippy::all)]
//! Minimalist speedrun timer

mod cfg;

use {
    cfg::Cfg,
    clap::{load_yaml, App},
    std::path::Path,
    vulkano::{
        buffer::{BufferUsage, CpuAccessibleBuffer},
        command_buffer::{AutoCommandBufferBuilder, CommandBuffer},
        device::{Device, DeviceExtensions, Features},
        instance::{Instance, InstanceExtensions, PhysicalDevice},
    },
};

fn main() {
    // Parse command line args
    let cli_argfile = load_yaml!("../res/sys/cli.yaml");
    let _matches = App::from(cli_argfile).get_matches();

    let _cfg = Cfg::init_cfg(Path::new("./res/cfg/cfg.toml"));

    // Create Vulkano Instance
    // TODO Nicer no vulkan output
    let instance =
        Instance::new(None, &InstanceExtensions::none(), None).expect("failed to create instance");

    // TODO allow user to select device
    let physical = PhysicalDevice::enumerate(&instance)
        .next()
        .expect("no device available");

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
    let source_content = 0..64;
    let source =
        CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), false, source_content)
            .expect("failed to create buffer");

    // Create content to buffer
    let dest_content = (0..64).map(|_| 0);
    let dest =
        CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), false, dest_content)
            .expect("failed to create buffer");

    // Create command buffer
    let mut builder = AutoCommandBufferBuilder::new(device.clone(), queue.family()).unwrap();

    // Copy data into buffer
    builder.copy_buffer(source.clone(), dest.clone()).unwrap();
    
    // Build buffer
    let command_buffer = builder.build().unwrap();

    // Execute command buffer
    let finished = command_buffer.execute(queue.clone()).unwrap();

}
