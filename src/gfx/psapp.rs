use {
    crate::cfg::Cfg,
    std::sync::Arc,
    vulkano::{
        device::{Device, DeviceCreationError, DeviceExtensions, Features, QueuesIter},
        instance::{Instance, PhysicalDevice, QueueFamily},
        swapchain::Surface,
    },
    vulkano_win::{VkSurfaceBuild, CreationError},
    winit::{dpi::PhysicalSize, event_loop::EventLoop, window::Window, window::WindowBuilder},
};

/// Struct containing data required for application rendering and input handling
pub struct PSApp {
    pub vk_instance: Arc<Instance>,
    pub event_loop: EventLoop<()>,
    pub window: Arc<Surface<Window>>,
    pub device: Arc<Device>,
    pub queues: QueuesIter,
}

impl PSApp {
    pub fn init(config: &Cfg) -> Self {
        // Create Vulkano Instance
        // TODO Nicer output if no vulkan capable device
        let instance = {
            let extensions = vulkano_win::required_extensions();
            Instance::new(None, &extensions, None).expect("Failed to create Vulkan instance")
        };

        // Get physical device properties
        let physical = PhysicalDevice::enumerate(&instance)
            .next()
            .expect("No vulkan compatible device");
        Self::print_vk_ques(&physical);
        let queue_family = Self::get_graphics_capable_que_family(&physical)
            .expect("Physical device does not have a que capable of drawing graphics");

        let event_loop = EventLoop::new();
        let surface = Self::init_vk_winit_window(&event_loop, instance.clone(), config)
            .expect("Failed to create Vulkan capable window");

        // Get device and queues to render to
        let (device, mut queues) = Self::init_render_target(physical, queue_family)
            .expect("Failed to create vulkan device");

        #[allow(clippy::redundant_field_names)]
        PSApp {
            vk_instance: instance,
            event_loop,
            window: surface,
            device,
            queues,
        }
    }

    fn init_render_target(
        physical: PhysicalDevice,
        queue_family: QueueFamily,
    ) -> Result<(Arc<Device>, QueuesIter), DeviceCreationError> {
        Device::new(
            physical,
            &Features::none(),
            &DeviceExtensions::supported_by_device(physical),
            [(queue_family, 0.5)].iter().cloned(),
        )
    }

    /// Initialize Vulkan capable window
    fn init_vk_winit_window(
        event_loop: &EventLoop<()>,
        instance: Arc<Instance>,
        config: &Cfg,
    ) -> Result<Arc<Surface<Window>>, CreationError> {
        WindowBuilder::new()
            .with_title(&config.title)
            .with_inner_size(PhysicalSize::new(config.width, config.height))
            .build_vk_surface(&event_loop, instance)
    }

    /// Get vk que capable of drawing graphics
    fn get_graphics_capable_que_family<'a>(
        compute_device: &'a PhysicalDevice,
    ) -> Option<QueueFamily<'a>> {
        compute_device
            .queue_families()
            .find(|&q| q.supports_graphics())
    }

    /// Print vk ques for device to stdout
    fn print_vk_ques(device_list: &PhysicalDevice) {
        for family in device_list.queue_families() {
            println!(
                "Found a queue family with {:?} queue(s)",
                family.queues_count()
            );
        }
    }
}
