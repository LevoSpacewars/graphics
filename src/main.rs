extern crate sdl2; 
extern crate gl;


use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Point;
 
pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    
 
    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas()
        .accelerated().build().map_err(|e| e.to_string()).expect("my own failure message");


    let mut counter = 0;
    while counter < 200{
        canvas.clear();
        counter += 1;
        let center= Point::new(counter,counter);
        draw_circle(&mut canvas, center, 10,5);
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0,0,0));
        canvas.present();

        std::thread::sleep(Duration::from_millis(10));
    }
    
}



fn draw_circle(canvas: &mut Canvas<Window>, center: Point, radius: i32, br:i32){
    let mut px:i32 = 0;
    let mut py:i32= 0;
    let mut r:f32 = 0.0;

    canvas.set_draw_color(sdl2::pixels::Color::RGB(255,255,255));
    for x in -radius..radius+1{
        px = x + center.x as i32;
        for y in -radius..radius+1{
            py = y + center.y as i32;
            
            r = (x*x + y*y) as f32;
            r = r.sqrt();
            if r < radius as f32{
                canvas.draw_point(Point::new(px, py));
            }
        }
    }

    canvas.set_draw_color(sdl2::pixels::Color::RGB(255,0,0));
   
    for x in -radius-br..radius+br+1{
        px = x + center.x as i32;
        for y in -radius-br..radius+br + 1{
            py = y + center.y as i32;
            
            r = (x*x + y*y) as f32;
            r = r.sqrt();
            if r <= (radius+br) as f32 && r > radius as f32{
                canvas.draw_point(Point::new(px, py));
            }
        }
    }




}
