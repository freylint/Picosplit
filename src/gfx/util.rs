use {
    crate::cfg::Cfg,
    std::sync::Arc,
    vulkano::{
        device::{Device, DeviceExtensions, Features, QueuesIter},
        instance::{Instance, PhysicalDevice, PhysicalDevicesIter, QueueFamily},
        swapchain::Surface,
    },
    vulkano_win::VkSurfaceBuild,
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
            Instance::new(None, &extensions, None).expect("failed to create Vulkan instance")
        };

        // Get physical device properties
        let physical = PhysicalDevice::enumerate(&instance).next().expect("No vulkan compatible device.");
        Self::print_vk_ques(&physical);
        let queue_family = Self::get_graphics_capable_que_family(&physical);

        let (event_loop, surface) = Self::init_vk_winit_window(instance.clone(), config);

        // Get device and queues to render to
        let (device, mut queues) = Self::init_render_target(physical, queue_family);

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
    ) -> (Arc<Device>, QueuesIter) {
        Device::new(
            physical,
            &Features::none(),
            &DeviceExtensions::supported_by_device(physical),
            [(queue_family, 0.5)].iter().cloned(),
        )
        .expect("Failed to create device")
    }

    /// Initialize Vulkan capable window
    fn init_vk_winit_window(
        instance: Arc<Instance>,
        config: &Cfg,
    ) -> (EventLoop<()>, Arc<Surface<Window>>) {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title(&config.title)
            .with_inner_size(PhysicalSize::new(config.width, config.height))
            .build_vk_surface(&event_loop, instance)
            .expect("Failed to create Vulkan window");
        (event_loop, window)
    }

    /// Get vk que capable of drawing graphics
    fn get_graphics_capable_que_family<'a>(compute_device: &'a PhysicalDevice) -> QueueFamily<'a> {
        compute_device
            .queue_families()
            .find(|&q| q.supports_graphics())
            .expect("couldn't find a graphical queue family")
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
