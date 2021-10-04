pub mod error;

use crate::render::vulkan::{Vulkan, PresentMode};
use ash::{Entry, Instance};
use crate::render::vulkan::builder::error::BuildVulkanError;
use std::ffi::{CString, CStr, c_void};
use ash::vk;
use crate::window::Window;
use crate::common::SdlError;
use std::ptr::{null_mut, null};
use sdl2_sys::*;
use ash::vk::{Bool32, Handle};
use ash::extensions::khr;
use std::sync::Arc;
use crate::render::vulkan::core::{Core, ImageView, Semaphore};

#[derive(Debug)]
pub struct VulkanBuilder{
    app_name : CString,
    app_version : (u32,u32,u32),
    engine_name : CString,
    engine_version : (u32,u32,u32),
    api_version : u32,
    enable_debug : bool,
    debug_severity : vk::DebugUtilsMessageSeverityFlagsEXT,
    present_mode : vk::PresentModeKHR,
    format : vk::Format,
    color_space : vk::ColorSpaceKHR
}

impl Default for VulkanBuilder{
    fn default() -> Self {
        VulkanBuilder{
            app_name: CString::new("elikar app").unwrap(),
            app_version : (0,0,0),
            engine_name: CString::new("elikar").unwrap(),
            engine_version : (0,0,0),
            api_version : vk::API_VERSION_1_2,
            enable_debug: false,
            debug_severity: vk::DebugUtilsMessageSeverityFlagsEXT::empty(),
            present_mode: vk::PresentModeKHR::IMMEDIATE,
            format: vk::Format::B8G8R8A8_UNORM,
            color_space: vk::ColorSpaceKHR::SRGB_NONLINEAR
        }
    }
}

impl VulkanBuilder{

    pub fn app_name(mut self,name : &str) -> Self {
        self.app_name = CString::new(name).unwrap();
        self
    }

    pub fn app_version(mut self,version : (u32,u32,u32)) -> Self{
        self.app_version = version;
        self
    }

    pub fn engine_name(mut self,name : &str) -> Self {
        self.engine_name = CString::new(name).unwrap();
        self
    }

    pub fn engine_version(mut self,version : (u32,u32,u32)) -> Self {
        self.engine_version = version;
        self
    }

    pub fn enable_debug(mut self) -> Self {
        self.enable_debug = true;
        self
    }

    pub fn debug_all(mut self) -> Self {
        self.debug_severity = vk::DebugUtilsMessageSeverityFlagsEXT::all();
        self
    }

    pub fn debug_error(mut self) -> Self {
        self.debug_severity |= vk::DebugUtilsMessageSeverityFlagsEXT::ERROR;
        self
    }

    pub fn debug_warning(mut self) -> Self {
        self.debug_severity |= vk::DebugUtilsMessageSeverityFlagsEXT::WARNING;
        self
    }

    pub fn debug_info(mut self) -> Self {
        self.debug_severity |= vk::DebugUtilsMessageSeverityFlagsEXT::INFO;
        self
    }

    pub fn debug_verbose(mut self) -> Self {
        self.debug_severity |= vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE;
        self
    }

    pub fn present_mode(mut self,present_mode : PresentMode) -> Self {
        let present_mode = match present_mode{
            PresentMode::Immediate => vk::PresentModeKHR::IMMEDIATE,
            PresentMode::Mailbox => vk::PresentModeKHR::MAILBOX,
            PresentMode::FIFO => vk::PresentModeKHR::FIFO,
            PresentMode::FIFORelaxed => vk::PresentModeKHR::FIFO_RELAXED
        };
        self.present_mode = present_mode;
        self
    }

