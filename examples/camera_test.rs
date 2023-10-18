extern crate rand;
extern crate grid;

use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "EvoCraft -> Camera Test".to_owned(),
        fullscreen: false,
        window_height: 1080,
        window_width: 1920,
        window_resizable: false,
        ..Default::default()
    }
}

/// Draws a tile from a tileset
pub fn draw_tile(texture: &Texture2D, source: Rect, dest: Rect) {

    draw_texture_ex(
        texture,
        dest.x,
        dest.y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(dest.w, dest.h)),
            source: Some(source),
            ..Default::default()
        },
    );
}

pub fn asset_tile(x: usize, y: usize, size: f32) -> Rect {
    return Rect::new(x as f32 * size, y as f32 * size, size, size);
}


#[macroquad::main(window_conf)]
async fn main() {

    let aspect_ratio = screen_height() / screen_width();
    let mut zoom_factor = 0.05;

    // let tileset = load_image("assets/dungeon_crawl.png").await.unwrap();
    // let tile_size = 32.;

    // let chest = tileset.sub_image(Rect::new(6.0, 0.0, 32.0, 32.0));
    // chest.export_png("assets/chest.png");

    let tileset = load_texture("assets/dungeon_crawl.png").await.unwrap();
    let tile_size = 32.;

    let chest_rect = asset_tile(6, 0, tile_size);
    let eyeball_rect = asset_tile(50, 67, tile_size);
    let grass_rect = asset_tile(11, 9, tile_size);

    let hero_rect = asset_tile(7, 80, tile_size);

    build_textures_atlas();

    //vec2(0.005 * aspect_ratio, 0.005),

    let mut px = 0.0;
    let mut py = 0.0;

    let mut x = 1.0;
    let mut y = 1.0;
    let mut s = 1.0;
    let animation_speed = 10.0;
    let movement_speed = 5.0;

    let camera_offset = vec2(0.0, 0.0);

    loop {
        let fps = get_fps();
        let frame_time = get_frame_time();
        let elapsed = get_time();

        // get scroll wheel delta
        let scroll = mouse_wheel().1;
        zoom_factor += scroll * 0.01;
        zoom_factor = clamp(zoom_factor, 0.05, 0.1);
    
        clear_background(BLACK);

        set_camera(&Camera2D {
            zoom: vec2(zoom_factor * aspect_ratio, zoom_factor),
            target: vec2(px, py),
            ..Default::default()
        });

        for i in -10..10 {
            for j in -10..10 {
                draw_circle(i as f32, j as f32, 0.05, Color::new(0.0, 1.0, 0.0, 1.0));
                draw_tile(&tileset, grass_rect, Rect::new(i as f32, j as f32, 1., 1.));
            }
        }


        // pulse size from center of texture using animation_speed
        //s = (elapsed as f32 * animation_speed).sin() * 0.1 + 1.0;
        y = (elapsed as f32 * animation_speed).sin() * 0.1 + 1.0 * frame_time;
        x += 2.0 * frame_time;

        draw_tile(&tileset, eyeball_rect, Rect::new(x, y, s, s));

        if is_key_down(KeyCode::W) {
            py -= movement_speed * frame_time;
        }
        if is_key_down(KeyCode::S) {
            py += movement_speed * frame_time;
        }
        if is_key_down(KeyCode::A) {
            px -= movement_speed * frame_time;
        }
        if is_key_down(KeyCode::D) {
            px += movement_speed * frame_time;
        }

        draw_tile(&tileset, hero_rect, Rect::new(px, py, 1., 1.));

        let gx = px.round();
        let gy = py.round();

        draw_circle(gx + 0.5, gy + 0.5, 0.05, Color::new(1.0, 0.0, 0.0, 1.0));

        let cx = px + 0.5;
        let cy = py + 0.5;
        draw_circle(cx, cy, 0.05, Color::new(1.0, 0.0, 0.0, 1.0));

        set_default_camera();

        // draw fps
        draw_text(&format!("fps: {}", fps), 10.0, 20.0, 20.0, WHITE);

        draw_text(&format!("grid: {}, {}", gx, gy), 10.0, 40.0, 20.0, WHITE);

        next_frame().await;
    }
}