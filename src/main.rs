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
extern crate glium;
extern crate glium_graphics;
extern crate shader_version;
extern crate input;
extern crate window;
extern crate glutin_window;
#[macro_use(lift)]
extern crate carboxyl;
extern crate carboxyl_window;

use window::WindowSettings;
use carboxyl::Signal;
use carboxyl_window::StreamingWindow;
use runners::Node;
use std::sync::Arc;

mod runners;


#[derive(Clone, Debug)]
struct Model;

#[derive(Clone, Debug)]
struct Triangle;
#[derive(Clone, Debug)]
struct EmptyNode;

impl Node for Triangle {}
impl Node for EmptyNode {}

/// Some trivial application logic
fn app_logic<W: StreamingWindow>(window: &W) -> Signal<Model> {
    Signal::new(Model)
}

/// A functional view
fn view((width, height): (u32, u32), model: Model) -> Arc<Box<Node>> {
    Arc::new(Box::new(EmptyNode))
}

fn main() {
    runners::run_glium(
        WindowSettings::new("carboxyl_window :: example/simple.rs", (640, 480)),
        |window| lift!(view, &window.size(), &app_logic(window))
    );
}