    pub fn build(self,window : &Window) -> Result<Vulkan,BuildVulkanError>{
        let entry = unsafe { Entry::new() } ?;

        // application info
        let app_info = vk::ApplicationInfo::builder()
            .application_name(self.app_name.as_c_str())
            .application_version(
                vk::make_api_version(
                    0,
                    self.app_version.0,
                    self.app_version.1,
                    self.app_version.2))
            .engine_name(self.engine_name.as_c_str())
            .engine_version(
                vk::make_api_version(
                    0,
                    self.engine_version.0,
                    self.engine_version.1,
                    self.engine_version.2))
            .api_version(self.api_version);

        // enable some layers and extensions
        let mut layers = vec![];
        let mut extensions = vec![];
        if self.enable_debug {
            layers.push(CString::new("VK_LAYER_KHRONOS_validation").unwrap());
            extensions.push(CString::new("VK_EXT_debug_utils").unwrap());
        }
        let extensions : Vec<CString> = [extensions,sdl_extensions(&window)?].concat();

        let layer_ptrs = layers.iter().map(|s|s.as_ptr()).collect::<Vec<_>>();
        let extension_ptrs = extensions.iter().map(|s|s.as_ptr()).collect::<Vec<_>>();

        let instance_info = vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .enabled_layer_names(&layer_ptrs)
            .enabled_extension_names(&extension_ptrs);

        // create instances
        let instance = unsafe {
            entry.create_instance(&instance_info,Option::None)
        }?;

        // use DebugUtils
        let mut debug_utils = Option::None;
        let mut messenger = Option::None;
        if self.enable_debug {
            let inner_debug_utils = ash::extensions::ext::DebugUtils::new(&entry,&instance);
            let messenger_info = vk::DebugUtilsMessengerCreateInfoEXT::builder()
                .message_severity(self.debug_severity)
                .message_type(vk::DebugUtilsMessageTypeFlagsEXT::all())
                .pfn_user_callback(Some(debug_callback));
            let inner_messenger = unsafe {
                inner_debug_utils.create_debug_utils_messenger(&messenger_info,Option::None)
            }?;

            debug_utils = Some(inner_debug_utils);
            messenger = Some(inner_messenger);
        }

        // enumerate all physical devices
        let physical_devices = unsafe {
            instance.enumerate_physical_devices()
        }?;

        // find a suitable GPU
        let physical_device = find_gpu(&instance,&physical_devices)
            .ok_or(BuildVulkanError::CannotFindSuitableGPU)?;

        let memory_properties = unsafe {
            instance.get_physical_device_memory_properties(physical_device)
        };

        let properties = unsafe {
            instance.get_physical_device_properties(physical_device)
        };

        // create device and queue
        let device_extensions = [CString::new("VK_KHR_swapchain").unwrap()];
        let device_extension_ptrs = device_extensions.iter()
            .map(|str|str.as_ptr())
            .collect::<Vec<_>>();

        let mut device_layers = vec![];
        if self.enable_debug {
            device_layers.push(CString::new("VK_LAYER_KHRONOS_validation").unwrap())
        }
        let device_layer_ptrs = device_layers.iter()
            .map(|str|str.as_ptr())
            .collect::<Vec<_>>();

        let queue_family_index = find_queue(&instance,&physical_device)
            .ok_or(BuildVulkanError::CannotFindSuitableQueue)?;

        let graphics_queue_priority = 1.0f32;
        let transfer_queue_priority = 0.7f32;
        let priorities = [graphics_queue_priority,transfer_queue_priority];

        let queue_info = vk::DeviceQueueCreateInfo::builder()
            .queue_family_index(queue_family_index)
            .queue_priorities(&priorities);
        let queue_infos = [queue_info.build()];

        let device_info = vk::DeviceCreateInfo::builder()
            .enabled_extension_names(&device_extension_ptrs)
            .enabled_layer_names(&device_layer_ptrs)
            .queue_create_infos(&queue_infos);

        let device = unsafe {
            instance.create_device(physical_device,&device_info,Option::None)
        }?;

        let graphics_queue = unsafe {
            device.get_device_queue(queue_family_index,0)
        };
        let transfer_queue = unsafe {
            device.get_device_queue(queue_family_index,1)
        };

        // create command pool
        let command_pool_info = vk::CommandPoolCreateInfo::builder()
            .queue_family_index(queue_family_index)
            .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER);
        let command_pool = unsafe {
            device.create_command_pool(&command_pool_info,Option::None)
        }?;

