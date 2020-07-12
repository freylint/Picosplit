use {
    std::sync::Arc,
    vulkano::{instance::Instance, swapchain::Surface},
    vulkano_win::VkSurfaceBuild,
    winit::{event_loop::EventLoop, window::Window, window::WindowBuilder},
};

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
