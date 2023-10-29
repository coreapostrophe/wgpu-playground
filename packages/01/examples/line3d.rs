use bytemuck::cast_slice;
use cgmath::{Point3, Vector3};
use commonlib::{
    renderer::RendererBuilder,
    transform::{create_projection, create_transforms, create_view_projection},
    vertices::Vertex3D,
};
use wgpu::{
    util::DeviceExt, vertex_attr_array, BindGroupEntry, BindGroupLayoutEntry, BufferAddress,
    BufferUsages, ShaderStages, VertexAttribute, VertexBufferLayout,
};
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

fn create_vertices() -> [Vertex3D; 300] {
    let mut vertices = [Vertex3D {
        position: [0.0, 0.0, 0.0],
    }; 300];
    for i in 0..300 {
        let t = 0.1 * (i as f32) / 30.0;
        let x = (-t).exp() * (30.0 * t).sin();
        let z = (-t).exp() * (30.0 * t).cos();
        let y = 2.0 * t - 1.0;
        vertices[i] = Vertex3D {
            position: [x, y, z],
        };
    }
    vertices
}

const VERTEX_ATTRIBUTE: [VertexAttribute; 1] = vertex_attr_array![0=>Float32x3];

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
        .with_title("Line 3D")
        .build(&event_loop)
        .expect("to create window");

    let window_size: PhysicalSize<u32> = window.inner_size();

    let mut renderer = RendererBuilder::new(window)
        .create_instance()
        .create_surface()
        .get_adapter()
        .get_device(Some("Device"))
        .create_surface_configuration()
        .set_primitive_state(
            wgpu::PrimitiveTopology::LineStrip,
            Some(wgpu::IndexFormat::Uint32),
        )
        .create_shader_module(Some("Shader Module"), include_str!("line3d.wgsl"))
        .add_vertex_buffer_layout(VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex3D>() as BufferAddress,
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
        .create_pipeline_layout(Some("Pipeline Layout"))
        .create_render_pipeline(Some("Render Pipeline"))
        .build();

    let camera_position: Point3<f32> = (1.5, 1.0, 3.0).into();
    let look_direction: Point3<f32> = (0.0, 0.0, 0.0).into();
    let up_direction: Vector3<f32> = Vector3::unit_y();

    let model_matrix = create_transforms([0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [1.0, 1.0, 1.0]);
    let (view_matrix, _projection_matrix, view_projection_matrix) = create_view_projection(
        camera_position,
        look_direction,
        up_direction,
        window_size.width as f32 / window_size.height as f32,
        is_perspective,
    );
    let mvp_matrix = view_projection_matrix * model_matrix;

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
                label: Some("Uniform Bind"),
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

                let surface_configuration = renderer.surface_configuration().unwrap();
                let surface = renderer.surface().unwrap();
                let device = renderer.device().unwrap();
                surface.configure(device, surface_configuration);

                let new_projection_matrix =
                    create_projection(size.width as f32 / size.height as f32, is_perspective);
                let mvp_mat = new_projection_matrix * view_matrix * model_matrix;
                let mvp_ref: &[f32; 16] = mvp_mat.as_ref();
                renderer.queue().unwrap().write_buffer(
                    &uniform_buffer,
                    0,
                    bytemuck::cast_slice(mvp_ref),
                );
            }
            Event::RedrawRequested(_) => {
                let surface = renderer.surface().unwrap();
                let device = renderer.device().unwrap();
                let queue = renderer.queue().unwrap();
                let render_pipeline = renderer.render_pipeline().unwrap();
                let surface_texture = surface.get_current_texture().unwrap();
                let texture_view = surface_texture.texture.create_view(&Default::default());

                let mut command_encoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

                {
                    let mut render_pass =
                        command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                            label: None,
                            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Clear(wgpu::Color {
                                        r: 0.5,
                                        g: 0.5,
                                        b: 0.5,
                                        a: 1.0,
                                    }),
                                    store: true,
                                },
                                resolve_target: None,
                                view: &texture_view,
                            })],
                            depth_stencil_attachment: None,
                        });
                    render_pass.set_pipeline(&render_pipeline);
                    render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                    render_pass.set_bind_group(0, &uniform_bind_group, &[]);
                    render_pass.draw(0..300, 0..1)
                }
                queue.submit(Some(command_encoder.finish()));
                surface_texture.present();
            }
            _ => (),
        }
    })
}
