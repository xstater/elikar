use ash::{Entry, Instance, Device};
use ash::vk::{self, ApplicationInfo, make_api_version, InstanceCreateInfo, PhysicalDeviceType, DeviceCreateInfo, DeviceQueueCreateInfo, QueueFlags, DebugUtilsMessengerCreateInfoEXT, DebugUtilsMessageSeverityFlagsEXT, DebugUtilsMessageTypeFlagsEXT, DebugUtilsMessengerCallbackDataEXT, Bool32, FALSE, SurfaceKHR, Handle, SwapchainCreateInfoKHR, Format, ColorSpaceKHR, PresentModeKHR, ImageUsageFlags, SharingMode, CompositeAlphaFlagsKHR, ImageViewCreateInfo, ImageViewType, ComponentMapping, ComponentSwizzle, ImageSubresourceRange, ImageAspectFlags, RenderPassCreateInfo, AttachmentDescription, SampleCountFlags, AttachmentLoadOp, AttachmentStoreOp, ImageLayout, SubpassDescription, AttachmentReference, PipelineBindPoint, SubpassDependency, SUBPASS_EXTERNAL, PipelineStageFlags, AccessFlags, ShaderModuleCreateInfo, FramebufferCreateInfo,  PipelineShaderStageCreateInfo, ShaderStageFlags, PipelineVertexInputStateCreateInfo, PipelineInputAssemblyStateCreateInfo, PrimitiveTopology, Viewport, Rect2D, Offset2D, PipelineViewportStateCreateInfo, PipelineRasterizationStateCreateInfo, PolygonMode, CullModeFlags, FrontFace, PipelineMultisampleStateCreateInfo, PipelineColorBlendAttachmentState, ColorComponentFlags, BlendFactor, BlendOp, PipelineColorBlendStateCreateInfo, LogicOp, DynamicState, PipelineDynamicStateCreateInfo, PipelineLayoutCreateInfo, GraphicsPipelineCreateInfo, PipelineCache, CommandPoolCreateInfo, CommandBufferAllocateInfo, CommandBufferLevel, CommandBufferBeginInfo, RenderPassBeginInfo, ClearValue, ClearColorValue, SubpassContents, SemaphoreCreateInfo, SwapchainKHR, Semaphore, Fence, SubmitInfo, CommandBuffer, Queue, PresentInfoKHR, VertexInputBindingDescription, VertexInputRate, VertexInputAttributeDescription, BufferCreateInfo, BufferUsageFlags, MemoryPropertyFlags, MemoryAllocateInfo, MemoryMapFlags};
use std::ffi::{CStr, CString, c_void};
use std::os::raw::c_char;
use sdl2_sys::*;
use std::ptr::{null_mut, null};
use ash::extensions::ext::DebugUtils;
use std::ops::BitOr;
use ash::extensions::khr::{Surface, Swapchain};
use std::fs::File;
use ash::util::{read_spv, Align};
use xecs::System;
use xecs::system::End;
use std::cell::{RefMut, Ref};
use std::sync::Arc;
use std::mem::align_of;
use elikar::window::Window;
use elikar::common::SdlError;
use elikar::common::Result as ElikarResult;
use elikar::{Elikar, ElikarStates, window};
use elikar::events::PollEvents;
use std::convert::Infallible;

macro_rules! offset_of {
    ($base:path, $field:ident) => {{
        #[allow(unused_unsafe)]
        unsafe {
            let b: $base = std::mem::zeroed();
            (&b.$field as *const _ as isize) - (&b as *const _ as isize)
        }
    }};
}

#[derive(Copy, Clone,Debug)]
#[repr(C)]
struct Vertex {
    position : [f32;2],
    color : [f32;3]
}

const VERTICES : [Vertex;3] = [
    Vertex {
        position: [0.0,-0.8],
        // position : [-1.0,1.0],
        color: [0.8,0.2,0.2]
    },
    Vertex {
        position : [0.5,0.5],
        // position : [1.0,1.0],
        color : [0.2,0.8,0.2]
    },
    Vertex {
        position : [-0.5,0.5],
        // position : [0.0,-1.0],
        color : [0.2,0.2,0.8]
    }
];

