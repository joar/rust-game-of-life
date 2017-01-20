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
use piston::window::WindowSettings;
use piston::event_loop::{Events, EventLoop};
use piston::input::{ RenderEvent, UpdateEvent, RenderArgs, UpdateArgs };

mod world;
mod game;
use game::{ GameSettings, Game };
use world::{ random_world };

fn main() {
    use clap::{ Arg, App };
    env_logger::init().unwrap();

    let matches = App::new("Game of Life")
        .version("1.0")
        .author("Joar Wandborg <joar@wandborg.se>")
        .arg(Arg::with_name("updates-per-second")
             .short("u")
             .takes_value(true))
        .get_matches();

    let opengl = OpenGL::V3_2;

    let settings: GameSettings = Default::default();

    let mut window: Window = WindowSettings::new(
            "Game of Life",
            settings.window_size()
        )
        .fullscreen(false)
        .vsync(true)
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::new(
        GlGraphics::new(opengl),
        settings.clone(),
        random_world(settings.grid_size.clone()),
    );

    let mut events = window.events();

    let updates_per_second: u64 = matches.value_of("updates-per-second")
        .unwrap_or("30").parse().unwrap();

    events.set_ups(updates_per_second);

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&r);
        }

        if let Some(u) = e.update_args() {
            game.update(&u);
        }
    }
}
