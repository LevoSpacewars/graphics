extern crate sdl2; 
extern crate gl;
use rand::Rng;


use std::time::Instant;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Point;


mod physics_engine;
use physics_engine::{PhysicsEngine, PhysicsObject, Gradient};

mod visualizer;
use visualizer::Visualizer;



 
pub fn main() {

    let mut visualizer = Visualizer::new(500,500,60.0);
    let mut physics_engine = PhysicsEngine::new(Gradient {  },0.1);
    let mut rng = rand::thread_rng();

    for i in 0..50{
        let a: f64 =  rng.gen();
        let b: f64 =  rng.gen();
        let c: f64 =  rng.gen();
        let d: f64 =  rng.gen();
        physics_engine.populate([500.0 * a ,500.0 * b], [100.0 * c - 50.0 ,100.0 * d - 50.0 ], 5.0, 5);
    }
    
    
    let mut counter = 0;
    while counter != 1000{
        counter += 1;
        let now = Instant::now();
        physics_engine.propagate();
        println!("Propagate Time = {}", now.elapsed().as_millis());

        let now = Instant::now();
        visualizer.reset();
        println!("reset time = {}", now.elapsed().as_millis());

        let now = Instant::now();
        visualizer.render_frame(physics_engine.get_positions());
        println!("Drawing time = {}", now.elapsed().as_millis());

        let now = Instant::now();
        visualizer.draw_frame();
        println!("display time = {}", now.elapsed().as_millis());

        if counter%10 == 0{
            let now = Instant::now();
            physics_engine.re_center();
            println!("recenter time = {}", now.elapsed().as_millis());
        }
    }

    
}



