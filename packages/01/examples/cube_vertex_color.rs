use std::iter;

use bytemuck::cast_slice;
use cgmath::Point3;
use commonlib::{
    renderer::RendererBuilder,
    transform::{create_projection, create_transforms, create_view_projection},
    vertices::{vertex_data::cube_data_index, Vertex4DColored},
};
use wgpu::{
    util::DeviceExt, vertex_attr_array, BindGroupEntry, BindGroupLayoutEntry, BufferAddress,
    BufferUsages, CommandEncoderDescriptor, Extent3d, ShaderStages, TextureDescriptor,
    TextureDimension, TextureFormat, TextureUsages, VertexAttribute,
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

fn create_vertex(position: &[i8; 3], color: &[i8; 3]) -> Vertex4DColored {
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

fn create_vertices() -> (Vec<Vertex4DColored>, Vec<u16>) {
    let (positions, colors, indices) = cube_data_index();
    let mut vertices: Vec<Vertex4DColored> = Vec::new();
    for i in 0..positions.len() {
        vertices.push(create_vertex(&positions[i], &colors[i]));
    }
    (vertices, indices)
}

const VERTEX_ATTRIBUTE: [VertexAttribute; 2] = vertex_attr_array![0=>Float32x4, 1=>Float32x4];
const IS_PERSPECTIVE: bool = true;

fn main() {
    env_logger::init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Cube Vertex Color")
        .build(&event_loop)
        .expect("to create window");

    let mut renderer = RendererBuilder::new(window)
        .create_instance()
        .create_surface()
        .get_adapter()
        .get_device(Some("Device"))
        .create_surface_configuration()
        .create_shader_module(
            Some("Shader Module"),
            include_str!("cube_vertex_color.wgsl"),
        )
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

    let (vertices, indices) = create_vertices();

    let camera_position: Point3<f32> = (3.0, 1.5, 3.0).into();
    let look_direction: Point3<f32> = (0.0, 0.0, 0.0).into();
    let up_direction = cgmath::Vector3::<f32>::unit_y();

    let model_matrix = create_transforms([0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [1.0, 1.0, 1.0]);
    let (view_matrix, _projection_matrix, view_projection_matrix) = create_view_projection(
        camera_position,
        look_direction,
        up_direction,
        renderer.surface_configuration().unwrap().width as f32
            / renderer.surface_configuration().unwrap().height as f32,
        IS_PERSPECTIVE,
    );
    let mvp_matrix = view_projection_matrix * model_matrix;

    let vertex_buffer =
        renderer
            .device()
            .unwrap()
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: cast_slice(&vertices),
                usage: BufferUsages::VERTEX,
            });

    let index_buffer =
        renderer
            .device()
            .unwrap()
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: cast_slice(&indices),
                usage: BufferUsages::INDEX,
            });

    let uniform_buffer =
        renderer
            .device()
            .unwrap()
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Uniform Buffer"),
                contents: cast_slice(mvp_matrix.as_ref() as &[f32; 16]),
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

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => control_flow.set_exit(),
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                {
                    let surface_configuration = renderer.mut_surface_configuration().unwrap();
                    surface_configuration.width = size.width;
                    surface_configuration.height = size.height;
                }

                let surface = renderer.surface().unwrap();
                let device = renderer.device().unwrap();
                let surface_configuration = renderer.surface_configuration().unwrap();
                surface.configure(device, surface_configuration);

                let new_projection_matrix =
                    create_projection(size.width as f32 / size.height as f32, IS_PERSPECTIVE);
                let mvp_matrix = new_projection_matrix * view_matrix * model_matrix;

                let queue = renderer.queue().unwrap();
                queue.write_buffer(
                    &uniform_buffer,
                    0,
                    cast_slice(mvp_matrix.as_ref() as &[f32; 16]),
                );
            }
            Event::RedrawRequested(_) => {
                let device = renderer.device().unwrap();
                let surface = renderer.surface().unwrap();
                let render_pipeline = renderer.render_pipeline().unwrap();
                let surface_configuration = renderer.surface_configuration().unwrap();

                let surface_texture = surface.get_current_texture().unwrap();
                let texture_view = surface_texture.texture.create_view(&Default::default());
                let depth_texture = device.create_texture(&TextureDescriptor {
                    label: Some("Depth Texture"),
                    size: Extent3d {
                        depth_or_array_layers: 1,
                        width: surface_configuration.width,
                        height: surface_configuration.height,
                    },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: TextureDimension::D2,
                    format: TextureFormat::Depth24Plus,
                    usage: TextureUsages::RENDER_ATTACHMENT,
                    view_formats: &[],
                });
                let depth_view = depth_texture.create_view(&Default::default());

                let mut command_encoder =
                    device.create_command_encoder(&CommandEncoderDescriptor {
                        label: Some("Command Encoder"),
                    });
                {
                    let mut render_pass =
                        command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                            label: Some("Render Pass"),
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
                    render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                    render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                    render_pass.set_bind_group(0, &uniform_bind_group, &[]);
                    render_pass.draw_indexed(0..indices.len() as u32, 0, 0..1);
                }

                let queue = renderer.queue().unwrap();
                queue.submit(iter::once(command_encoder.finish()));
                surface_texture.present();
            }
            _ => (),
        }
    })
}
