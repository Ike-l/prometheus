use small_read_only::ReadOnly;
use wgpu::{Device, DeviceDescriptor, Features, Instance, Limits, MemoryHints, PowerPreference, Queue, RenderPipeline, RequestAdapterOptions, Surface, SurfaceConfiguration, TextureUsages};
use winit::{event_loop::ActiveEventLoop, window::Window};

use crate::{prelude::{concat_title, App, Res, ResMut}, prom_plugins::dependent_plugins::render_plugin::{bind_groups::{bind_group_layouts::BindGroupLayouts, default_bind_groups::DefaultBindGroups}, pipelines::{main_pipeline::create_main_pipeline, shadow_pipeline::create_shadow_pipeline}, texture::Texture}};

use super::dimensions::WindowDimensions;

#[derive(Debug, ReadOnly)]
pub struct WindowState {
    config: SurfaceConfiguration,
    surface: Surface<'static>,
    device: Device,
    queue: Queue,

    depth_texture: Texture,

    main_pipeline: RenderPipeline,
    shadow_pipeline: RenderPipeline,

    bind_group_layouts: BindGroupLayouts
}



impl WindowState {
    pub async fn new(window: Window, title: &'static str) -> Self {
        let size = window.inner_size();
        let title = concat_title(title);

        let instance = Instance::default();
        let surface = instance.create_surface(window).unwrap();
        let adapter = instance.request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false
        }).await.unwrap();

        let (device, queue) = adapter.request_device(&DeviceDescriptor { 
            label: Some(&title("DeviceDescriptor")), 
            required_features: Features::empty(), 
            required_limits: Limits {
                max_bind_groups: 6,
                ..Default::default()
            }, 
            memory_hints: MemoryHints::default() 
        }, None).await.unwrap();

        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_format = surface_capabilities.formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_capabilities.formats[0]);

        
        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: Vec::new(),
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        let depth_texture = Texture::create_depth_texture(&device, &config);

        let bind_group_layouts = BindGroupLayouts::new(&device);
        let main_pipeline = create_main_pipeline(&device, config.format, &bind_group_layouts);
        let shadow_pipeline = create_shadow_pipeline(&device, &bind_group_layouts);

        Self {
            surface,
            device,
            queue,
            config,
            depth_texture,
            main_pipeline,
            shadow_pipeline,
            bind_group_layouts,
        }
    }
    
}

pub fn instanciate_state(event_loop: Res<&ActiveEventLoop>, mut app: ResMut<&mut App>) {
    let size = event_loop.primary_monitor().unwrap().size();

    let window = event_loop.create_window(Window::default_attributes()
        .with_title(app.title())
        .with_inner_size(size)
    ).unwrap();

    let state = pollster::block_on(
        WindowState::new(window, app.title())
    );

    let default_bind_groups = DefaultBindGroups::new(state.device(), state.bind_group_layouts());

    let window_dimensions = WindowDimensions::from(size);

    app.insert_resource(window_dimensions);
    app.insert_resource(state);
    app.insert_resource(default_bind_groups);
}



