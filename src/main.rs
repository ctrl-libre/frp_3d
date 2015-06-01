#[macro_use(gfx_vertex, gfx_parameters)]
extern crate gfx;
extern crate gfx_device_gl;
extern crate glutin;
#[macro_use(lift)]
extern crate carboxyl;
extern crate carboxyl_time;
extern crate carboxyl_window;
extern crate window;
extern crate input;
extern crate shader_version;
extern crate glutin_window;
extern crate gfx_func;
extern crate time;
extern crate nalgebra;

use std::rc::Rc;
use std::cell::RefCell;
use carboxyl_time::every;
use carboxyl_window::{ SourceWindow, RunnableWindow, StreamingWindow };
use carboxyl_window::button::Direction;
use window::{ WindowSettings };
use shader_version::OpenGL;
use glutin_window::GlutinWindow;
use gfx::traits::FactoryExt;
use gfx::{ batch, Stream, ClearData };
use gfx_func::element::{ Batch, Cleared, Draw };
use gfx_func::cam::{ self, MovementState3 };
use input::{ Button, Key };
use time::Duration;
use nalgebra::{ PerspMat3, Pnt3, cast };

pub mod shared_win;


gfx_vertex!( Vertex {
    a_Pos@ pos: [f32; 3],
    a_Color@ color: [f32; 3],
});

gfx_parameters!( Params {
    model_view_proj@ model_view_proj: [[f32; 4]; 4],
});

struct ButtonConfig {
    forward: Button,
    backward: Button,
    right: Button,
    left: Button,
    up: Button,
    down: Button,
}

impl Default for ButtonConfig {
    fn default() -> ButtonConfig {
        ButtonConfig {
            forward: Button::Keyboard(Key::W),
            backward: Button::Keyboard(Key::S),
            right: Button::Keyboard(Key::D),
            left: Button::Keyboard(Key::A),
            up: Button::Keyboard(Key::Space),
            down: Button::Keyboard(Key::LShift),
        }
    }
}

fn main() {
    const GLVERSION: OpenGL = OpenGL::_2_1;
    let settings = WindowSettings::new("gfx + carboxyl_window", (640, 480));
    let window = Rc::new(RefCell::new(GlutinWindow::new(GLVERSION, settings)));
    let (mut stream, mut device, mut factory) = shared_win::init_shared(window.clone());
    let mut source = SourceWindow::new(window.clone());

    let buttons = source.buttons();
    let ticks = every(Duration::milliseconds(5))
        .map(|dt| dt.num_milliseconds() as f64 / 1e3);
    let relative = source.cursor()
        .snapshot(&ticks, |pos, _| pos)
        .scan(
            ((0.0, 0.0), (0.0, 0.0)),
            |(old, _), new| (new, (old.0 - new.0, old.1 - new.1))
        )
        .snapshot(&ticks, |(_, delta), _| delta);
    let projection = lift!(
        |(w, h)| PerspMat3::new(w as f64 / h as f64, 63.0 / 180.0 * 3.14, 0.001, 1000.0),
        &source.size()
    );
    let button_config = ButtonConfig::default();
    let camera = cam::fps_camera(
        Pnt3::new(0.0, 0.0, 2.0),
        &ticks,
        &lift!(
            MovementState3::new,
            &Direction::track(&buttons, button_config.right, button_config.left),
            &Direction::track(&buttons, button_config.up, button_config.down),
            &Direction::track(&buttons, button_config.backward, button_config.forward)
        ),
        &cam::space_attitude(&relative, 0.003),
        &projection
    );

    let triangle = {
        let mesh = factory.create_mesh(&[
            Vertex { pos: [ -0.5, -0.5, -1.0 ], color: [1.0, 0.0, 0.0] },
            Vertex { pos: [  0.5, -0.5, -1.0 ], color: [0.0, 1.0, 0.0] },
            Vertex { pos: [  0.0,  0.5, -1.0 ], color: [0.0, 0.0, 1.0] },
        ]);
        let program = {
            let vs = gfx::ShaderSource {
                glsl_120: Some(include_bytes!("triangle_120.glslv")),
                .. gfx::ShaderSource::empty()
            };
            let fs = gfx::ShaderSource {
                glsl_120: Some(include_bytes!("triangle_120.glslf")),
                .. gfx::ShaderSource::empty()
            };
            factory.link_program_source(vs, fs).unwrap()
        };
        lift!(
            move |cam| {
                let data = Params {
                    model_view_proj: *cast::<_, nalgebra::Mat4<f32>>(cam.proj_view()).as_array(),
                    _r: std::marker::PhantomData,
                };
                Batch(batch::Full::new(mesh.clone(), program.clone(), data).unwrap())
            },
            &camera
        )
    };
    let clear = ClearData { color: [0.3, 0.3, 0.3, 1.0], depth: 1.0, stencil: 0 };
    let scene = lift!(move |elem| Cleared::new(clear, elem), &triangle);

    source.run_with(120.0, || {
        let current = scene.sample();
        current.draw(&mut stream);
        stream.present(&mut device);
    });
}
