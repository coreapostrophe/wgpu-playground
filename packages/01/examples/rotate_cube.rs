use bytemuck::cast_slice;
use commonlib::{
    projection::Projection,
    renderer::RendererBuilder,
    vertices::{vertex_data, Vertex4DColored},
};
use wgpu::{
    util::DeviceExt, vertex_attr_array, BindGroupEntry, BindGroupLayoutEntry, BufferAddress,
    BufferUsages, Extent3d, ShaderStages, TextureDescriptor, TextureUsages, VertexAttribute,
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

fn create_vertex(position: [i8; 3], color: [i8; 3]) -> Vertex4DColored {
    Vertex4DColored {
        position: [
            position[0] as f32,
            position[1] as f32,
            position[2] as f32,
            1.0,
        ],
        color: [color[0] as f32, color[1] as f32, color[2] as f32, 1.0],
    }
}

fn create_vertices() -> Vec<Vertex4DColored> {
    let (pos, col, _uv, _normal) = vertex_data::cube_data();
    let mut data: Vec<Vertex4DColored> = Vec::with_capacity(pos.len());
    for i in 0..pos.len() {
        data.push(create_vertex(pos[i], col[i]));
    }
    data
}

const ANIMATION_SPEED: f32 = 1.0;
const VERTEX_ATTRIBUTE: [VertexAttribute; 2] = vertex_attr_array![0=>Float32x4,1=>Float32x4];

fn main() {
    env_logger::init();

    let mut is_perspective = true;

    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        is_perspective = match args[1].as_str() {
            "ortho" => false,
            _ => true,
        }
    }

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Rotate Cube")
        .build(&event_loop)
        .expect("to create window");

    let mut renderer = RendererBuilder::new(window)
        .create_instance()
        .create_surface()
        .get_adapter()
        .get_device(Some("Device"))
        .create_surface_configuration()
        .create_shader_module(Some("Shader"), include_str!("rotate_cube.wgsl"))
        .add_vertex_buffer_layout(wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex4DColored>() as BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &VERTEX_ATTRIBUTE,
        })
        .add_bind_group_layout(
            Some("Uniform Bind Group Layout"),
            &[BindGroupLayoutEntry {
                binding: 0,
                count: None,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                visibility: ShaderStages::VERTEX,
            }],
        )
        .set_primitive_state(wgpu::PrimitiveTopology::TriangleList, None)
        .set_depth_stencil_state(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth24Plus,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::LessEqual,
            stencil: Default::default(),
            bias: Default::default(),
        })
        .create_pipeline_layout(Some("Pipeline Layout"))
        .create_render_pipeline(Some("Render Pipeline"))
        .build();

    let mut projection = Projection::new(
        renderer.surface_configuration().unwrap().width as f32,
        renderer.surface_configuration().unwrap().height as f32,
    );
    projection.set_is_perspective(is_perspective);

    let uniform_buffer =
        renderer
            .device()
            .unwrap()
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Uniform Buffer"),
                contents: cast_slice(projection.mvp_matrix_slice()),
                usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            });

    let uniform_bind_group =
        renderer
            .device()
            .unwrap()
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Uniform Bind Group"),
                layout: renderer.bind_group_layouts().get(0).unwrap(),
                entries: &[BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                }],
            });

    let vertex_buffer =
        renderer
            .device()
            .unwrap()
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: cast_slice(&create_vertices()),
                usage: BufferUsages::VERTEX,
            });

    let render_start_time = std::time::Instant::now();

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                control_flow.set_exit();
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                {
                    let surface_configuration = renderer.mut_surface_configuration().unwrap();
                    surface_configuration.width = size.width;
                    surface_configuration.height = size.height;
                }

                let surface_configuration = renderer.surface_configuration().unwrap();
                let surface = renderer.surface().unwrap();
                let device = renderer.device().unwrap();
                surface.configure(device, surface_configuration);

                projection.set_aspect_ratio(size.width as f32 / size.height as f32);

                renderer.queue().unwrap().write_buffer(
                    &uniform_buffer,
                    0,
                    bytemuck::cast_slice(projection.mvp_matrix_slice()),
                );
            }
            Event::RedrawRequested(_) => {
                let current_time = std::time::Instant::now();
                let current_duration = current_time - render_start_time;
                let animated_duration = ANIMATION_SPEED * current_duration.as_secs_f32();

                projection.set_model_rotation([
                    animated_duration.sin(),
                    animated_duration.cos(),
                    0.0,
                ]);

                renderer.queue().unwrap().write_buffer(
                    &uniform_buffer,
                    0,
                    bytemuck::cast_slice(projection.mvp_matrix_slice()),
                );

                let surface = renderer.surface().unwrap();
                let device = renderer.device().unwrap();
                let queue = renderer.queue().unwrap();
                let render_pipeline = renderer.render_pipeline().unwrap();
                let surface_texture = surface.get_current_texture().unwrap();
                let surface_configuration = renderer.surface_configuration().unwrap();

                let texture_view = surface_texture.texture.create_view(&Default::default());
                let depth_texture = renderer
                    .device()
                    .unwrap()
                    .create_texture(&TextureDescriptor {
                        label: Some("Texture"),
                        size: Extent3d {
                            depth_or_array_layers: 1,
                            width: surface_configuration.width,
                            height: surface_configuration.height,
                        },
                        mip_level_count: 1,
                        sample_count: 1,
                        dimension: wgpu::TextureDimension::D2,
                        format: wgpu::TextureFormat::Depth24Plus,
                        usage: TextureUsages::RENDER_ATTACHMENT,
                        view_formats: &[],
                    });
                let depth_view = depth_texture.create_view(&Default::default());

                let mut command_encoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

                {
                    let mut render_pass =
                        command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                            label: None,
                            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                view: &texture_view,
                                resolve_target: None,
                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Clear(wgpu::Color {
                                        r: 0.5,
                                        g: 0.5,
                                        b: 0.5,
                                        a: 1.0,
                                    }),
                                    store: true,
                                },
                            })],
                            depth_stencil_attachment: Some(
                                wgpu::RenderPassDepthStencilAttachment {
                                    view: &depth_view,
                                    depth_ops: Some(wgpu::Operations {
                                        load: wgpu::LoadOp::Clear(1.0),
                                        store: false,
                                    }),
                                    stencil_ops: None,
                                },
                            ),
                        });
                    render_pass.set_pipeline(&render_pipeline);
                    render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                    render_pass.set_bind_group(0, &uniform_bind_group, &[]);
                    render_pass.draw(0..36, 0..1)
                }
                queue.submit(Some(command_encoder.finish()));
                surface_texture.present();
            }
            Event::MainEventsCleared => {
                renderer.window().unwrap().request_redraw();
            }
            _ => (),
        }
    })
}
