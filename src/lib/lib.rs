use wasm_bindgen::prelude::wasm_bindgen;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

pub fn init_logger() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub fn set_web_canvas(window: &mut Window, width: u32, height: u32) {
    use winit::dpi::PhysicalSize;
    window.set_inner_size(PhysicalSize::new(width, height));

    use winit::platform::web::WindowExtWebSys;
    web_sys::window()
        .and_then(|win| win.document())
        .and_then(|doc| {
            let dst = doc.get_element_by_id("wgpu-playground")?;
            let canvas = web_sys::Element::from(window.canvas());
            dst.append_child(&canvas).ok()?;
            Some(())
        })
        .expect("Couldn't append canvas to document body.");
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    init_logger();

    let event_loop = EventLoop::new();
    let mut window = WindowBuilder::new()
        .with_title("Wgpu Playground")
        .build(&event_loop)
        .unwrap();

    #[cfg(target_arch = "wasm32")]
    set_web_canvas(&mut window, 450, 400);

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => control_flow.set_exit(),
            _ => (),
        }
    })
}
