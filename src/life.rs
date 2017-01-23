extern crate piston;
#[macro_use] extern crate log;
extern crate env_logger;

extern crate opengl_graphics;
extern crate glutin_window;
extern crate graphics;
extern crate clap;

extern crate rand;

use opengl_graphics::{ GlGraphics, OpenGL };
use glutin_window::GlutinWindow as Window;
use piston::window::{ AdvancedWindow, WindowSettings };
use piston::event_loop::{Events, EventLoop};
use piston::input::{
    RenderEvent,
    UpdateEvent,
    PressEvent,
    ReleaseEvent,
    ResizeEvent,
    MouseCursorEvent,
    RenderArgs,
    UpdateArgs,
    Button,
};

mod world;
mod game;
use game::{ Game };
use world::{ random_world };

fn main() {
    use clap::{ Arg, App };
    env_logger::init().unwrap();

    let matches = App::new("Game of Life")
        .version("1.0")
        .author("Joar Wandborg <joar@wandborg.se>")
        .arg(Arg::with_name("ups")
            .short("u")
            .long("updates-per-second")
            .takes_value(true))
        .arg(Arg::with_name("fps")
            .short("f")
            .long("frames-per-second")
            .takes_value(true))
        .get_matches();

    let opengl = OpenGL::V3_2;

    let window_size = (640u32, 480u32);

    let mut window: Window = WindowSettings::new(
            "Game of Life",
            window_size,
        )
        .fullscreen(false)
        .vsync(true)
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::new(
        window_size,
        GlGraphics::new(opengl),
        random_world((64, 48)),
    );

    let mut events = window.events();

    let ups: u64 = matches.value_of("ups")
        .unwrap_or("30").parse().unwrap();
    let fps: u64 = matches.value_of("fps")
        .unwrap_or("30").parse().unwrap();

    events.set_ups(ups);
    events.max_fps(fps);

    while let Some(e) = events.next(&mut window) {
        e.mouse_cursor(|x, y| {
            game.update_mouse_pos(x, y);
        });

        e.resize(|width, height| {
            game.update_size(width, height);
        });

        if let Some(button) = e.press_args() {
            game.handle_button_press(button);
        }

        if let Some(button) = e.release_args() {
            game.handle_button_release(button);
        }

        if let Some(r) = e.render_args() {
            game.render(&r);
        }

        if let Some(u) = e.update_args() {
            game.tick(&u);
        }
    }
}
