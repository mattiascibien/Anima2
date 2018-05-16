extern crate gfx;

extern crate glutin;
extern crate gfx_window_glutin;

use gfx::Device;
use gfx_window_glutin as gfx_glutin;
use glutin::{GlContext, GlRequest};
use glutin::Api::OpenGl;

type ColorFormat = gfx::format::Srgba8;
type DepthFormat = gfx::format::DepthStencil;

const WINDOW_TITLE: &str = "Anima Render";

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

pub fn render(width: u32, height: u32, vsync: bool, file: &str) {
    let mut events_loop = glutin::EventsLoop::new();
    let builder = glutin::WindowBuilder::new()
        .with_title(WINDOW_TITLE)
        .with_dimensions(width, height);
    let context = glutin::ContextBuilder::new()
        .with_gl(GlRequest::Specific(OpenGl,(3,2)))
        .with_vsync(vsync);

    let (window, mut device, mut _factory, _rtv, mut _stv) = gfx_glutin::init::<ColorFormat, DepthFormat>(builder, context, &events_loop);

    let mut running = true;
    while running {
        events_loop.poll_events(|event| {
            if let glutin::Event::WindowEvent { event, .. } = event {
                match event {
                    glutin::WindowEvent::KeyboardInput {
                        input: glutin::KeyboardInput {
                            virtual_keycode: Some(glutin::VirtualKeyCode::Escape), ..
                        }, ..
                    } => running = false,
                    _ => {}
                }
            }
        });

        window.swap_buffers().unwrap();
        device.cleanup();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
