mod rebuild;

use crate::render::renderer::{Renderer, Vertex};
use xecs::{System, World, EntityId};
use xecs::resource::Resource;
use xecs::system::End;
use std::cell::{Ref, RefMut};
use ash::vk;
use crate::ElikarStates;
use crate::render::transform::Transform;
use crate::render::sprite::Sprite;

impl<'a> System<'a> for Renderer{
    type Resource = (&'a World,&'a mut ElikarStates);
    type Dependencies = End;

    fn update(&'a mut self, (world,mut states) : (Ref<'a,World>,RefMut<'a,ElikarStates>)) {
        // 2d sprite
        self.sprite_vertices.clear();
        if self.sprite_vertex_buffer != vk::Buffer::null() {
            unsafe {
                self.vulkan.device.destroy_buffer(self.sprite_vertex_buffer, Option::None)
            }
        }
        if self.sprite_memory != vk::DeviceMemory::null() {
            unsafe {
                self.vulkan.device.free_memory(self.sprite_memory,Option::None)
            }
        }
        for (transform,sprite) in world.query::<(&Transform,&Sprite)>() {
            let transform : &Transform = transform;
            let sprite : &Sprite = sprite;

            self.sprite_vertices.push(Vertex{ pos: [0.0,0.0,0.0] });
            self.sprite_vertices.push(Vertex{ pos: [1.0,0.0,0.0] });
            self.sprite_vertices.push(Vertex{ pos: [0.0,1.0,0.0] });
        }
        let vertex_buffer_info = vk::BufferCreateInfo::builder()
            .size((3 * std::mem::size_of::<Vertex>()) as _)
            .usage(vk::BufferUsageFlags::VERTEX_BUFFER)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);
        let res = unsafe {
            self.vulkan.device.create_buffer(&vertex_buffer_info,Option::None)
        };
        if let Err(error) = res {
            states.error(error);
            return;
        }
        self.sprite_vertex_buffer = res.unwrap();

        let buffer_reqs = unsafe {
            self.vulkan.device.get_buffer_memory_requirements(self.sprite_vertex_buffer)
        };

        // find memory type
        let memory_type_index = find_memory_type(
            &self.vulkan.memory_properties.memory_types,
            &buffer_reqs,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT
        );
        // if let None = memory_type_index {
        //     return;
        // }
        let memory_type_index = memory_type_index.unwrap();

        let buffer_allocate_info = vk::MemoryAllocateInfo::builder()
            .memory_type_index(memory_type_index as _)
            .allocation_size(buffer_reqs.size);
        let memory = unsafe {
            self.vulkan.device.allocate_memory(&buffer_allocate_info,Option::None)
        };
        if let Err(error) = memory {
            states.error(error);
            return;
        }
        self.sprite_memory = memory.unwrap();

        let res = unsafe {
            self.vulkan.device.bind_buffer_memory(
                self.sprite_vertex_buffer,
                self.sprite_memory,
                0
            )
        };
        if let Err(error) = res {
            states.error(error);
            return;
        }

        let buffer_ptr = unsafe {
            self.vulkan.device.map_memory(
                self.sprite_memory,
                0,
                buffer_reqs.size,
                vk::MemoryMapFlags::empty())
        }.unwrap();
        let mut buffer_slice = unsafe {
            ash::util::Align::new(
                buffer_ptr,
                std::mem::align_of::<Vertex>() as _,
                buffer_reqs.size
            )
        };
        buffer_slice.copy_from_slice(&self.sprite_vertices);
        unsafe {
            self.vulkan.device.unmap_memory(self.sprite_memory);
        }



        // draw
        let res = unsafe {
            self.vulkan.swapchain_manager.acquire_next_image(
                self.vulkan.swapchain,
                u64::MAX,
                self.image_available_semaphore,
                vk::Fence::null()
            )
        };
        if let Err(error) = res {
            states.error(error);
            return;
        }
        let (image_index,flag) = res.unwrap();
        let image_index = image_index as usize;

