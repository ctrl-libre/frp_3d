use glium::{ Surface, DrawError };
use glium_graphics::{ Glium2d, GliumGraphics, GliumWindow, GlyphCache };
use glutin_window::GlutinWindow;
use carboxyl_window::{ StreamingWindow, WindowWrapper };
use carboxyl::Signal;
use elmesque::{ Element, Renderer };
use shader_version::OpenGL;
use window::WindowSettings;
use std::sync::Arc;
use glium::backend::{Facade};

pub trait Node: Send + Sync + 'static {
    fn draw(&self, surface: &mut Surface, display: &Facade) -> Result<(), DrawError>;
}

pub fn run_glium<F>(settings: WindowSettings, app: F)
    where F: FnOnce(&WindowWrapper<GlutinWindow>) -> Signal<Arc<Box<Node>>>,
{
    use std::rc::Rc;
    use std::cell::RefCell;
    use std::path::Path;

    const GLVERSION: OpenGL = OpenGL::_2_1;
    let glutin_window = Rc::new(RefCell::new(
        GlutinWindow::new(GLVERSION, settings)));
    let window = WindowWrapper::new(glutin_window.clone(), 10_000_000);
    let scene = lift!(|s, m| (s, m), &window.size(), &app(&window));
    let glium_window = GliumWindow::new(&glutin_window).unwrap();

    window.run(|| {
        let ((w, h), element) = scene.sample();
        let mut target = glium_window.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        target.clear_depth(1.0);
        element.draw(&mut target, &glium_window);
        target.finish();
    });
}
