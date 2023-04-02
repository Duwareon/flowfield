use macroquad::prelude::*;
use std::f32::consts::PI;
use std::time::Instant;

//enum Expression {
    //Empty,
    //Num(f32),
    //Add(Box<Expression>, Box<Expression>),
    //Subtract(Box<Expression>, Box<Expression>),
    //Multiply(Box<Expression>, Box<Expression>),
    //Divide(Box<Expression>, Box<Expression>),
    //Sin(Box<Expression>),
    //Cos(Box<Expression>),
    //X,
    //Y,
//}



#[derive(Debug, Clone, Copy)]
struct Line {
    p1: Vec2,
    p2: Vec2,
    thickness: f32,
    color: Color,
}

#[derive(Debug, Clone, Copy)]
struct Arrow {
    line: Line,
    angle: f32,
    length: f32,
}

trait Drawable {
    fn draw(self);
}

impl Drawable for Line {
    fn draw(self) {
        draw_line(
            self.p1.x,
            self.p1.y,
            self.p2.x,
            self.p2.y,
            self.thickness,
            self.color,
        )
    }
}

impl Drawable for Arrow {
    fn draw(self) {
        let deltax = self.line.p2.x - self.line.p1.x;
        let deltay = self.line.p2.y - self.line.p1.y;
        let slope = deltay / deltax;

        let endpoint = Vec2::new(self.line.p2.x, self.line.p2.y);

        let baseoffset: Vec2;
        if deltax > 0.0 {
            baseoffset = Vec2::new(1.0, slope);
        } else {
            baseoffset = Vec2::new(-1.0, -slope)
        }

        let normbaseoffset = baseoffset.normalize();
        let mulbaseoffset = normbaseoffset * self.length;

        let basepoint = endpoint - mulbaseoffset;

        let end1: Vec2 = rotate_around(self.line.p2, self.angle, basepoint);
        let end2: Vec2 = rotate_around(self.line.p2, -self.angle, basepoint);

        self.line.draw();
        draw_line(
            self.line.p2.x,
            self.line.p2.y,
            end1.x,
            end1.y,
            self.line.thickness,
            self.line.color,
        );
        draw_line(
            self.line.p2.x,
            self.line.p2.y,
            end2.x,
            end2.y,
            self.line.thickness,
            self.line.color,
        );
    }
}

impl Arrow {
    #[allow(dead_code)]
    fn new_arrow_from_angle(
        p1: Vec2,
        angle: f32,
        length: f32,
        thickness: f32,
        arrowangle: f32,
        arrowlength: f32,
        color: Color,
    ) -> Arrow {
        let newpoint = rotate_around(p1, angle, Vec2::new(p1.x + length, p1.y));
        Arrow {
            line: Line {
                p1,
                p2: Vec2::new(newpoint.x, newpoint.y),
                thickness,
                color,
            },
            angle: arrowangle,
            length: arrowlength,
        }
    }
}

fn rotate_around(cvec: Vec2, angle: f32, point: Vec2) -> Vec2 {
    let mut newpoint = point;

    newpoint.x -= cvec.x;
    newpoint.y -= cvec.y;

    let newx = newpoint.x * angle.cos() - newpoint.y * angle.sin();
    let newy = newpoint.x * angle.sin() + newpoint.y * angle.cos();

    newpoint.x = newx + cvec.x;
    newpoint.y = newy + cvec.y;

    newpoint
}

fn draw_line_vec(p1: Vec2, p2: Vec2, thickness: f32, color: Color) {
    draw_line(p1.x, p1.y, p2.x, p2.y, thickness, color)
}

fn draw_flow_line(
    p1: Vec2,
    map: &dyn Fn(Vec2) -> Vec2,
    length: usize,
    seglength: f32,
    thickness: f32,
    color: Color,
) -> Vec2 {
    //let mut vel = startvel;
    let mut pos = p1;
    for _ in 0..length {
        let vel = map(pos) * seglength;
        let pos2 = pos + vel;
        draw_line_vec(pos, pos2, thickness, color);
        pos = pos2;
    }
    pos
}

fn write_text(text: &str, p1: Vec2, size: u16, font: Font, color: Color) {
    let params = TextParams {
        font,
        font_size: size,
        font_scale: 1.0,
        font_scale_aspect: 1.0,
        rotation: 0.0,
        color,
    };
    draw_text_ex(text, p1.x, p1.y, params)
}