        self.need_update_commands[image_index] = true;

        //record buffer
        let command_buffer = self.command_buffers[image_index];
        if self.need_update_commands[image_index] {
            unsafe {
                self.vulkan.device.reset_command_buffer(command_buffer,vk::CommandBufferResetFlags::all())
            }.unwrap();

            let begin_info = vk::CommandBufferBeginInfo::default();
            let res = unsafe {
                self.vulkan.device.begin_command_buffer(command_buffer,&begin_info)
            };
            if let Err(error) = res {
                states.error(error);
                return;
            }

            // start render pass
            let clear_colors = [vk::ClearValue {
                color : vk::ClearColorValue {
                    float32 : [self.clear_color[0],self.clear_color[1],self.clear_color[2],1.0]
                }
            }];
            let render_pass_begin_info = vk::RenderPassBeginInfo::builder()
                .framebuffer(self.framebuffers[image_index])
                .render_pass(self.render_passes[0])
                .clear_values(&clear_colors)
                .render_area(vk::Rect2D {
                    offset: vk::Offset2D{ x: 0, y: 0 },
                    extent: self.vulkan.surface_extent
                });

            unsafe {
                self.vulkan.device.cmd_begin_render_pass(
                    command_buffer,
                    &render_pass_begin_info,
                    vk::SubpassContents::INLINE);
                self.vulkan.device.cmd_bind_pipeline(
                    command_buffer,
                    vk::PipelineBindPoint::GRAPHICS,
                    self.pipelines[0]);
                let vertex_buffers = [self.sprite_vertex_buffer];
                let offsets = [0];
                self.vulkan.device.cmd_bind_vertex_buffers(
                    command_buffer,
                    0,
                    &vertex_buffers,
                    &offsets);
                self.vulkan.device.cmd_draw(
                    command_buffer,
                    self.sprite_vertices.len() as _,
                    1,
                    0,
                    0);
                self.vulkan.device.cmd_end_render_pass(command_buffer);
            }

            let res = unsafe {
                self.vulkan.device.end_command_buffer(command_buffer)
            };
            if let Err(error) = res {
                states.error(error);
                return;
            }

            self.need_update_commands[image_index] = false;
        }

        // draw frame
        let wait_semaphores = [self.image_available_semaphore];
        let wait_stages = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let signal_semaphores = [self.render_finish_semaphore];
        let command_buffer = [command_buffer];
        let submit_info = vk::SubmitInfo::builder()
            .command_buffers(&command_buffer)
            .signal_semaphores(&signal_semaphores)
            .wait_semaphores(&wait_semaphores)
            .wait_dst_stage_mask(&wait_stages);
        let submit_infos = [submit_info.build()];

        let res = unsafe {
            self.vulkan.device.queue_submit(
                self.vulkan.graphics_queue,
                &submit_infos,
                vk::Fence::null())
        };
        if let Err(error) = res {
            states.error(error);
            return;
        }

        let swapchains = [self.vulkan.swapchain];
        let image_indices = [image_index as u32];

        let present_info = vk::PresentInfoKHR::builder()
            .wait_semaphores(&signal_semaphores)
            .swapchains(&swapchains)
            .image_indices(&image_indices);

        let res = unsafe {
            self.vulkan.swapchain_manager.queue_present(self.vulkan.graphics_queue,&present_info)
                .and_then(|_|{
                    self.vulkan.device.queue_wait_idle(self.vulkan.graphics_queue)
                })
        };
        if let Err(error) = res {
            states.error(error);
            return;
        }
    }
}

fn find_memory_type(memory_types : &[vk::MemoryType],reqs : &vk::MemoryRequirements,properties : vk::MemoryPropertyFlags) -> Option<usize> {
    for (index,memory_type) in memory_types.iter().enumerate() {
        if reqs.memory_type_bits & (1 << index) != 0 && memory_type.property_flags & properties == properties {
            return Some(index);
        }
    }
    None
}