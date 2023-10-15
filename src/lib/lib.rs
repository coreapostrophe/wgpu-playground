use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

pub async fn run() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Wgpu Playground")
        .build(&event_loop)
        .unwrap();

    let size = window.inner_size();

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        dx12_shader_compiler: Default::default(),
    });

    let surface = unsafe { instance.create_surface(&window) }
        .expect("Instance to create surface from window");

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptionsBase {
            power_preference: Default::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        })
        .await
        .expect("Instance to find an appropriate adapter");

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: Default::default(),
                limits: Default::default(),
            },
            None,
        )
        .await
        .expect("Adapter to fetch a device");

    let surface_capabilities = surface.get_capabilities(&adapter);
    let format = surface_capabilities
        .formats
        .iter()
        .copied()
        .find(|format| format.is_srgb())
        .expect("Surface to have srgb texture format");

    let mut surface_configuration = wgpu::SurfaceConfiguration {
        format: format,
        width: size.width,
        height: size.height,
        alpha_mode: wgpu::CompositeAlphaMode::Auto,
        present_mode: Default::default(),
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: vec![],
    };

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::all(),
            })],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            ..Default::default()
        },
        depth_stencil: Default::default(),
        multisample: Default::default(),
        multiview: Default::default(),
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
                surface_configuration.width = size.width;
                surface_configuration.height = size.height;
                surface.configure(&device, &surface_configuration);
            }
            Event::RedrawRequested(_) => {
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
                                        r: 0.05,
                                        g: 0.062,
                                        b: 0.08,
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
                    render_pass.draw(0..3, 0..1)
                }
                queue.submit(Some(command_encoder.finish()));
                surface_texture.present();
            }
            _ => (),
        }
    })
}
