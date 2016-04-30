extern crate sdl2;
extern crate sdl2_ttf;
extern crate rand;

use std::path::Path;
use std::thread;
use std::time::Duration;

use sdl2::pixels::Color;
use sdl2::event::Event::*;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode::*;
use sdl2::render::TextureQuery;
use rand::{Rng, Rand};
use rand::distributions::{IndependentSample, Range};

mod state;

pub use state::State;

pub enum GameError {
    Lolwtf
}

pub struct Game<'e>
{
    states: Vec<&'e mut State>
}

impl<'e> Game<'e> {
    pub fn new(state: &'e mut State) -> Game<'e>
    {
        Game
        {
            states: vec![state]
        }
    }

    pub fn push_state(&mut self, state: &'e mut State) {
        self.states.push(state)
    }

    pub fn pop_state() {

    }

    pub fn run(&mut self) {
        let screen_width = 800;
        let screen_height = 600;

        let mut rng = rand::thread_rng();
        let sdl_context = sdl2::init().unwrap();
        let mut timer = sdl_context.timer().unwrap();
        let mut event_pump = sdl_context.event_pump().unwrap();
        let video = sdl_context.video().unwrap();
        let ttf_context = sdl2_ttf::init().unwrap();

        let mut font = ttf_context.load_font(Path::new("resources/DejaVuSerif.ttf"), 128).unwrap();
        let surface = font.render("ruffel")
            .blended(Color::rand(&mut rng)).unwrap();

        let window = video.window("Ruffel", 800, 600)
            .position_centered().opengl()
            .build().unwrap();

        let mut renderer = window.renderer()
            .accelerated()
            .build().unwrap();

        let mut font_texture = renderer.create_texture_from_surface(&surface).unwrap();

        let TextureQuery { width, height, .. } = font_texture.query();

        // If the example text is too big for the screen, downscale it (and center irregardless)
        let padding = 64;

        // Initialize State handlers
        for s in &mut self.states
        {
            s.init();
        }

        let mut done = false;
        let mut delta = Duration::new(0, 0);
        while !done {
            let start_time = timer.ticks();

            for event in event_pump.poll_iter() {
                match event {
                    Quit { .. } => done = true,
                    KeyDown { keycode, .. } => match keycode {
                        Some(Escape) => done = true,
                        _ => {}
                    },
                    _ => {}
                }
            }

            let between = Range::new(0, 400);
            let target = Rect::new(between.ind_sample(&mut rng), 50, between.ind_sample(&mut rng) as u32, 500);
            renderer.set_draw_color(Color::rand(&mut rng));
            renderer.clear();
            renderer.copy(&mut font_texture, None, Some(target));
            renderer.present();


            // Updating
            for s in &mut self.states
            {
                s.update(delta);
            }

            // Rendering
            for s in &mut self.states
            {
                s.draw();
            }

            let end_time = timer.ticks();
            delta = Duration::from_millis((end_time - start_time) as u64);
            thread::sleep_ms(1000/60);
        }
    }
}