use std::vec;

use crate::physics_engine::PhysicsObject;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Point;
use sdl2::Sdl;
use sdl2::VideoSubsystem;


pub struct Visualizer{
    width: i32,
    height: i32,
    refreshrate: f64,
    sdl_context: Sdl,
    video_subsystem: VideoSubsystem,
    canvas: Canvas<Window>,
}


impl Visualizer{
    pub fn add_circle(&mut self, object: &PhysicsObject){
        let mut px:i32 = 0;
        let mut py:i32= 0;
        let mut r:f32 = 0.0;
    
        self.canvas.set_draw_color(sdl2::pixels::Color::RGB(255,255,255));
        for x in -object.radius..object.radius+1{
            px = x + object.position[0] as i32;
            for y in -object.radius..object.radius+1{
                py = y + object.position[1] as i32;
                
                r = (x*x + y*y) as f32;
                r = r.sqrt();
                if r < object.radius as f32{
                    self.canvas.draw_point(Point::new(px, py));
                }
            }
        }
    
        self.canvas.set_draw_color(sdl2::pixels::Color::RGB(255,0,0));
       
        for x in -object.radius..object.radius+1{
            px = x + object.position[0] as i32;
            for y in -object.radius..object.radius+1{
                py = y + object.position[1] as i32;
                
                r = (x*x + y*y) as f32;
                r = r.sqrt();
                if r <= (object.radius+2) as f32 && r > object.radius as f32{
                    self.canvas.draw_point(Point::new(px, py));
                }
            }
        }
    }

    pub fn render_frame(&mut self, objects: &Vec<PhysicsObject>){
        self.canvas.clear();

        for obj in objects{
            self.add_circle(obj);
        }
    }

    

    pub fn draw_frame(&mut self){

        self.canvas.set_draw_color(sdl2::pixels::Color::RGB(0,0,0));
        self.canvas.present();
        //std::thread::sleep(Duration::from_millis(15));

    }

    pub fn new(width:i32, height:i32, refreshrate:f64) -> Visualizer{
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas()
            .accelerated().build().map_err(|e| e.to_string()).expect("my own failure message");

        let mut visualizer = Visualizer{
                                    sdl_context:sdl_context,
                                    video_subsystem:video_subsystem,
                                    canvas:canvas,
                                    width:width,
                                    height:height,
                                    refreshrate:refreshrate,
                                    };
        
        return visualizer;

    }


}