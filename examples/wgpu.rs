use std::borrow::Cow;

use elikar::{Elikar, States, window::{Window, event::WindowEventType}};
use futures::{StreamExt, executor::block_on};
use wgpu::{Backends, Instance};
use xecs::{query::WithId, system::System};

const WIDTH:u32 = 1280;
const HEIGHT:u32 = 768;

fn main() {
    let mut game = Elikar::new().unwrap();
    let world = game.world();

    let window_id = game.window_builder()
        .title("wgpu")
        .size(WIDTH,HEIGHT)
        .build()
        .unwrap();

    let instance = Instance::new(Backends::PRIMARY);
    let surface = {
        let world = world.read().unwrap();
        let window = world.query::<&Window>().with_id()
            .find(|(id,_)|*id == window_id)
            .map(|(_,window)|window)
            .unwrap();
        unsafe { instance.create_surface(&window) }
    };

    // for adapter in instance.enumerate_adapters(Backends::PRIMARY) {
        // println!("{:?}",adapter);
        // println!("features:{:?}",adapter.features());
        // println!("limits:{:?}",adapter.limits());
    // }
    
    let adapter = block_on(instance.request_adapter(
            &wgpu::RequestAdapterOptions{
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            }))
    .unwrap();

    let (device, queue) = block_on(adapter.request_device(
            &wgpu::DeviceDescriptor{
                label: Some("device"),
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
            },None))
    .unwrap();

    let shader = device.create_shader_module(
        &wgpu::ShaderModuleDescriptor{
            label: Some("shader"),
            source: wgpu::ShaderSource::Wgsl(
                Cow::Borrowed(include_str!("..\\shaders\\wgpu\\shader.wgsl"))),
        });

    let bind_group_layout = device.create_bind_group_layout(
        &wgpu::BindGroupLayoutDescriptor{
            label: Some("bind_group_layout"),
            entries: &[],
        });

    let bind_group = device.create_bind_group(
        &wgpu::BindGroupDescriptor {
            label: Some("bind_group"),
            layout: &bind_group_layout,
            entries: &[],
        });

    let pipeline_layout = device.create_pipeline_layout(
        &wgpu::PipelineLayoutDescriptor{
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

    let render_pipeline = device.create_render_pipeline(
        &wgpu::RenderPipelineDescriptor{
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState{
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            primitive: wgpu::PrimitiveState{
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Front),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative:false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState{
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: Some(wgpu::FragmentState{
                module: &shader,
                entry_point: "fs_main",
                targets: &[
                    wgpu::ColorTargetState{
                        format: wgpu::TextureFormat::Bgra8UnormSrgb,
                        blend: None,
                        write_mask: wgpu::ColorWrites::ALL,
                    }
                ],
            }),
            multiview: None,
        }
    );

    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface.get_preferred_format(&adapter).unwrap(),
        width : WIDTH,
        height : HEIGHT,
        present_mode: wgpu::PresentMode::Immediate,
    };

    surface.configure(&device, &config);

    //store these in world
    {
        let mut world = world.write().unwrap();
        world.store_resource(device);
        world.store_resource(queue);
        world.store_resource(config);
        world.store_resource(surface);
        world.store_resource(render_pipeline);
        world.store_resource(bind_group);
    }

    let events = game.events();
    game.spawn_local(async move{
        let mut quit = events.on_quit();
        let world = quit.world();
        if let Some(_) = quit.next().await {
            let world = world.read().unwrap();
            let mut states = world.resource_mut::<States>().unwrap();
            states.quit()
        }
    });

    let events = game.events();
    game.spawn_local(async move{
        let mut window = events.on_window_events();
        let world = window.world();
        while let Some(event) = window.next().await {
            if let WindowEventType::SizeChanged(w,h) = event.event_type {
                let world = world.read().unwrap();
                let mut config = world.resource_mut::<wgpu::SurfaceConfiguration>().unwrap();
                let device = world.resource_ref::<wgpu::Device>().unwrap();
                let surface = world.resource_ref::<wgpu::Surface>().unwrap();
                config.width = w;
                config.height = h;
                surface.configure(&device, &config);
            }
        }
    });

    let events = game.events();
    game.spawn_local(async move {
        let mut render = events.on_render();
        let world = render.world();
        while let Some(_) = render.next().await {
            let world = world.read().unwrap();
            let frame = {
                let surface = world.resource_ref::<wgpu::Surface>().unwrap();
                surface.get_current_texture().unwrap()
            };
            let output = frame.texture
                .create_view(&wgpu::TextureViewDescriptor::default());
            let mut encoder = {
                let device = world.resource_ref::<wgpu::Device>().unwrap();
                device.create_command_encoder(&wgpu::CommandEncoderDescriptor{
                    label: Some("command_encoder"),
                })
            };

            let render_pipeline = world.resource_ref::<wgpu::RenderPipeline>().unwrap();
            let bind_group = world.resource_ref::<wgpu::BindGroup>().unwrap();
            // render pass
            {
                let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[
                        wgpu::RenderPassColorAttachment{
                            view: &output,
                            resolve_target: None,
                            ops: wgpu::Operations{
                                load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                                store: true,
                            },
                        }
                    ],
                    depth_stencil_attachment: None,
                });
                rpass.set_pipeline(&render_pipeline);
                rpass.set_bind_group(0, &bind_group,&[]);
                rpass.draw(0..3,0..1);
            }

            let queue = world.resource_ref::<wgpu::Queue>().unwrap();
            queue.submit([encoder.finish()]);
            frame.present();
        }
    });

    let events = game.events();
    game.spawn_local(async move{
        let mut enter_frame = events.on_enter_frame();
        let world = enter_frame.world();
        while let Some(index) = enter_frame.next().await {
            let world = world.read().unwrap();
            let states = world.resource_ref::<States>().unwrap();
            println!("frame {} : fps = {}Hz",index,states.fps())
        }
    });

    game.run();

}
