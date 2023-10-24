use wgpu::{
    Adapter, Backends, BindGroupLayout, BindGroupLayoutEntry, DepthStencilState, Device,
    DeviceDescriptor, IndexFormat, Instance, InstanceDescriptor, PipelineLayout, PrimitiveState,
    PrimitiveTopology, Queue, RenderPipeline, ShaderModule, Surface, SurfaceConfiguration,
    TextureUsages, VertexBufferLayout,
};
use winit::window::Window;

pub struct Renderer<'a> {
    window: Option<Window>,
    instance: Option<Instance>,
    surface: Option<Surface>,
    adapter: Option<Adapter>,
    device: Option<Device>,
    queue: Option<Queue>,
    surface_configuration: Option<SurfaceConfiguration>,
    shader: Option<ShaderModule>,
    pipeline_layout: Option<PipelineLayout>,
    render_pipeline: Option<RenderPipeline>,
    vertex_buffers_layout: Vec<VertexBufferLayout<'a>>,
    bind_group_layouts: Vec<BindGroupLayout>,
}

impl<'a> Renderer<'a> {
    pub fn window(&self) -> Option<&Window> {
        self.window.as_ref()
    }
    pub fn mut_window(&mut self) -> Option<&mut Window> {
        self.window.as_mut()
    }

    pub fn instance(&self) -> Option<&Instance> {
        self.instance.as_ref()
    }
    pub fn mut_instance(&mut self) -> Option<&mut Instance> {
        self.instance.as_mut()
    }

    pub fn surface(&self) -> Option<&Surface> {
        self.surface.as_ref()
    }
    pub fn mut_surface(&mut self) -> Option<&mut Surface> {
        self.surface.as_mut()
    }

    pub fn adapter(&self) -> Option<&Adapter> {
        self.adapter.as_ref()
    }
    pub fn mut_adapter(&mut self) -> Option<&mut Adapter> {
        self.adapter.as_mut()
    }

    pub fn device(&self) -> Option<&Device> {
        self.device.as_ref()
    }
    pub fn mut_device(&mut self) -> Option<&mut Device> {
        self.device.as_mut()
    }

    pub fn queue(&self) -> Option<&Queue> {
        self.queue.as_ref()
    }
    pub fn mut_queue(&mut self) -> Option<&mut Queue> {
        self.queue.as_mut()
    }

    pub fn surface_configuration(&self) -> Option<&SurfaceConfiguration> {
        self.surface_configuration.as_ref()
    }
    pub fn mut_surface_configuration(&mut self) -> Option<&mut SurfaceConfiguration> {
        self.surface_configuration.as_mut()
    }

    pub fn shader(&self) -> Option<&ShaderModule> {
        self.shader.as_ref()
    }
    pub fn mut_shader(&mut self) -> Option<&mut ShaderModule> {
        self.shader.as_mut()
    }

    pub fn pipeline_layout(&self) -> Option<&PipelineLayout> {
        self.pipeline_layout.as_ref()
    }
    pub fn mut_pipeline_layout(&mut self) -> Option<&mut PipelineLayout> {
        self.pipeline_layout.as_mut()
    }

    pub fn render_pipeline(&self) -> Option<&RenderPipeline> {
        self.render_pipeline.as_ref()
    }
    pub fn mut_render_pipeline(&mut self) -> Option<&mut RenderPipeline> {
        self.render_pipeline.as_mut()
    }

    pub fn vertex_buffers_layout(&self) -> &Vec<VertexBufferLayout<'a>> {
        self.vertex_buffers_layout.as_ref() as &Vec<VertexBufferLayout<'a>>
    }
    pub fn mut_vertex_buffers_layout(&mut self) -> &mut Vec<VertexBufferLayout<'a>> {
        self.vertex_buffers_layout.as_mut()
    }

    pub fn bind_group_layouts(&self) -> &Vec<BindGroupLayout> {
        self.bind_group_layouts.as_ref()
    }
    pub fn mut_bind_group_layouts(&mut self) -> &mut Vec<BindGroupLayout> {
        self.bind_group_layouts.as_mut()
    }
}

pub struct RendererBuilder<'a> {
    window: Option<Window>,
    instance: Option<Instance>,
    surface: Option<Surface>,
    adapter: Option<Adapter>,
    device: Option<Device>,
    queue: Option<Queue>,
    surface_configuration: Option<SurfaceConfiguration>,
    shader: Option<ShaderModule>,
    pipeline_layout: Option<PipelineLayout>,
    render_pipeline: Option<RenderPipeline>,
    primitive_state: Option<PrimitiveState>,
    vertex_buffers_layout: Vec<VertexBufferLayout<'a>>,
    bind_group_layouts: Vec<BindGroupLayout>,
    depth_stencil_state: Option<DepthStencilState>,
}

impl<'a> RendererBuilder<'a> {
    pub fn new(window: Window) -> Self {
        Self {
            window: Some(window),
            instance: None,
            surface: None,
            adapter: None,
            device: None,
            queue: None,
            surface_configuration: None,
            shader: None,
            pipeline_layout: None,
            render_pipeline: None,
            primitive_state: None,
            vertex_buffers_layout: Vec::new(),
            bind_group_layouts: Vec::new(),
            depth_stencil_state: None,
        }
    }

    pub fn create_instance(mut self) -> Self {
        self.instance = Some(Instance::new(InstanceDescriptor {
            backends: Backends::VULKAN,
            dx12_shader_compiler: Default::default(),
        }));
        self
    }

