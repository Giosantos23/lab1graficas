extern crate nalgebra_glm as glm;

use glm::Vec3;

mod framebuffer;
mod bmp;
mod line;
mod polygon;

use crate::bmp::WriteBmp;
use crate::framebuffer::{Framebuffer, Color}; 
use line::Line;
use crate::polygon::Polygon;


fn main() {
    let mut framebuffer = Framebuffer::new(800, 600);

    framebuffer.set_background_color(Color(0xFFFFFF));
    framebuffer.clear();
    
    framebuffer.set_current_color(Color(0x000000));


    // Primer polígono
    let points1 = vec![
        Vec3::new(165.0, 380.0, 0.0), Vec3::new(185.0, 360.0, 0.0), Vec3::new(180.0, 330.0, 0.0),
        Vec3::new(207.0, 345.0, 0.0), Vec3::new(233.0, 330.0, 0.0), Vec3::new(230.0, 360.0, 0.0),
        Vec3::new(250.0, 380.0, 0.0), Vec3::new(220.0, 385.0, 0.0), Vec3::new(205.0, 410.0, 0.0),
        Vec3::new(193.0, 383.0, 0.0)
    ];

    framebuffer.draw_polygon(&points1);

    // Segundo polígono
    let points2 = vec![
        Vec3::new(321.0, 335.0, 0.0), Vec3::new(288.0, 286.0, 0.0), Vec3::new(339.0, 251.0, 0.0),
        Vec3::new(374.0, 302.0, 0.0)
    ];

    framebuffer.draw_polygon(&points2);

    //poligono 3
    let points3 = vec![
        Vec3::new(377.0, 249.0, 0.0), Vec3::new(411.0, 197.0, 0.0), Vec3::new(436.0, 249.0, 0.0)
    ];

    framebuffer.draw_polygon(&points3);

    
    framebuffer.render_buffer("out.bmp");
        
    println!("Render de polígonos");

}