        let surface_manager = khr::Surface::new(&entry, &instance);
        let surface = sdl_surface(&window,&instance)?;

        // swapchain info
        let surface_capabilities = unsafe {
            surface_manager.get_physical_device_surface_capabilities(
                physical_device, surface)
        }?;
        let surface_formats = unsafe {
            surface_manager.get_physical_device_surface_formats(
                physical_device, surface)
        }?;
        let surface_present_modes = unsafe {
            surface_manager.get_physical_device_surface_present_modes(
                physical_device, surface)
        }?;

        if !unsafe {
            surface_manager.get_physical_device_surface_support(physical_device,queue_family_index,surface)
        }? {
            return Err(BuildVulkanError::CannotFindUsableFormat);
        }

        let format = surface_formats.iter()
            .cloned()
            .find(|format|
                format.format == self.format
                    && format.color_space == self.color_space)
            .ok_or(BuildVulkanError::CannotFindUsableFormat)?;

        let present_mode = surface_present_modes.iter()
            .cloned()
            .find(|mode|
                *mode == self.present_mode)
            .ok_or(BuildVulkanError::CannotSetPresentMode)?;

        let image_count = surface_capabilities.min_image_count + 1;
        let image_count = if surface_capabilities.max_image_count > 0 &&
            image_count > surface_capabilities.max_image_count {
            surface_capabilities.max_image_count
        } else {
            image_count
        };

        let swapchain_info = vk::SwapchainCreateInfoKHR::builder()
            .present_mode(present_mode)
            .surface(surface)
            .min_image_count(image_count)
            .image_extent(surface_capabilities.current_extent)
            .image_format(format.format)
            .image_color_space(format.color_space)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_array_layers(1)
            .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
            .clipped(true)
            .pre_transform(surface_capabilities.current_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE);
        let swapchain_manager = khr::Swapchain::new(&instance,&device);
        let swapchain = unsafe {
            swapchain_manager.create_swapchain(&swapchain_info,Option::None)
        }?;

        let swapchain_images = unsafe {
            swapchain_manager.get_swapchain_images(swapchain)
        }?;

        // create swapchain image views
        let mut swapchain_image_views = Vec::new();
        for image in swapchain_images {
            let image_view_info = vk::ImageViewCreateInfo::builder()
                .image(image)
                .view_type(vk::ImageViewType::TYPE_2D)
                .format(format.format)
                .components(vk::ComponentMapping {
                    r: vk::ComponentSwizzle::IDENTITY,
                    g: vk::ComponentSwizzle::IDENTITY,
                    b: vk::ComponentSwizzle::IDENTITY,
                    a: vk::ComponentSwizzle::IDENTITY
                })
                .subresource_range(vk::ImageSubresourceRange{
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    base_mip_level: 0,
                    level_count: 1,
                    base_array_layer: 0,
                    layer_count: 1
                });
            let view = unsafe {
                device.create_image_view(&image_view_info,Option::None)
            }?;
            swapchain_image_views.push(view);
        }

        let descriptor_pool_info = vk::DescriptorPoolCreateInfo::builder()
            .flags(vk::DescriptorPoolCreateFlags::FREE_DESCRIPTOR_SET)
            .max_sets(24)
            .pool_sizes(&[vk::DescriptorPoolSize{
                ty: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
                descriptor_count: 8
            },vk::DescriptorPoolSize{
                ty : vk::DescriptorType::SAMPLER,
                descriptor_count: 8,
            },vk::DescriptorPoolSize{
                ty: vk::DescriptorType::UNIFORM_BUFFER,
                descriptor_count: 8,
            }]);

        let descriptor_pool = unsafe {
            device.create_descriptor_pool(&descriptor_pool_info,Option::None)
        }?;

        let core = Arc::new(Core{
            entry,
            instance,
            debug_utils,
            messenger,
            physical_device,
            properties,
            memory_properties,
            device,
            queue_family_index,
            graphics_queue,
            transfer_queue,
            command_pool,
            descriptor_pool,
            surface_manager,
            swapchain_manager,
        });

