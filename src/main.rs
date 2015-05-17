//! Simple interactive application
//!
//! This example is very simple. You can move around a circle that follows the
//! mouse position. Using the mouse cursor you can change the color of the
//! circle and a little number displayed on its center.
//!
//! The event handling logic here is trivial, since the output is a very simple
//! function of input events. It is intended to demonstrate how you can set up
//! an application using carboxyl, carboxyl_window and elmesque.

extern crate elmesque;
extern crate graphics;
#[macro_use(implement_vertex,uniform)]
extern crate glium;
extern crate glium_graphics;
extern crate shader_version;
extern crate input;
extern crate window;
extern crate glutin_window;
#[macro_use(lift)]
extern crate carboxyl;
extern crate carboxyl_window;
extern crate nalgebra;
extern crate num;

use window::WindowSettings;
use carboxyl::Signal;
use carboxyl_window::StreamingWindow;
use runners::Node;
use std::sync::Arc;
use cam::Camera;
use nalgebra::PerspMat3;
use glium::{Surface, DrawError};
use glium::backend::{Facade};
use glium::draw_parameters::{DrawParameters, DepthTest, BackfaceCullingMode};

mod runners;
mod cam;

static vertex_shader: &'static str = "
    #version 110

    uniform mat4 model_view_proj;

    attribute vec3 position;
    attribute vec2 texcoord;

    varying vec2 v_texcoord;

    void main() {
        v_texcoord = texcoord;
        gl_Position = model_view_proj * vec4(position, 1.0);
    }
";

static fragment_shader: &'static str = 
"
    #version 110

    varying vec2 v_texcoord;

    void main() {
        gl_FragColor = vec4(v_texcoord, 1.0, 1.0);
    }
";


#[derive(Clone, Debug)]
struct Model;

#[derive(Clone, Debug)]
struct Triangle {
    vertices: [[f64; 3]; 3],
    camera: Camera<f64>,
}
#[derive(Clone, Debug)]
struct EmptyNode;

#[derive(Copy, Clone)]
struct PosTexVertex {
    position: [f64; 3],
    texcoords: [f64; 2],
}

implement_vertex!(PosTexVertex, position, texcoords);

impl Triangle {
    fn new(vertices: [PosTexVertex; 3], camera: Camera<f64>) {
        Triangle{vertices: vertices, camera: camera}
    }
}

impl Node for Triangle {
    fn draw(&self, surface: &mut Surface, display: &Facade) -> Result<(), DrawError> {
        // Building the uniforms
        let proj = PerspMat3::new(self.aspect_ratio, 63.0 / 180.0 * 3.14, 0.001, 1000.0);

        // Draw a frame
        let draw_params = {
            let mut def: DrawParameters = std::default::Default::default();
            def.depth_test = DepthTest::IfLessOrEqual;
            def.backface_culling = BackfaceCullingMode::CullingDisabled;
            def
        };

        let vertex_buffer = glium::vertex::VertexBuffer::new(display, &self.vertices);
        let index_buffer = glium::index::IndexBuffer::new(display, glium::index::TrianglesList::new(vec![0, 1, 2]));
        let program = glium::program::Program::new(display, glium::program::ProgramCreationInput::SourceCode{
            vertex_shader: vertex_shader,
            tessellation_control_shader: None,
            tessellation_evaluation_shader: None,
            geometry_shader: None,
            fragment_shader: fragment_shader,
            transform_feedback_varyings: None
        });

        surface.draw(
            &vertex_buffer, &index_buffer, &program,
            &uniform! {
                model_view_proj: self.camera.as_projection_matrix(),
            },
            &draw_params
        )
    }
}

impl Node for EmptyNode {
    fn draw(&self, surface: &mut Surface, display: &Facade) -> Result<(), DrawError> { Ok(()) }
}

/// Some trivial application logic
fn app_logic<W: StreamingWindow>(window: &W) -> Signal<Model> {
    Signal::new(Model)
}

/// A functional view
fn view((width, height): (u32, u32), model: Model) -> Arc<Box<Node>> {
        let data = &[
            PosTexVertex {
                position: [0.0, 0.0, 0.4],
                texcoords: [0.0, 1.0]
            },
            PosTexVertex {
                position: [12.0, 4.5, -1.8],
                texcoords: [1.0, 0.5]
            },
            PosTexVertex {
                position: [-7.124, 0.1, 0.0],
                texcoords: [0.0, 0.4]
            },
        ];
    Arc::new(Box::new(EmptyNode))
}

fn main() {



    runners::run_glium(
        WindowSettings::new("carboxyl_window :: example/simple.rs", (640, 480)),
        |window| lift!(view, &window.size(), &app_logic(window))
    );
}
