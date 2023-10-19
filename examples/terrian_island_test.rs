extern crate rand;
extern crate grid;
extern crate rayon;

use rand::thread_rng;
use rand::seq::SliceRandom;
use::rand::Rng;

use rayon::prelude::*;
use macroquad::prelude::*;
use perlin2d::PerlinNoise2D;

fn window_conf() -> Conf {
    Conf {
        window_title: "EvoCraft -> Terrian Island Test".to_owned(),
        fullscreen: false,
        window_height: 1080,
        window_width: 1920,
        window_resizable: false,
        ..Default::default()
    }
}


fn generate_island(range: isize, gradient: &Vec<f32>) -> Vec<(Vec2, f64, Color)> {
    let seed = rand::random::<i16>();

    //let mut grid = Vec::with_capacity((range*range) as usize);

    //let mut index = 0;

    let grid = (0..range)
        .into_par_iter()
        .map(|x| {
            let perlin = PerlinNoise2D::new(
                5, 1.0, 1.0, 1.00, 2.0, (100.0, 100.0), 1.0, seed.abs() as i32
            );

            let mut grid = Vec::with_capacity((range*2) as usize);

            for y in 0..range {
                let mut noise = perlin.get_noise(x as f64, y as f64);
    
                // normalize to 0 to 1
                noise = (noise + 1.0) / 2.0;

                //y * columns + x;
                let index = y * range + x;
    
                // get gradient at index using short hand index
                let g = gradient[index as usize];
                noise -= g as f64;
    
                let mut color: Color = BLUE;
    
                if noise < 0.1 {
                    color = BLUE;
                } else if noise <= 0.2 {
                    color = YELLOW;
                } else if noise <= 0.7 {
                    color = GREEN;
                } else if noise <= 0.9 {
                    color = BROWN;
                } else if noise <= 0.99999 {
                    color = GRAY;
                } else {
                    color = WHITE;
                }
    
                grid.push((Vec2::new(x as f32, y as f32), noise, color));
                //grid.push((Vec2::new(x as f32, y as f32), Color::new(noise as f32, noise as f32, noise as f32, 1.0)));
                //grid.push((Vec2::new(x as f32, y as f32), noise, Color::new(g as f32, g as f32, g as f32, 1.0)));
            }

            return grid;
        })
        .flat_map(|x| x)
        .collect::<Vec<(Vec2, f64, Color)>>();

    return grid;
}


fn generate_gradient(range: isize, padding: f32, power: f32) -> Vec<f32> {
    let mut gradient = Vec::new();

    let range_perc = range as f32 * padding;

    let c = (range / 2) as f32;

    for x in 0..range {
        for y in 0..range {
            let x1 = x as f32;
            let y1 = y as f32;
            
            // d is distance from c,c to x1, x2
            let d = ((x1 - c).powf(2.0) + (y1 - c).powf(2.0)).sqrt();

            // v is d / range at an exponential rate
            let v = (d / range_perc).powf(power);

            gradient.push(v);
        }
    }

    return gradient;
}


fn random_on_land(grid: &Vec<(Vec2, f64, Color)>, points: usize) -> Option<Vec<Vec2>> {
    let mut indicies = (0..grid.len()).collect::<Vec<usize>>();
    indicies.shuffle(&mut thread_rng());

    let mut found = Vec::with_capacity(points);

    for i in indicies {
        let (pos, noise, color) = grid[i];
        if noise > 0.1 {
            found.push(pos);
            if found.len() >= points {
                return Some(found);
            }
        }
    }

    return None;
}


#[macroquad::main(window_conf)]
async fn main() {

    let aspect_ratio = screen_height() / screen_width();
    let mut zoom_factor = 0.005;

    let range = 500;
    let padding = 0.3;
    let power = 1.5;

    let gradient = generate_gradient(range, padding, power);

    let mut grid = generate_island(range, &gradient);
    let mut points = random_on_land(&grid, 2).unwrap();
    let mut land1 = points.remove(0);
    let mut land2 = points.remove(0);

    let mut start_time = get_time();

    let mut p = Vec2::new(range as f32 / 2.0, range as f32 / 2.0);
    let speed = 1.0;

    loop {
        let frame_time = get_frame_time();

        if is_key_down(KeyCode::W) {
            p.y -= speed * frame_time;
        } else if is_key_down(KeyCode::S) {
            p.y += speed * frame_time;
        } else if is_key_down(KeyCode::A) {
            p.x -= speed * frame_time;
        } else if is_key_down(KeyCode::D) {
            p.x += speed * frame_time;
        }

        let camera = &Camera2D {
            zoom: vec2(zoom_factor * aspect_ratio, zoom_factor),
            target: p,
            ..Default::default()
        };

        let vp1 = camera.screen_to_world(Vec2::new(0.0, 0.0));
        let vp2 = camera.screen_to_world(Vec2::new(screen_width(), screen_height()));
        let viewport = Rect::new(
            vp1.x.floor(),
            vp1.y.floor(),
            (vp2.x - vp1.x).ceil(),
            (vp2.y - vp1.y).ceil()
        );

        set_camera(camera);

        // get scroll wheel delta
        let scroll = mouse_wheel().1;
        zoom_factor += scroll * 0.01;
        zoom_factor = clamp(zoom_factor, 0.005, 0.1);
    
        clear_background(BLACK);

        // for x y in viewport
        // for x in (viewport.x as isize)..((viewport.x + viewport.w) as isize) {
        //     for y in (viewport.y as isize)..((viewport.y + viewport.h) as isize) {

        //         // for (pos, noise, color) in &grid {
        //         //     if pos.x as i32 == x && pos.y as i32 == y {
        //         //         color = *color;
        //         //         noise = *noise;
        //         //     }
        //         // }

        //         let index = ((x + range) * (range * 2) + (y + range)) as usize;
        //         let (pos, noise, color) = &grid[index];
                
        //         draw_rectangle(pos.x, pos.y, 1.0, 1.0, *color);

        //         // color = c;
        //         // noise = n;

        //         // if noise > 0.1 {
        //         //     draw_rectangle(x as f32, y as f32, 1.0, 1.0, color);
        //         // }
        //     }
        // }
        
        for (pos, noise, color) in &grid {
            if viewport.contains(*pos) {
                draw_rectangle(pos.x, pos.y, 1.0, 1.0, *color);
            }
        }

        draw_rectangle(p.x, p.y, 1.0, 1.0, RED);

        draw_rectangle(land1.x, land1.y, 5.0, 5.0, RED);
        draw_rectangle(land2.x, land2.y, 5.0, 5.0, RED);

        draw_rectangle_lines(0.0, 0.0, range as f32, range as f32, 2.0, WHITE);

        //if elapsed time is greater than 5 seconds
        // if get_time() - start_time > 5.0 {
        //     start_time = get_time();
        //     grid = generate_island(range, &gradient);
        //     points = random_on_land(&grid, 2).unwrap();
        //     land1 = points.remove(0);
        //     land2 = points.remove(0);
        // }

        next_frame().await;
    }
}