fn sdl_extensions(window : &Window) -> ElikarResult<Vec<CString>> {
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

fn sdl_surface(window : &Window,instance : &Instance) -> ElikarResult<SurfaceKHR> {
    let mut surface : u64 = 0;
    if unsafe { SDL_Vulkan_CreateSurface(window.window_ptr(),instance.handle().as_raw() as usize,&mut surface) } == SDL_bool::SDL_TRUE {
        Ok(SurfaceKHR::from_raw(surface))
    } else {
        Err(SdlError::get())
    }
}

#[derive(Debug,Copy,Clone)]
enum GpuType {
    Discrete,
    Integrated,
    Virtual,
    Cpu,
    Other
}

impl From<PhysicalDeviceType> for GpuType {
    fn from(ty : PhysicalDeviceType) -> Self {
        if ty == PhysicalDeviceType::DISCRETE_GPU {
            GpuType::Discrete
        } else if ty == PhysicalDeviceType::INTEGRATED_GPU {
            GpuType::Integrated
        } else if ty == PhysicalDeviceType::VIRTUAL_GPU {
            GpuType::Virtual
        } else if ty == PhysicalDeviceType::CPU {
            GpuType::Cpu
        } else {
            GpuType::Other
        }
    }
}

unsafe extern "system" fn debug_callback(
    message_severity : DebugUtilsMessageSeverityFlagsEXT,
    #[allow(unused)]
    message_types : DebugUtilsMessageTypeFlagsEXT,
    p_callback_data : *const DebugUtilsMessengerCallbackDataEXT,
    #[allow(unused)]
    p_user_data : *mut c_void) -> Bool32 {
    if message_severity == DebugUtilsMessageSeverityFlagsEXT::INFO {
        println!("[Info]:{}",CStr::from_ptr((&*p_callback_data).p_message).to_str().unwrap())
    } else if message_severity == DebugUtilsMessageSeverityFlagsEXT::VERBOSE {
        println!("[Verbose]:{}",CStr::from_ptr((&*p_callback_data).p_message).to_str().unwrap())
    } else if message_severity == DebugUtilsMessageSeverityFlagsEXT::WARNING{
        println!("[Warning]:{}",CStr::from_ptr((&*p_callback_data).p_message).to_str().unwrap())
    } else if message_severity == DebugUtilsMessageSeverityFlagsEXT::ERROR {
        println!("[Error]:{}",CStr::from_ptr((&*p_callback_data).p_message).to_str().unwrap())
    }
    FALSE
}

fn main() {
    let mut game = Elikar::new().unwrap();

    let window = {
        let mut manager = game.current_stage_ref()
            .system_data_mut::<window::Manager>();
        manager.create_window()
            .title("elikar vulkan")
            .vulkan()
            .build()
            .unwrap()
            .id()
    };

    let entry = unsafe { Entry::new() }.unwrap();

    // let layers = entry.enumerate_instance_layer_properties().unwrap();
    // let extensions = entry.enumerate_instance_extension_properties().unwrap();

    // dbg!(&layers);
    // dbg!(&extensions);

    let app_name = CString::new("elikar vulkan test")
        .unwrap();
    let engine_name = CString::new("elikar")
        .unwrap();

    let app_info = ApplicationInfo::builder()
        .application_name(app_name.as_c_str())
        .application_version(make_api_version(0,0, 1,0))
        .engine_name(engine_name.as_c_str())
        .engine_version(make_api_version(0,0,1,0))
        .api_version(vk::API_VERSION_1_2);

    let layers = ["VK_LAYER_KHRONOS_validation"]
        .iter()
        .map(|&str| CString::new(str).unwrap() )
        .collect::<Vec<_>>();
    let layers_ptr = layers
        .iter()
        .map(|str| str.as_ptr() as *const c_char)
        .collect::<Vec<_>>();

    let mut extensions = {
        let manager = game.current_stage_ref()
            .system_data_ref::<window::Manager>();
        sdl_extensions(manager.window_ref(window).unwrap()).unwrap()
    };
    extensions.push(CString::new("VK_EXT_debug_utils").unwrap());

    dbg!(&extensions);

    let extensions_ptr = extensions
        .iter()
        .map(|string|{
            string.as_ptr() as *const c_char
        })
        .collect::<Vec<_>>();

    // create instance
    let instance_info = InstanceCreateInfo::builder()
        .application_info(&app_info)
        .enabled_layer_names(layers_ptr.as_slice())
        .enabled_extension_names(extensions_ptr.as_slice());

    let instance = unsafe {
        entry.create_instance(&instance_info,Option::None)
    }.unwrap();

    // debug utils
    let debug_utils = DebugUtils::new(&entry,&instance);
    let debug_utils_messenger_info = DebugUtilsMessengerCreateInfoEXT::builder()
        .message_severity(DebugUtilsMessageSeverityFlagsEXT::empty()
            .bitor(DebugUtilsMessageSeverityFlagsEXT::ERROR)
            .bitor(DebugUtilsMessageSeverityFlagsEXT::WARNING)
            .bitor(DebugUtilsMessageSeverityFlagsEXT::INFO))
        .message_type(DebugUtilsMessageTypeFlagsEXT::all())
        .pfn_user_callback(Some(debug_callback));
    let debug_utils_messenger = unsafe {
        debug_utils
            .create_debug_utils_messenger(&debug_utils_messenger_info, Option::None)
    }.unwrap();

    let physical_devices = unsafe {
        instance.enumerate_physical_devices()
    }.unwrap();

    #[allow(unused)]
    let physical_devices_info = physical_devices
    .iter()
    .map(|physical_device|{
        let properties = unsafe {
            instance.get_physical_device_properties(*physical_device)
        };
        let name = unsafe {
            CStr::from_ptr(properties.device_name.as_ptr())
        }.to_str()
            .unwrap()
            .to_owned();
        (name,GpuType::from(properties.device_type))
    })
    .collect::<Vec<_>>();

    // dbg!(physical_devices_info);

    // simply choose the first physical device
    let physical_device = *physical_devices.first().unwrap();

    // create surface
    let surface = {
        let manager = game.current_stage_ref()
            .system_data_ref::<window::Manager>();
        sdl_surface(manager.window_ref(window).unwrap(),&instance).unwrap()
    };
    let surface_manager = Surface::new(&entry,&instance);

    #[allow(unused)]
        let device_extensions = unsafe {
        instance.enumerate_device_extension_properties(physical_device)
    }.unwrap();
    #[allow(unused)]
        let device_layers = unsafe {
        instance.enumerate_device_layer_properties(physical_device)
    }.unwrap();
    let device_features = unsafe {
        instance.get_physical_device_features(physical_device)
    };
    let queue_family_properties = unsafe {
        instance.get_physical_device_queue_family_properties(physical_device)
    };
    #[allow(unused)]
        let memory_properties = unsafe {
        instance.get_physical_device_memory_properties(physical_device)
    };

    // dbg!(&device_extensions);
    // dbg!(&device_layers);
    // dbg!(&device_features);
    // dbg!(&queue_family_properties);

    let device_extensions = ["VK_KHR_swapchain"]
        .iter()
        .map(|&str|CString::new(str).unwrap())
        .collect::<Vec<_>>();
    let device_extensions_ptr = device_extensions
        .iter()
        .map(|str|str.as_ptr())
        .collect::<Vec<_>>();

    let device_layers = ["VK_LAYER_KHRONOS_validation"]
        .iter()
        .map(|&str|CString::new(str).unwrap())
        .collect::<Vec<_>>();
    let device_layers_ptr = device_layers
        .iter()
        .map(|str|str.as_ptr())
        .collect::<Vec<_>>();

    let device_queue_family_index = queue_family_properties
        .iter()
        .enumerate()
        .find(|(index,properties)|{
            properties.queue_count > 0 &&
                properties.queue_flags.contains(QueueFlags::GRAPHICS) &&
                unsafe {
                    surface_manager.get_physical_device_surface_support(physical_device, *index as u32, surface)
                }.unwrap()
        })
        .map(|(index,_)| index)
        .unwrap();

    let device_queue_priorities = vec![1.0_f32];

    let device_queue_info = DeviceQueueCreateInfo::builder()
        .queue_family_index(device_queue_family_index as u32)
        .queue_priorities(device_queue_priorities.as_slice());

    let queue_create_info = vec![device_queue_info.build()];

    let device_info = DeviceCreateInfo::builder()
        .enabled_layer_names(device_layers_ptr.as_slice())
        .enabled_extension_names(device_extensions_ptr.as_slice())
        .queue_create_infos(queue_create_info.as_slice())
        .enabled_features(&device_features);

    let device = unsafe {
        instance.create_device(physical_device,&device_info,Option::None)
    }.unwrap();

    let queue = unsafe {
        device.get_device_queue(device_queue_family_index as u32, 0)
    };


    // Get swapchain surface info
    let surface_capabilities = unsafe {
        surface_manager.get_physical_device_surface_capabilities(physical_device,surface)
    }.unwrap();
    let surface_formats = unsafe {
        surface_manager.get_physical_device_surface_formats(physical_device,surface)
    }.unwrap();
    let surface_present_modes = unsafe {
        surface_manager.get_physical_device_surface_present_modes(physical_device,surface)
    }.unwrap();

    dbg!(&surface_capabilities);
    dbg!(&surface_formats);
    dbg!(&surface_present_modes);

    // choose surface format
    let surface_format = surface_formats
        .iter()
        .cloned()
        .find(|format|{
            format.format == Format::B8G8R8A8_UNORM &&
                format.color_space == ColorSpaceKHR::SRGB_NONLINEAR
        }).unwrap();

    // choose present mode
    let present_mode = surface_present_modes
        .iter()
        .cloned()
        .find(|mode|{
            *mode == PresentModeKHR::MAILBOX
        })
        .unwrap_or(PresentModeKHR::IMMEDIATE);

    // choose swap extent
    // just simply choose the current extent
    let extent = surface_capabilities.current_extent;

    // set image count
    let image_count = surface_capabilities.min_image_count + 1;
    let image_count = if surface_capabilities.max_image_count > 0 && image_count > surface_capabilities.max_image_count {
        surface_capabilities.max_image_count
    } else {
        image_count
    };

    // create swapchain
    let swapchain_info = SwapchainCreateInfoKHR::builder()
        .surface(surface)
        .image_format(surface_format.format)
        .image_extent(extent)
        .image_color_space(surface_format.color_space)
        .min_image_count(image_count)
        .present_mode(present_mode)
        .image_usage(ImageUsageFlags::COLOR_ATTACHMENT)
        .image_array_layers(1)
        .image_sharing_mode(SharingMode::EXCLUSIVE)
        .clipped(true)
        .pre_transform(surface_capabilities.current_transform)
        .composite_alpha(CompositeAlphaFlagsKHR::OPAQUE);
    let swapchain_manager = Swapchain::new(&instance,&device);
    let swapchain = unsafe {
        swapchain_manager.create_swapchain(&swapchain_info,Option::None)
    }.unwrap();

    // get swapchain images
    let present_images = unsafe {
        swapchain_manager.get_swapchain_images(swapchain)
    }.unwrap();

    // create swapchain image views
    let present_image_views = present_images
        .iter()
        .map(|image| {
            let image_view_info = ImageViewCreateInfo::builder()
                .image(*image)
                .view_type(ImageViewType::TYPE_2D)
                .format(surface_format.format)
                .components(ComponentMapping::builder()
                    .r(ComponentSwizzle::IDENTITY)
                    .g(ComponentSwizzle::IDENTITY)
                    .b(ComponentSwizzle::IDENTITY)
                    .a(ComponentSwizzle::IDENTITY)
                    .build())
                .subresource_range(ImageSubresourceRange::builder()
                    .aspect_mask(ImageAspectFlags::COLOR)
                    .base_mip_level(0)
                    .level_count(1)
                    .base_array_layer(0)
                    .layer_count(1)
                    .build());
            unsafe {
                device.create_image_view(&image_view_info,Option::None)
            }.unwrap()
        })
        .collect::<Vec<_>>();

    // vertex buffer
    let vertex_buffer_info = BufferCreateInfo::builder()
        .size(dbg!(3 * std::mem::size_of::<Vertex>()) as u64)
        .usage(BufferUsageFlags::VERTEX_BUFFER)
        .sharing_mode(SharingMode::EXCLUSIVE);
    let vertex_buffer = unsafe {
        device.create_buffer(&vertex_buffer_info,Option::None)
    }.unwrap();

    // get buffer requirements
    let vertex_buffer_reqs = unsafe {
        device.get_buffer_memory_requirements(vertex_buffer)
    };

    dbg!(&vertex_buffer_reqs);
    // find memory type
    let memory_type_index = memory_properties.memory_types
        .iter()
        .enumerate()
        .find(|(index,memory_type)|{
            let properties = MemoryPropertyFlags::HOST_VISIBLE | MemoryPropertyFlags::HOST_COHERENT;
            (vertex_buffer_reqs.memory_type_bits & (1 << *index)) != 0 &&
                (memory_type.property_flags & properties == properties)
        }).map(|(index,_)| index)
        .unwrap();

    dbg!(memory_type_index);

    // allocate buffer memory
    let memory_allocate_info = MemoryAllocateInfo::builder()
        .memory_type_index(memory_type_index as _)
        .allocation_size(vertex_buffer_reqs.size);
    let memory = unsafe {
        device.allocate_memory(&memory_allocate_info,Option::None)
    }.unwrap();

    // bind buffer memory
    unsafe {
        device.bind_buffer_memory(vertex_buffer,memory,0)
    }.unwrap();

    // copy data
    let vertex_ptr = unsafe {
        device.map_memory(
            memory,
            0,
            vertex_buffer_reqs.size,
            MemoryMapFlags::empty())
    }.unwrap();
    let mut vertex_slice = unsafe {
        Align::new(
            vertex_ptr,
            dbg!(align_of::<Vertex>()) as u64,
            // vertex_buffer_reqs.alignment,
            vertex_buffer_reqs.size
            // (std::mem::size_of::<Vertex>() * VERTICES.len()) as u64
        )
    };
    vertex_slice.copy_from_slice(&VERTICES);
    unsafe {
        device.unmap_memory(memory)
    };


    // render pass info
    let attachments = [
        AttachmentDescription {
            format: surface_format.format,
            samples : SampleCountFlags::TYPE_1,
            load_op : AttachmentLoadOp::CLEAR,
            store_op : AttachmentStoreOp::STORE,
            final_layout : ImageLayout::PRESENT_SRC_KHR,
            .. Default::default()
        }
    ];

    let color_attachment_refs = [
        AttachmentReference {
            attachment : 0,
            layout : ImageLayout::COLOR_ATTACHMENT_OPTIMAL
        }
    ];

    let dependencies = [
        SubpassDependency{
            src_subpass : SUBPASS_EXTERNAL,
            src_stage_mask : PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            dst_access_mask :
            AccessFlags::COLOR_ATTACHMENT_READ |
                AccessFlags::COLOR_ATTACHMENT_WRITE,
            dst_stage_mask : PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            .. Default::default()
        }
    ];

    let subpasses = [
        SubpassDescription::builder()
            .color_attachments(&color_attachment_refs)
            .pipeline_bind_point(PipelineBindPoint::GRAPHICS)
            .build()
    ];

    let render_pass_info = RenderPassCreateInfo::builder()
        .attachments(&attachments)
        .subpasses(&subpasses)
        .dependencies(&dependencies);

    // create render pass
    let render_pass = unsafe {
        device.create_render_pass(&render_pass_info,Option::None)
    }.unwrap();

    // read spir-v
    let mut vs_file = File::open("./shaders/vulkan_test/vert.spv").unwrap();
    let mut fs_file = File::open("./shaders/vulkan_test/frag.spv").unwrap();
    let vs_spirv = read_spv(&mut vs_file).unwrap();
    let fs_spirv = read_spv(&mut fs_file).unwrap();

    let vert_shader_info = ShaderModuleCreateInfo::builder()
        .code(vs_spirv.as_slice());
    let frag_shader_info = ShaderModuleCreateInfo::builder()
        .code(fs_spirv.as_slice());

    // create shader module
    let vert_shader = unsafe {
        device.create_shader_module(&vert_shader_info,Option::None)
    }.unwrap();
    let frag_shader = unsafe {
        device.create_shader_module(&frag_shader_info,Option::None)
    }.unwrap();

    // shader stage
    let function_name = CString::new("main").unwrap();
    let shader_stages = [
        PipelineShaderStageCreateInfo::builder()
            .module(vert_shader)
            .stage(ShaderStageFlags::VERTEX)
            .name(function_name.as_c_str())
            .build(),
        PipelineShaderStageCreateInfo::builder()
            .module(frag_shader)
            .stage(ShaderStageFlags::FRAGMENT)
            .name(function_name.as_c_str())
            .build()
    ];

    // vertex binging
    let vertex_binding = VertexInputBindingDescription::builder()
        .binding(0)
        .stride(std::mem::size_of::<Vertex>() as _)
        .input_rate(VertexInputRate::VERTEX);
    let vertex_bindings = [vertex_binding.build()];

    // vertex attributes
    let vertex_position = VertexInputAttributeDescription::builder()
        .binding(0)
        .location(0)
        .format(Format::R32G32_SFLOAT)
        .offset(offset_of!(Vertex,position) as u32);
    let vertex_color = VertexInputAttributeDescription::builder()
        .binding(0)
        .location(1)
        .format(Format::R32G32B32_SFLOAT)
        .offset(dbg!(offset_of!(Vertex,color)) as _);
    let vertex_attributes = [vertex_position.build(),vertex_color.build()];

    // vertex input
    let vertex_input_info = PipelineVertexInputStateCreateInfo::builder()
        .vertex_binding_descriptions(&vertex_bindings)
        .vertex_attribute_descriptions(&vertex_attributes);

    // input assembly
    let input_assembly_info = PipelineInputAssemblyStateCreateInfo::builder()
        .topology(PrimitiveTopology::TRIANGLE_LIST)
        .primitive_restart_enable(false);

    // viewport
    let viewports = [Viewport {
        x : 0.0,
        y : 0.0,
        width : extent.width as _,
        height : extent.height as _,
        min_depth : 0.0,
        max_depth : 1.0
    }];

    // scissor
    let scissors = [Rect2D{
        offset  : Offset2D{
            x: 0,
            y: 0
        },
        extent
    }];
    // viewport & scissor
    let viewport_info = PipelineViewportStateCreateInfo::builder()
        .viewports(&viewports)
        .scissors(&scissors);

    // rasterization
    let rasterization_info = PipelineRasterizationStateCreateInfo::builder()
        .depth_clamp_enable(false)
        .rasterizer_discard_enable(false)
        .polygon_mode(PolygonMode::FILL)
        .line_width(1.0)
        .cull_mode(CullModeFlags::NONE)
        .front_face(FrontFace::CLOCKWISE)
        .depth_bias_constant_factor(0.0)
        .depth_bias_clamp(0.0)
        .depth_bias_slope_factor(0.0);

    // multisample
    let multisample_info = PipelineMultisampleStateCreateInfo::builder()
        .sample_shading_enable(false)
        .rasterization_samples(SampleCountFlags::TYPE_1)
        .min_sample_shading(1.0)
        .alpha_to_coverage_enable(false)
        .alpha_to_one_enable(false);

    // color blend
    let color_blend_attachment = PipelineColorBlendAttachmentState::builder()
        .color_write_mask(ColorComponentFlags::all())
        .blend_enable(false)
        .src_color_blend_factor(BlendFactor::ONE)
        .dst_color_blend_factor(BlendFactor::ZERO)
        .color_blend_op(BlendOp::ADD)
        .src_color_blend_factor(BlendFactor::ONE)
        .dst_alpha_blend_factor(BlendFactor::ZERO)
        .alpha_blend_op(BlendOp::ADD);
    let color_blend_attachments = [color_blend_attachment.build()];

    let color_blend_info = PipelineColorBlendStateCreateInfo::builder()
        .logic_op_enable(false)
        .logic_op(LogicOp::COPY)
        .attachments(&color_blend_attachments)
        .blend_constants([0.0,0.0,0.0,0.0]);

    // dynamic state
    let dynamic_states = [DynamicState::VIEWPORT,DynamicState::SCISSOR];
    #[allow(unused)]
    let dynamic_state_info = PipelineDynamicStateCreateInfo::builder()
        .dynamic_states(&dynamic_states);

    // pipeline layout
    let pipeline_layout_info = PipelineLayoutCreateInfo::builder();

    let pipeline_layout = unsafe {
        device.create_pipeline_layout(&pipeline_layout_info,Option::None)
    }.unwrap();

    // graphics pipeline
    let pipeline_info = GraphicsPipelineCreateInfo::builder()
        .stages(&shader_stages)
        .vertex_input_state(&vertex_input_info)
        .input_assembly_state(&input_assembly_info)
        .viewport_state(&viewport_info)
        .rasterization_state(&rasterization_info)
        .multisample_state(&multisample_info)
        .color_blend_state(&color_blend_info)
        // .dynamic_state(&dynamic_state_info)
        .layout(pipeline_layout)
        .render_pass(render_pass)
        .subpass(0);
    let pipeline_infos = [pipeline_info.build()];
    let pipelines = unsafe {
        device.create_graphics_pipelines(PipelineCache::null(),&pipeline_infos,Option::None)
    }.unwrap();
    let pipeline = pipelines.into_iter().next().unwrap();

    // must be deleted after pipeline was created
    unsafe { device.destroy_shader_module(vert_shader,Option::None) };
    unsafe { device.destroy_shader_module(frag_shader,Option::None) };

    // framebuffer
    let framebuffers = present_image_views
        .iter()
        .map(|view|{
            let attachments = [*view];
            let framebuffer_info = FramebufferCreateInfo::builder()
                .render_pass(render_pass)
                .attachments(&attachments)
                .width(extent.width)
                .height(extent.height)
                .layers(1);

            unsafe {
                device.create_framebuffer(&framebuffer_info,Option::None)
            }.unwrap()
        })
        .collect::<Vec<_>>();

    // command pool
    let command_pool_info = CommandPoolCreateInfo::builder()
        .queue_family_index(device_queue_family_index as u32);
    let command_pool = unsafe {
        device.create_command_pool(&command_pool_info,Option::None)
    }.unwrap();

    // command buffers
    let command_buffer_info = CommandBufferAllocateInfo::builder()
        .command_pool(command_pool)
        .level(CommandBufferLevel::PRIMARY)
        .command_buffer_count(framebuffers.len() as u32);
    let command_buffers = unsafe {
        device.allocate_command_buffers(&command_buffer_info)
    }.unwrap();

    // buffers
    let vertex_buffers = [vertex_buffer];
    let offsets = [0];

    // record buffer
    for (command_buffer,framebuffer) in command_buffers.iter().zip(framebuffers.iter()) {
        let begin_info = CommandBufferBeginInfo::default();
        unsafe {
            device.begin_command_buffer(*command_buffer, &begin_info)
        }.unwrap();

        // start a render pass
        let clear_color = ClearValue{
            color : ClearColorValue{
                float32 : [0.0,0.0,0.0,1.0]
            }
        };
        let clear_colors = [clear_color];
        let render_pass_begin = RenderPassBeginInfo::builder()
            .render_pass(render_pass)
            .framebuffer(*framebuffer)
            .render_area(Rect2D{
                offset : Offset2D{
                    x: 0, y: 0
                },
                extent
            }).clear_values(&clear_colors);

        unsafe {
            device.cmd_begin_render_pass(*command_buffer,&render_pass_begin,SubpassContents::INLINE);
            device.cmd_bind_pipeline(*command_buffer,PipelineBindPoint::GRAPHICS,pipeline);
            device.cmd_bind_vertex_buffers(*command_buffer,0,&vertex_buffers,&offsets);
            device.cmd_draw(*command_buffer,3,1,0,0);
            device.cmd_end_render_pass(*command_buffer)
        }
        unsafe {
            device.end_command_buffer(*command_buffer)
        }.unwrap();
    }

    // Semaphroes
    let semaphroe_info = SemaphoreCreateInfo::default();

    let image_available_semaphore = unsafe {
        device.create_semaphore(&semaphroe_info,Option::None)
    }.unwrap();
    let render_finish_semaphore = unsafe {
        device.create_semaphore(&semaphroe_info,Option::None)
    }.unwrap();

    // main loop
    struct DrawFrame{
        device : Arc<Device>,
        queue : Queue,
        swapchain : SwapchainKHR,
        swapchain_manager : Arc<Swapchain>,
        image_available_semaphore : Semaphore,
        render_finish_semaphore : Semaphore,
        command_buffers : Arc<Vec<CommandBuffer>>
    }
    impl<'a> System<'a> for DrawFrame {
        type InitResource = ();
        type Resource = (&'a mut ElikarStates,&'a PollEvents);
        type Dependencies = End;
        type Error = Infallible;

        fn update(&'a mut self, (mut state,events) : (RefMut<'a,ElikarStates>,Ref<'a,PollEvents>)) -> Result<(),Self::Error> {
            if events.quit.is_some() {
                state.quit();
                return Ok(())
            }
            let (image_index,_) = unsafe {
                self.swapchain_manager.acquire_next_image(
                    self.swapchain,
                    u64::MAX,
                    self.image_available_semaphore,
                    Fence::null()
                )
            }.unwrap();

            let wait_semaphroes = [self.image_available_semaphore];
            let wait_stages = [PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
            let command_buffer = [*self.command_buffers.get(image_index as usize).unwrap()];
            let signal_semaphroes = [self.render_finish_semaphore];

            let submit_info = SubmitInfo::builder()
                .wait_semaphores(&wait_semaphroes)
                .wait_dst_stage_mask(&wait_stages)
                .command_buffers(&command_buffer)
                .signal_semaphores(&signal_semaphroes);
            let submit_infos = [submit_info.build()];

            unsafe {
                self.device.queue_submit(self.queue,&submit_infos,Fence::null())
            }.unwrap();

            let swapchains = [self.swapchain];
            let image_indices = [image_index];

            let present_info = PresentInfoKHR::builder()
                .wait_semaphores(&signal_semaphroes)
                .swapchains(&swapchains)
                .image_indices(&image_indices);

            unsafe {
                self.swapchain_manager.queue_present(self.queue,&present_info).unwrap();
                self.device.queue_wait_idle(self.queue).unwrap();
            }

            println!("fps:{}Hz",state.fps());
            Ok(())
        }
    }
    let device = Arc::new(device);
    let swapchain_manager = Arc::new(swapchain_manager);
    let command_buffers = Arc::new(command_buffers);
    game.current_stage_mut()
        .add_system(DrawFrame{
            device : device.clone(),
            queue,
            swapchain,
            swapchain_manager : swapchain_manager.clone(),
            image_available_semaphore,
            render_finish_semaphore,
            command_buffers  : command_buffers.clone()
        });

    game.run();

    // let device = device;
    // let swapchain_manager = *swapchain_manager;
    // let command_buffers = *command_buffers;

    unsafe { device.device_wait_idle() }.unwrap();

    unsafe { device.destroy_semaphore(image_available_semaphore,Option::None) };
    unsafe { device.destroy_semaphore(render_finish_semaphore,Option::None) };
    unsafe { device.destroy_command_pool(command_pool,Option::None) };
    unsafe { device.free_memory(memory,Option::None) };
    unsafe { device.destroy_buffer(vertex_buffer,Option::None) };
    for framebuffer in framebuffers {
        unsafe { device.destroy_framebuffer(framebuffer,Option::None) };
    }
    unsafe { device.destroy_pipeline(pipeline,Option::None) };
    unsafe { device.destroy_pipeline_layout(pipeline_layout,Option::None) };
    unsafe { device.destroy_render_pass(render_pass,Option::None) };
    for image_view in present_image_views {
        unsafe { device.destroy_image_view(image_view,Option::None) };
    }
    unsafe { swapchain_manager.destroy_swapchain(swapchain,Option::None) };
    unsafe { surface_manager.destroy_surface(surface,Option::None) };
    unsafe { debug_utils.destroy_debug_utils_messenger(debug_utils_messenger,Option::None) };
    unsafe { device.destroy_device(Option::None) };
    unsafe { instance.destroy_instance(Option::None) };
}
