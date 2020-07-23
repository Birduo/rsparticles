use piston_window::*;
use rand::Rng;
use noise::*;

const BG:[f32; 4] = [33.0/255.0, 33.0/255.0, 33.0/255.0, 1.0];

struct Particle {
    x: f64,
    y: f64,
    xv: f64,
    yv: f64,
    size: f64,
    color: [f32; 4]
}

fn main() {
    let mut rng = rand::thread_rng();
    let perlin = Perlin::new();
    let mut particles:Vec<Particle> = vec![];
    let mut window:PistonWindow = WindowSettings::new("lab", [852.0, 480.0])
        .exit_on_esc(true)
        .vsync(true)
        .fullscreen(true)
        .build().unwrap();

    let mut count:u8 = 0;
    while let Some(e) = window.next() {
        /*
        match e {
            Input::Button(Key::F11) => window. 
            _ => {}
        };*/

        if count >= 30 {
            particles.push(Particle {
                x: window.size().width * 0.5,
                y: window.size().height * 0.95,
                xv: rng.gen_range(-1.0, 1.0) * 3.0,
                yv: rng.gen_range(-1.0, -0.5) * 6.0,
                size: window.size().height * 0.025, // adaptive: window.size().height * 0.025
                color: [0.5, 0.0, 1.0, 1.0]
                //brights [(rng.gen_range(0.0, 1.0) as f32).round(), (rng.gen_range(0.0, 1.0) as f32)\
                //.round(), (rng.gen_range(0.0, 1.0) as f32).round(), 1.0]
                //random [rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0), 1.0]
            });

            count = 0;
        }
        count += 1;

        particles.retain(|p| !(p.x > window.size().width || p.y > window.size().height || p.x < 0.0 || p.y < 0.0 || p.color[3] <= 0.0));

        window.draw_2d(&e, |c, g, _| {
            clear(BG, g);

            for p in &mut particles {
                p.x += p.xv;
                p.y += p.yv;

                //fade
                /*
                if p.color[3] > 0.0 {
                    p.color[3] -= 4.0/255.0
                }
                */

                //perlin fade
                //print!("{:?} ", p.color[3]);
                p.color[3] = perlin.get([p.x/1000.0, p.y/1000.0]).abs() as f32;
                //println!("{:?}", p.color[3]);
                
                ellipse(p.color, ellipse::centered([p.x, p.y, p.size, p.size]), c.transform, g);
            }
        });
    }
}