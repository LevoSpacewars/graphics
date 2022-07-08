use std::vec;

use crate::physics_engine::PhysicsObject;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use std::time::Duration;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Point;
use sdl2::Sdl;
use sdl2::VideoSubsystem;
use std::time::Instant;

pub struct Visualizer{
    width: usize,
    height: usize,
    refreshrate: f64,
    sdl_context: Sdl,
    video_subsystem: VideoSubsystem,
    canvas:  Canvas<Window>,
    pub texture: sdl2::render::Texture<'static>,
    _texture_creator: TextureCreator<WindowContext>,
    pixels: Vec<u8>,
    timer: Instant,
    ptime: u128,

}



impl Visualizer{
    pub fn add_circle(&mut self, object: &PhysicsObject){
        let mut px:f64 = 0.0;
        let mut py:f64 = 0.0;
        let mut r:f64 = 0.0;
        let pitch:usize = self.width * 3;

            for x in 0..2*object.radius as usize{
                px = x as f64 + object.position[0] - object.radius as f64;
                if (px < 0.0) | (px > self.width as f64){
                    continue;
                }

                for y in 0..2*object.radius as usize{
                    py = y as f64 + object.position[1] - object.radius as f64;
                    if (py < 0.0) | (py > self.height as f64){
                        continue;
                    }
                    //println!("{} {}", px,py);

                    r = (x as f64 - object.radius as f64)*(x as f64 - object.radius as f64) + (y as f64 - object.radius as f64)*(y as f64 - object.radius as f64);
                    r = r.sqrt();
                    if r < object.radius as f64{
                        let offset = py as usize * pitch + px as usize * 3;
                        //println!("{}",offset);
                        self.pixels[offset] = 255 as u8;
                        self.pixels[offset + 1] = 255 as u8;
                        self.pixels[offset + 2] = 255 as u8;
                    }

                    else if r + 1.0 <= object.radius as f64{
                        let offset = py as usize * pitch + px as usize * 3;
                        self.pixels[offset] = 255 as u8;
                        self.pixels[offset + 1] = 0 as u8;
                        self.pixels[offset + 2] = 0 as u8;
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

    pub fn reset(&mut self){
        self.pixels = vec![0;self.height*self.width*3];
    }

    

    pub fn draw_frame(&mut self){



        self.texture.update(None, &self.pixels, self.width*3);

        //self.canvas.clear();
        self.canvas.copy(&self.texture, None, None).expect("msg");

        self.canvas.set_draw_color(sdl2::pixels::Color::RGB(0,0,0));
        

        self.canvas.present();
        
        if self.timer.elapsed().as_millis() < 15 {
            std::thread::sleep(Duration::from_millis(self.timer.elapsed().as_millis() as u64));
        }

        self.timer = Instant::now();

        

    }

    pub fn new(width:usize, height:usize, refreshrate:f64) -> Visualizer{
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("rust-sdl2 demo", width as u32, height as u32)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas()
            .accelerated().build().map_err(|e| e.to_string()).expect("my own failure message");
        
        let texture_creator = canvas.texture_creator();
         

        let texture_creator_pointer = &texture_creator as *const TextureCreator<WindowContext>;
        let texture = unsafe { &*texture_creator_pointer }.create_texture_streaming(sdl2::pixels::PixelFormatEnum::RGB24, width as u32, height as u32).expect("msg");
        
        let pixels = vec![0;width*height*3];

        let inst = Instant::now();
        
        let mut visualizer = Visualizer{
                                    sdl_context:sdl_context,
                                    video_subsystem:video_subsystem,
                                    canvas:canvas,
                                    width:width,
                                    height:height,
                                    refreshrate:refreshrate,
                                    _texture_creator: texture_creator,
                                    texture: texture,
                                    pixels:pixels,
                                    ptime: 0,
                                    timer: inst,
                                    };
        return visualizer;
    }

}