    pub fn create_surface(mut self) -> Self {
        let window = self.window.as_ref().expect("renderer to have a window");
        let instance = self
            .instance
            .as_ref()
            .expect("renderer to have an instance");
        self.surface = Some(
            unsafe { instance.create_surface(window) }.expect("to create surface from instance"),
        );
        self
    }

    pub fn get_adapter(mut self) -> Self {
        let instance = self
            .instance
            .as_ref()
            .expect("renderer to have an instance");
        let surface = self.surface.as_ref().expect("renderer to have a surface");
        self.adapter = Some(pollster::block_on(Self::request_adapter(
            &instance, &surface,
        )));
        self
    }

    async fn request_adapter(instance: &Instance, surface: &Surface) -> Adapter {
        instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::None,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .expect("instance to have a compatible adapter")
    }

    pub fn get_device(mut self, label: Option<&str>) -> Self {
        let adapter = self.adapter.as_ref().expect("renderer to have an adapter");
        let (device, queue) = pollster::block_on(Self::request_device(&adapter, label));
        self.device = Some(device);
        self.queue = Some(queue);
        self
    }

    async fn request_device(adapter: &Adapter, label: Option<&str>) -> (Device, Queue) {
        adapter
            .request_device(
                &DeviceDescriptor {
                    label,
                    features: Default::default(),
                    limits: Default::default(),
                },
                None,
            )
            .await
            .expect("adapter to have an available device")
    }

    pub fn create_surface_configuration(mut self) -> Self {
        let window = self.window.as_ref().expect("renderer to have a window");
        let surface = self.surface.as_ref().expect("renderer to have a surface");
        let adapter = self.adapter.as_ref().expect("renderer to have an adapter");

        let size = window.inner_size();
        let surface_capabilities = surface.get_capabilities(&adapter);
        let format = surface_capabilities
            .formats
            .iter()
            .copied()
            .find(|format| format.is_srgb())
            .expect("surface to have srgb texture format");

        self.surface_configuration = Some(SurfaceConfiguration {
            format,
            usage: TextureUsages::RENDER_ATTACHMENT,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
        });
        self
    }

    pub fn create_shader_module(mut self, label: Option<&str>, shader_path: &str) -> Self {
        let device = self.device.as_ref().expect("renderer to have a device");
        self.shader = Some(device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label,
            source: wgpu::ShaderSource::Wgsl(shader_path.into()),
        }));
        self
    }

    pub fn add_vertex_buffer_layout(mut self, buffer: VertexBufferLayout<'a>) -> Self {
        self.vertex_buffers_layout.push(buffer);
        self
    }

    pub fn add_bind_group_layout(
        mut self,
        label: Option<&str>,
        entries: &[BindGroupLayoutEntry],
    ) -> Self {
        let device = self.device.as_ref().expect("renderer to have a device");
        let bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label, entries });
        self.bind_group_layouts.push(bind_group_layout);
        self
    }

    pub fn create_pipeline_layout(mut self, label: Option<&str>) -> Self {
        let device = self.device.as_ref().expect("renderer to have a device");
        let bind_group_layouts: &Vec<BindGroupLayout> = self.bind_group_layouts.as_ref();
        let referenced_bind_group_layouts: Vec<&BindGroupLayout> =
            bind_group_layouts.iter().collect();

        self.pipeline_layout = Some(device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label,
                bind_group_layouts: &referenced_bind_group_layouts,
                push_constant_ranges: &[],
            },
        ));
        self
    }

    pub fn set_primitive_state(
        mut self,
        topology: PrimitiveTopology,
        strip_index_format: Option<IndexFormat>,
    ) -> Self {
        self.primitive_state = Some(wgpu::PrimitiveState {
            topology,
            strip_index_format,
            ..Default::default()
        });
        self
    }

    pub fn set_depth_stencil_state(mut self, depth_stencil_state: DepthStencilState) -> Self {
        self.depth_stencil_state = Some(depth_stencil_state);
        self
    }

    pub fn create_render_pipeline(mut self, label: Option<&str>) -> Self {
        let device = self.device.as_ref().expect("renderer to have a device");
        let shader = self
            .shader
            .as_ref()
            .expect("renderer to have a shader module");
        let surface_configuration = self
            .surface_configuration
            .as_ref()
            .expect("renderer to have a surface configuration");
        let pipeline_layout = self
            .pipeline_layout
            .as_ref()
            .expect("renderer to have a pipeline layout");
        let vertex_buffers_layout: &Vec<VertexBufferLayout> = self.vertex_buffers_layout.as_ref();

        let primitive_state = match self.primitive_state.clone() {
            Some(primitive_state) => primitive_state,
            None => wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                ..Default::default()
            },
        };

        let depth_stencil_state = self.depth_stencil_state.clone();

        self.render_pipeline = Some(device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label,
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &vertex_buffers_layout,
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: surface_configuration.format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::all(),
                    })],
                }),
                primitive: primitive_state,
                depth_stencil: depth_stencil_state,
                multisample: Default::default(),
                multiview: Default::default(),
            },
        ));
        self
    }

    pub fn build(self) -> Renderer<'a> {
        Renderer {
            window: self.window,
            instance: self.instance,
            surface: self.surface,
            adapter: self.adapter,
            device: self.device,
            queue: self.queue,
            surface_configuration: self.surface_configuration,
            shader: self.shader,
            pipeline_layout: self.pipeline_layout,
            render_pipeline: self.render_pipeline,
            bind_group_layouts: self.bind_group_layouts,
            vertex_buffers_layout: self.vertex_buffers_layout,
        }
    }
}
