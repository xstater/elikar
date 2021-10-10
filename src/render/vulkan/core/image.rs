use crate::render::vulkan::core::{
    AshRaw, Buffer, CommandBuffers, Core, DeviceMemory, ImageView, ImageViewBuilder,
};
use ash::vk;
use std::sync::Arc;

pub struct Image {
    pub(in crate::render) core: Arc<Core>,
    pub(in crate::render) image: vk::Image,
    pub(in crate::render) image_type: vk::ImageType,
    pub(in crate::render) extent: vk::Extent3D,
    pub(in crate::render) format: vk::Format,
    pub(in crate::render) layout: vk::ImageLayout,
    pub(in crate::render) memory: Option<DeviceMemory>,
}

impl Image {
    pub fn builder() -> ImageBuilder {
        ImageBuilder {
            info: vk::ImageCreateInfo {
                image_type: vk::ImageType::TYPE_2D,
                format: vk::Format::R8G8B8A8_UNORM,
                samples: vk::SampleCountFlags::TYPE_1,
                tiling: vk::ImageTiling::OPTIMAL,
                usage: vk::ImageUsageFlags::SAMPLED,
                sharing_mode: vk::SharingMode::EXCLUSIVE,
                initial_layout: vk::ImageLayout::UNDEFINED,
                mip_levels: 1,
                array_layers: 1,
                ..Default::default()
            },
            queue_family_indices: vec![],
        }
    }

    pub fn image_type(&self) -> vk::ImageType {
        self.image_type
    }

    pub fn extent(&self) -> vk::Extent3D {
        self.extent
    }

    pub fn format(&self) -> vk::Format {
        self.format
    }

    pub fn layout(&self) -> vk::ImageLayout {
        self.layout
    }

    pub fn copy_from_buffer(&mut self, buffer: &Buffer) -> Result<(), vk::Result> {
        debug_assert!(
            self.has_memory() || buffer.has_memory(),
            "Cannot copy data from buffer! Buffer has not bound any memory."
        );
        let command_buffers = CommandBuffers::allocate(self.core.clone(), 1)?;
        let command_buffer = command_buffers.command_buffers[0];
        let begin_info = vk::CommandBufferBeginInfo::builder()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);

        unsafe {
            self.core
                .device
                .begin_command_buffer(command_buffer, &begin_info)?;

            let copy = [vk::BufferImageCopy {
                buffer_offset: 0,
                buffer_row_length: 0,
                buffer_image_height: 0,
                image_subresource: vk::ImageSubresourceLayers {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    mip_level: 0,
                    base_array_layer: 0,
                    layer_count: 1,
                },
                image_offset: vk::Offset3D { x: 0, y: 0, z: 0 },
                image_extent: self.extent,
            }];
            self.core.device.cmd_copy_buffer_to_image(
                command_buffer,
                buffer.buffer,
                self.image,
                self.layout,
                &copy,
            );

            self.core.device.end_command_buffer(command_buffer)?;
        }

