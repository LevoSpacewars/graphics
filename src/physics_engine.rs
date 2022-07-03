extern crate sdl2;

use std::vec;
use std::collections::HashMap;
use sdl2::rect::Point;

use ndarray::ArrayBase;
use ndarray::array;


pub struct PhysicsObject<'a>{
    pub position: [f64; 2],
    pub veclocity: [f64; 2],
    pub acceleration: [f64; 2],
    pub mass: f64,
    pub radius: i32,
    pub group: &'a str,
    pub nacceleration: [f64; 2],
    pub nposition:[f64; 2],
}

pub struct Gradient{

}

impl Gradient{
    pub fn func(pos: &[f64;2])->[f64;2]{
        let x = -(pos[0]-300.0)/((pos[0]-300.0)*(pos[0]-300.0) + 10.0).sqrt() * 1.0;
        let y = -(pos[1]-300.0)/((pos[1]-300.0)*(pos[1]-300.0) + 10.0).sqrt();
        println!("{} {} | {} {}",pos[0],pos[1],x,y);


        return [x,y]
    }
}




pub struct PhysicsEngine<'a>{

    objects: Vec<PhysicsObject<'a>>,
    grad: Gradient,
   // group_interactions:HashMap<&'a str , Box<dyn Fn(Point,Point)->Point>>,
    dt: f64,
}


impl PhysicsEngine<'_>{
    pub fn populate(&mut self, pos: [f64; 2], vel:[f64; 2], mass: f64, radius: i32){
        let mut accel = [0.0,0.0];
        let mut naccel = [0.0,0.0];
        let mut npos = [0.0,0.0];
        let mut po = PhysicsObject{position: pos, veclocity: vel, mass:mass, radius:radius, acceleration:accel, nacceleration:naccel, nposition: npos, group:"A"};
        self.objects.push(po);
    }

    fn calculate_naccel(&mut self){
        for index in 0..self.objects.len(){
            let nx = self.objects[index].nposition[0];
            let ny = self.objects[index].nposition[1];

            self.objects[index].nacceleration[0] = 0.0;
            self.objects[index].nacceleration[1] = 0.0;

            for tar in 0..self.objects.len(){
                if tar == index{
                    continue;
                }
                let ox = self.objects[tar].nposition[0];
                let oy = self.objects[tar].nposition[1];

                self.objects[index].nacceleration[0] += -(nx-ox)/((nx-ox)*(nx-ox) + 10.0).sqrt();
                self.objects[index].nacceleration[1] += -(ny-oy)/((ny-oy)*(ny-oy) + 10.0).sqrt();
            }

            //println!("{} {}",self.objects[index].nacceleration[0], self.objects[index].nacceleration[1]);
        }
    }


    pub fn get_positions(&mut self) -> &Vec<PhysicsObject>{
        return &self.objects;
    }


    pub fn propagate(&mut self){

        for element in &mut self.objects{
            let x = element.position[0] + element.veclocity[0] * self.dt + 0.5 * self.dt * self.dt * element.acceleration[0];
            let y = element.position[1] + element.veclocity[1] * self.dt + 0.5 * self.dt * self.dt * element.acceleration[1];
            
            element.nposition[0] = x;
            element.nposition[1] = y;
        }

        self.calculate_naccel();
        for element in &mut self.objects{
            let vx = element.veclocity[0] + self.dt * 0.5 * (element.acceleration[0] + element.nacceleration[0]);
            let vy = element.veclocity[1] + self.dt * 0.5 * (element.acceleration[1] + element.nacceleration[1]);
            element.acceleration[0] = element.nacceleration[0];
            element.acceleration[1] = element.nacceleration[1];
            element.veclocity = [vx,vy];
            element.position[0] = element.nposition[0];
            element.position[1] = element.nposition[1];
        }
        

    }

    pub fn new(grad:Gradient, dt:f64 ) -> PhysicsEngine<'static>{
        let mut vec = Vec::<PhysicsObject>::new();
        let mut pe = PhysicsEngine{objects:vec,
            grad:grad,
            dt:dt};

        return pe;
    }
}