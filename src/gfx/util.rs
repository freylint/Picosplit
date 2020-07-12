use {
    std::sync::Arc,
    vulkano::{
        instance::{Instance, PhysicalDevice, PhysicalDevicesIter},
        swapchain::Surface,
    },
    vulkano_win::VkSurfaceBuild,
    winit::{event_loop::EventLoop, window::Window, window::WindowBuilder},
};

/// Initialize Vulkan capable window
pub fn init_vk_window(
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

pub fn print_vk_ques(device_list: &PhysicalDevice) {
    for family in device_list.queue_families() {
        println!(
            "Found a queue family with {:?} queue(s)",
            family.queues_count()
        );
    }
}