        // submit
        let submit_info = [vk::SubmitInfo::builder()
            .command_buffers(command_buffers.raw())
            .build()];
        unsafe {
            self.core.device.queue_submit(
                self.core.transfer_queue,
                &submit_info,
                vk::Fence::null(),
            )?;
            self.core.device.queue_wait_idle(self.core.transfer_queue)?;
        }
        Ok(())
    }

    pub fn convert_layout_to(&mut self, layout: vk::ImageLayout) -> Result<(), vk::Result> {
        let command_buffers = CommandBuffers::allocate(self.core.clone(), 1)?;
        let command_buffer = command_buffers.command_buffers[0];
        let begin_info = vk::CommandBufferBeginInfo::builder()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);

        unsafe {
            self.core
                .device
                .begin_command_buffer(command_buffer, &begin_info)?;
            let mut barrier = vk::ImageMemoryBarrier {
                old_layout: self.layout,
                new_layout: layout,
                src_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
                dst_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
                image: self.image,
                subresource_range: vk::ImageSubresourceRange {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    base_mip_level: 0,
                    level_count: 1,
                    base_array_layer: 0,
                    layer_count: 1,
                },
                ..Default::default()
            };

            #[allow(unused)]
            let mut source = vk::PipelineStageFlags::empty();
            #[allow(unused)]
            let mut destination = vk::PipelineStageFlags::empty();

            if self.layout == vk::ImageLayout::UNDEFINED
                && layout == vk::ImageLayout::TRANSFER_DST_OPTIMAL
            {
                barrier.src_access_mask = vk::AccessFlags::empty();
                barrier.dst_access_mask = vk::AccessFlags::TRANSFER_WRITE;

                source = vk::PipelineStageFlags::TOP_OF_PIPE;
                destination = vk::PipelineStageFlags::TRANSFER;
            } else if self.layout == vk::ImageLayout::TRANSFER_DST_OPTIMAL
                && layout == vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL
            {
                barrier.src_access_mask = vk::AccessFlags::TRANSFER_WRITE;
                barrier.dst_access_mask = vk::AccessFlags::SHADER_READ;

                source = vk::PipelineStageFlags::TRANSFER;
                destination = vk::PipelineStageFlags::FRAGMENT_SHADER;
            } else {
                panic!("Convert image layout failed! Convert is not supported");
            }

            let barrier = [barrier];

            self.core.device.cmd_pipeline_barrier(
                command_buffer,
                source,
                destination,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                &barrier,
            );

            self.core.device.end_command_buffer(command_buffer)?;
        }

        // submit
        let submit_info = [vk::SubmitInfo::builder()
            .command_buffers(command_buffers.raw())
            .build()];
        unsafe {
            self.core.device.queue_submit(
                self.core.transfer_queue,
                &submit_info,
                vk::Fence::null(),
            )?;
            self.core.device.queue_wait_idle(self.core.transfer_queue)?;
        }

        self.layout = layout;
        Ok(())
    }

    pub fn create_view(&self) -> ImageViewBuilder {
        ImageView::builder(self)
    }

    pub fn memory_requirements(&self) -> vk::MemoryRequirements {
        unsafe { self.core.device.get_image_memory_requirements(self.image) }
    }

    pub fn bind_memory(&mut self, memory: DeviceMemory) -> Result<(), vk::Result> {
        unsafe {
            self.core
                .device
                .bind_image_memory(self.image, memory.device_memory, 0)?
        };
        self.memory = Some(memory);
        Ok(())
    }

    pub fn has_memory(&self) -> bool {
        self.memory.is_some()
    }

    pub fn memory(&self) -> Option<&DeviceMemory> {
        self.memory.as_ref()
    }

    pub fn memory_mut(&mut self) -> Option<&mut DeviceMemory> {
        self.memory.as_mut()
    }
}

impl AshRaw for Image {
    type Raw = vk::Image;

    fn raw(&self) -> &Self::Raw {
        &self.image
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe { self.core.device.destroy_image(self.image, Option::None) }
    }
}

pub struct ImageBuilder {
    info: vk::ImageCreateInfo,
    queue_family_indices: Vec<u32>,
}

impl ImageBuilder {
    pub fn image_type(mut self, image_type: vk::ImageType) -> Self {
        self.info.image_type = image_type;
        self
    }

    pub fn format(mut self, format: vk::Format) -> Self {
        self.info.format = format;
        self
    }

    pub fn extent(mut self, extent: vk::Extent3D) -> Self {
        self.info.extent = extent;
        self
    }

    pub fn mip_levels(mut self, level: u32) -> Self {
        self.info.mip_levels = level;
        self
    }

    pub fn array_layers(mut self, layers: u32) -> Self {
        self.info.array_layers = layers;
        self
    }

    pub fn samples(mut self, samples: vk::SampleCountFlags) -> Self {
        self.info.samples = samples;
        self
    }

    pub fn tiling(mut self, tiling: vk::ImageTiling) -> Self {
        self.info.tiling = tiling;
        self
    }

    pub fn usage(mut self, usage: vk::ImageUsageFlags) -> Self {
        self.info.usage = usage;
        self
    }

    pub fn sharing_mode(mut self, sharing_mode: vk::SharingMode) -> Self {
        self.info.sharing_mode = sharing_mode;
        self
    }

    pub fn queue_family_index(mut self, index: u32) -> Self {
        self.queue_family_indices.push(index);
        self
    }

    pub fn initial_layout(mut self, layout: vk::ImageLayout) -> Self {
        self.info.initial_layout = layout;
        self
    }

    pub(in crate::render) fn build(self, core: Arc<Core>) -> Result<Image, vk::Result> {
        let mut info = self.info;
        info.queue_family_index_count = self.queue_family_indices.len() as _;
        info.p_queue_family_indices = self.queue_family_indices.as_ptr();

        let image = unsafe { core.device.create_image(&info, Option::None) }?;
        Ok(Image {
            core,
            image,
            image_type: self.info.image_type,
            extent: self.info.extent,
            format: self.info.format,
            layout: self.info.initial_layout,
            memory: Option::None,
        })
    }
}