        let swapchain_image_views = swapchain_image_views.into_iter()
            .map(|image_view| {
                ImageView {
                    core : core.clone(),
                    image_view
                }
            })
            .collect::<Vec<_>>();

        let image_available_semaphore = Semaphore::new(core.clone())?;
        let render_finish_semaphore = Semaphore::new(core.clone())?;

        Ok(Vulkan{
            core : core.clone(),
            surface,
            surface_format : format,
            surface_extent : surface_capabilities.current_extent,
            swapchain,
            swapchain_image_views,
            render_command_buffers: vec![],
            image_available_semaphore,
            render_finish_semaphore,
            window_id : window.id()
        })
    }
}


unsafe extern "system" fn debug_callback(
    message_severity : vk::DebugUtilsMessageSeverityFlagsEXT,
    #[allow(unused)]
    message_types : vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data : *const vk::DebugUtilsMessengerCallbackDataEXT,
    #[allow(unused)]
    p_user_data : *mut c_void) -> Bool32 {
    if message_severity == vk::DebugUtilsMessageSeverityFlagsEXT::INFO {
        println!("[Info]:{}",CStr::from_ptr((&*p_callback_data).p_message).to_str().unwrap())
    } else if message_severity == vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE {
        println!("[Verbose]:{}",CStr::from_ptr((&*p_callback_data).p_message).to_str().unwrap())
    } else if message_severity == vk::DebugUtilsMessageSeverityFlagsEXT::WARNING{
        println!("[Warning]:{}",CStr::from_ptr((&*p_callback_data).p_message).to_str().unwrap())
    } else if message_severity == vk::DebugUtilsMessageSeverityFlagsEXT::ERROR {
        println!("[Error]:{}",CStr::from_ptr((&*p_callback_data).p_message).to_str().unwrap())
    }
    vk::FALSE
}

fn sdl_surface(window : &Window,instance : &Instance) -> Result<vk::SurfaceKHR,SdlError> {
    let mut surface : u64 = 0;
    if unsafe { SDL_Vulkan_CreateSurface(window.window_ptr(),instance.handle().as_raw() as usize,&mut surface) } == SDL_bool::SDL_TRUE {
        Ok(vk::SurfaceKHR::from_raw(surface))
    } else {
        Err(SdlError::get())
    }
}

fn sdl_extensions(window : &Window) -> Result<Vec<CString>,SdlError> {
    let mut count = 0;
    let mut extensions = vec![];
    if unsafe { SDL_Vulkan_GetInstanceExtensions(window.window_ptr(),&mut count,null_mut()) } == SDL_bool::SDL_TRUE {
        for _ in 0..count {
            extensions.push(null())
        }
        if unsafe { SDL_Vulkan_GetInstanceExtensions(window.window_ptr(),&mut count,extensions.as_mut_ptr()) } == SDL_bool::SDL_TRUE {
            return Ok(extensions
                .iter()
                .map(|&ptr|{
                    unsafe {
                        CStr::from_ptr(ptr)
                    }.to_owned()
                })
                .collect());
        }
    }
    Err(SdlError::get())
}

fn find_gpu(instance : &Instance,physical_devices : &[vk::PhysicalDevice]) -> Option<vk::PhysicalDevice> {
    for physical_device in physical_devices {
        let properties = unsafe {
            instance.get_physical_device_properties(*physical_device)
        };
        if properties.device_type == vk::PhysicalDeviceType::DISCRETE_GPU {
            return Some(*physical_device);
        }
    }
    Option::None
}

fn find_queue(instance : &Instance,physical_device : &vk::PhysicalDevice) -> Option<u32>{
    let properties = unsafe {
        instance.get_physical_device_queue_family_properties(*physical_device)
    };
    for (index,properties) in properties.iter().enumerate() {
        if properties.queue_flags.contains(vk::QueueFlags::GRAPHICS) &&
            properties.queue_flags.contains(vk::QueueFlags::TRANSFER) &&
            // we need 2 queues at least
            properties.queue_count > 1 {
            return Option::Some(index as _);
        }
    }
    Option::None
}