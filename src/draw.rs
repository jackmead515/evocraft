use macroquad::prelude::*;

use crate::creature;
use crate::models::*;
use crate::consts;

fn draw_debug_grid(width: i32, height: i32, grid_size: i32) {
    let grid_color = Color::new(1.0, 1.0, 1.0, 0.1);

    for i in 0..(width / grid_size) {
        let t = (i * grid_size) as f32;
        draw_line(t, 0.0, t, height as f32, 1.0, grid_color);
    }

    for i in 0..(height / grid_size) {
        let t = (i * grid_size) as f32;
        draw_line(0.0, t, width as f32, t, 1.0, grid_color);
    }
}

pub fn draw(game_state: &GameState) {
    clear_background(BLACK);
    draw_debug_grid(consts::SCREEN_WIDTH, consts::SCREEN_HEIGHT, consts::GRID_SIZE);

    let player = &game_state.player;
    draw_text_ex(player.text, player.position.x, player.position.y, TextParams {
        font: Some(&game_state.font),
        font_size: 20,
        color: player.color,
        ..Default::default()
    });

    let creatures = &game_state.creatures;
    for creature in creatures.iter() {
        draw_text_ex(creature.text, creature.position.x, creature.position.y, TextParams {
            font: Some(&game_state.font),
            font_size: 20,
            ..Default::default()
        });
    }

    // snap to grid
    let (mx, my) = mouse_position();
    let (gx, gy) = consts::grid_pos(mx, my);
    let (wx, wy) = consts::world_pos(gx, gy);

    // draw grid position
    draw_rectangle(wx as f32, wy as f32, consts::GRID_SIZE as f32, consts::GRID_SIZE as f32, Color { r: 0.1, g: 1.0, b: 0.2, a: 0.2 });
    draw_rectangle_lines(wx as f32, wy as f32, consts::GRID_SIZE as f32, consts::GRID_SIZE as f32, 1.0, WHITE);

    // draw fps
    draw_text(&format!("FPS: {}", game_state.stats.fps), 10.0, 20.0, 20.0, WHITE);
    draw_text(&format!("MP: {:?}", mouse_position()), 10.0, 40.0, 20.0, WHITE);

    // draw player health and energy
    draw_text(&format!("HP: {}", player.health.current), 10.0, 60.0, 20.0, WHITE);
    draw_text(&format!("EP: {}", player.energy.current), 10.0, 80.0, 20.0, WHITE);
}