//fn parse_functext(functext: &str) -> Expression {
    //let mut ast: Expression = Expression::Empty;

    //let mut stream: String = functext.to_string();
    //stream = stream.chars().rev().collect::<String>();

    //let mut match_ctr: usize = 0;

    //loop {
        //let nextchar = stream.pop();
        //if nextchar.is_none() {break};
        //match nextchar.unwrap() {
            //'(' => {
                //let mut content: Vec<char> = vec![];
                //match_ctr = 1;
                //while match_ctr != 0 {

                    //let symbol = stream.pop().expect("Unmatched parentheses");
                    //if symbol == '(' {
                        //match_ctr += 1
                    //}
                    //else if symbol == ')' {
                        //match_ctr -= 1
                    //}
                    //if match_ctr != 0 {
                        //content.append(&mut vec![symbol])
                    //}
                //}
                //ast.insert(content);

            //}
            //_ => {}
        //}
    //}


    //ast
//}

#[macroquad::main("Poggers")]
async fn main() {
    let letterkeys = [
        KeyCode::A,
        KeyCode::B,
        KeyCode::C,
        KeyCode::D,
        KeyCode::E,
        KeyCode::F,
        KeyCode::G,
        KeyCode::H,
        KeyCode::I,
        KeyCode::J,
        KeyCode::K,
        KeyCode::L,
        KeyCode::M,
        KeyCode::N,
        KeyCode::O,
        KeyCode::P,
        KeyCode::Q,
        KeyCode::R,
        KeyCode::S,
        KeyCode::T,
        KeyCode::U,
        KeyCode::V,
        KeyCode::W,
        KeyCode::X,
        KeyCode::Y,
        KeyCode::Z,
        KeyCode::Key0,
        KeyCode::Key1,
        KeyCode::Key2,
        KeyCode::Key3,
        KeyCode::Key4,
        KeyCode::Key5,
        KeyCode::Key6,
        KeyCode::Key7,
        KeyCode::Key8,
        KeyCode::Key9,
        KeyCode::Space,
        KeyCode::Slash,
    ];
    let mut midheight: f32;
    let mut midwidth: f32;
    //let f = |p1: Vec2| -> Vec2 {Vec2::new((p1.x/300.0).sin(), (p1.y/300.0).cos())};
    let mut map = |p1: Vec2| -> Vec2 {
        let scale: f32 = 0.01;
        let sp1 = (p1 - Vec2::new(200.0, 200.0)) * scale;
        //Vec2::from_angle(sp1.x + sp1.y).normalize()
        Vec2::new(sp1.x.tan(), sp1.y.tan())
    };

    let font: Font = load_ttf_font("/usr/share/fonts/vollkorn/Vollkorn-Medium.ttf")
        .await
        .expect("Could not find font");

    let mut time: usize = 0;

    let sqgap: f32 = 15.0;
    let arrowlength: f32 = 1.0;
    let drawarrows = true;
    let mut functext = "(/ (sin x) (cos y))".to_string();
    let mut cursor: usize = functext.len();
    loop {
        let start = Instant::now();
        let height = screen_height();
        let width = screen_width();
        clear_background(WHITE);

        // Get input and edit text
        let pressed = get_last_key_pressed();

        if pressed.is_some() {
            let key = pressed.unwrap();
            if key == KeyCode::Backspace {
                if cursor == functext.len() {
                    functext.pop();
                } else {
                    functext.remove(cursor);
                }
                if cursor != 0 {
                    cursor -= 1;
                }
            } else if letterkeys.contains(&key) {
                let button = get_char_pressed();
                if button.is_some() {
                    functext.insert(cursor, button.unwrap());
                    cursor += 1;
                }
            };
        };

        // Update mapping function
        //let newmap = parse_functext(functext);



        // Draw arrows if wanted
        if drawarrows {
            let sqwidth: usize = (width / sqgap) as usize;
            let sqheight: usize = (height / sqgap) as usize;
            for i in 0..sqwidth {
                for j in 0..sqheight {
                    let x = i as f32 * sqgap;
                    let y = j as f32 * sqgap;
                    let p1 = Vec2::new(x, y);
                    let p2 = p1 + (map(p1) * arrowlength);
                    Arrow {
                        line: Line {
                            p1,
                            p2,
                            thickness: 1.0,
                            color: BLACK,
                        },
                        angle: PI / 8f32,
                        length: 10.0,
                    }
                    .draw()
                }
            }
        }

        // Draw flow line
        let (x, y) = mouse_position();
        draw_flow_line(Vec2::new(x, y), &map, 5000, 0.5, 4.0, RED);

        // Draw function text
        write_text(functext.as_str(), Vec2::new(100.0, 100.0), 50, font, BLACK);

        // Draw frame time / performance measure
        let now = Instant::now();
        write_text(
            now.duration_since(start).as_millis().to_string().as_str(),
            Vec2::new(100.0, 150.0),
            50,
            font,
            BLACK,
        );
        time += 1;

        next_frame().await;
    }
}
