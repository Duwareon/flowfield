use std::f32::consts::PI;
use std::time::Instant;

use macroquad::prelude::*;


fn rotate_around(cx: f32, cy: f32, angle: f32, point: Vec2) -> Vec2 {
    let mut newpoint = point;

    newpoint.x -= cx;
    newpoint.y -= cy;

    let newx = newpoint.x * angle.cos() - newpoint.y * angle.sin();
    let newy = newpoint.x * angle.sin() + newpoint.y * angle.cos();

    newpoint.x = newx + cx;
    newpoint.y = newy + cy;

    newpoint
}

fn draw_arrow(x1: f32, y1: f32, x2: f32, y2: f32, thickness: f32,  arrowangle: f32, arrowlength: f32, color: Color){
    let deltax = x2-x1;
    let deltay = y2-y1;
    let slope = deltay/deltax;

    let endpoint = Vec2::new(x2, y2);
    
    let baseoffset: Vec2;
    if deltax > 0.0 {
        baseoffset = Vec2::new(1.0, slope);
    }
    else {
        baseoffset = Vec2::new(-1.0, -slope)
    }

    let normbaseoffset = baseoffset.normalize();
    let mulbaseoffset = normbaseoffset*arrowlength;
    
    let basepoint = endpoint - mulbaseoffset;

    
    let end1: Vec2 = rotate_around(x2, y2, arrowangle,basepoint);
    let end2: Vec2 = rotate_around(x2, y2, -arrowangle, basepoint);

    draw_line(x1, y1, x2, y2, thickness, color);
    draw_line(x2, y2, end1.x, end1.y, thickness, color);
    draw_line(x2, y2, end2.x, end2.y, thickness, color);
}

#[allow(dead_code)]
fn draw_arrow_angle(x: f32, y: f32, angle: f32, length: f32, thickness: f32, arrowangle: f32, arrowlength: f32, color: Color) {
    let newpoint = rotate_around(x, y, angle, Vec2::new(x+length, y));
    draw_arrow(x, y, newpoint.x, newpoint.y, thickness, arrowangle, arrowlength, color);
}

fn draw_line_vec(p1: Vec2, p2: Vec2, thickness: f32, color: Color) {
    draw_line(p1.x, p1.y, p2.x, p2.y, thickness, color)
}

fn draw_flow_line(p1: Vec2, f: &dyn Fn(Vec2) -> Vec2, length: usize, seglength: f32, thickness: f32, color: Color) -> Vec2 {
    //let mut vel = startvel;
    let mut pos = p1;
    for i in 0..length {
        let vel = f(pos)*seglength;
        let pos2 = pos+vel;
        draw_line_vec(pos, pos2, thickness, color);
        pos = pos2;
    }
    f(p1)
}

#[macroquad::main("Poggers")]
async fn main() {
    let mut midheight: f32;
    let mut midwidth: f32;
    //let f = |p1: Vec2| -> Vec2 {Vec2::new((p1.x/300.0).sin(), (p1.y/300.0).cos())};
    let f = |p1: Vec2| -> Vec2 {
        let scale: f32 = 0.01;
        let sp1 = (p1 - Vec2::new(200.0, 200.0))*scale;
        Vec2::from_angle(
            sp1.x.cos() / sp1.y.cos()
        ).normalize()
    };

    let mut time: usize = 0;

    let linesqsize: usize = 1;
    let linesqgap: f32 = 100.0;

    let arrowsqgap: f32 = 15.0;
    let arrowlength: f32 = 1.0;
    let drawarrows = true;
    loop {
        //let start = Instant::now();
        let height = screen_height();
        let width = screen_width();
        let midheight = height/2.0;
        let midwidth = width/2.0;
    	clear_background(WHITE);

        if drawarrows {
            let arrowsqwidth: usize = (width/arrowsqgap) as usize;
            let arrowsqheight: usize = (height/arrowsqgap) as usize;
            for i in 0..arrowsqwidth {
                for j in 0..arrowsqheight {
                    let x = i as f32 * arrowsqgap;
                    let y = j as f32 * arrowsqgap;
                    let p1 = Vec2::new(x, y);
                    let p2 = p1+(f(p1)*arrowlength);
                    draw_arrow(p1.x, p1.y, p2.x, p2.y, 1.0, PI/8.0, 10.0, BLACK);
                }
            }
        }

        let (x, y) = mouse_position();
        draw_flow_line(Vec2::new(x, y), &f, 5000, 0.5,4.0, RED);
        //let now = Instant::now();
        //draw_text(now.duration_since(start).as_millis().to_string().as_str(), 100.0, 100.0, 50.0, BLACK);
        next_frame().await;
        //time+=1;
    }
}
