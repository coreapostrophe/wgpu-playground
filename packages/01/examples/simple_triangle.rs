use commonlib::renderer::RendererBuilder;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

fn main() {
    env_logger::init();
    
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Simple Triangle")
        .build(&event_loop)
        .expect("to create window");

    let mut renderer = RendererBuilder::new(window)
        .create_instance()
        .create_surface()
        .get_adapter()
        .get_device(Some("Device"))
        .create_surface_configuration()
        .create_shader_module(Some("Shader"), include_str!("simple_triangle.wgsl"))
        .create_pipeline_layout(Some("Pipeline Layout"))
        .create_render_pipeline(Some("Render Pipeline"))
        .build();

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
                    let surface_configuration = renderer
                        .mut_surface_configuration()
                        .expect("renderer to have a surface configuration");
                    surface_configuration.width = size.width;
                    surface_configuration.height = size.height;
                }

                let surface_configuration = renderer
                    .surface_configuration()
                    .expect("renderer to have a surface configuration");
                let surface = renderer
                    .surface()
                    .expect("renderer to have a surface configuration");
                let device = renderer
                    .device()
                    .expect("renderer to have a surface configuration");
                surface.configure(device, surface_configuration);
            }
            Event::RedrawRequested(_) => {
                let surface = renderer
                    .surface()
                    .expect("renderer to have a surface configuration");
                let device = renderer
                    .device()
                    .expect("renderer to have a surface configuration");
                let queue = renderer
                    .queue()
                    .expect("renderer to have a surface configuration");
                let render_pipeline = renderer
                    .render_pipeline()
                    .expect("renderer to have a surface configuration");
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
                    render_pass.draw(0..3, 0..1)
                }
                queue.submit(Some(command_encoder.finish()));
                surface_texture.present();
            }
            _ => (),
        }
    });
}
