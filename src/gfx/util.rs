use {
    std::sync::Arc,
    vulkano::{
        instance::{Instance, PhysicalDevice, QueueFamily},
        swapchain::Surface,
    },
    vulkano_win::VkSurfaceBuild,
    winit::{event_loop::EventLoop, window::Window, window::WindowBuilder},
};

/// Initialize Vulkan capable window
pub fn init_vk_winit_window(
    event_loop: &EventLoop<()>,
    instance: Arc<Instance>,
) -> (EventLoop<()>, Arc<Surface<Window>>) {
    (
        EventLoop::new(),
        WindowBuilder::new()
            .build_vk_surface(event_loop, instance)
            .expect("Failed to create Vulkan window"),
    )
}

/// Get vk device to compute with
pub fn get_vk_physical_device<'a>(instance: &'a Arc<Instance>) -> PhysicalDevice {
    PhysicalDevice::enumerate(&instance)
        .next()
        .expect("No device available")
}

/// Get vk que capable of drawing graphics
pub fn get_graphics_capable_que_family<'a>(compute_device: &'a PhysicalDevice) -> QueueFamily<'a> {
    compute_device
        .queue_families()
        .find(|&q| q.supports_graphics())
        .expect("couldn't find a graphical queue family")
}

/// Print vk ques for device to stdout
pub fn print_vk_ques(device_list: &PhysicalDevice) {
    for family in device_list.queue_families() {
        println!(
            "Found a queue family with {:?} queue(s)",
            family.queues_count()
        );
    }
}
