use macroquad::prelude::*;

use crate::models::*;
use crate::consts::*;

fn draw_debug_grid(width: u32, height: u32, grid_size: u32) {
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
    draw_debug_grid(SCREEN_WIDTH, SCREEN_HEIGHT, GRID_SIZE);

    let creatures = &game_state.creatures;
    for creature in creatures {
        if let Ok(c) = creature.lock() {
            let x = c.position.x + c.random_offset.0;
            let y = c.position.y + c.random_offset.1;
            draw_text_ex(c.text, x, y, TextParams {
                font: Some(&game_state.font),
                font_size: 20,
                ..Default::default()
            });

            let hw = 10.0 * (c.health.current / c.health.max);
            let ew = 10.0 * (c.energy.current / c.energy.max);
            draw_line(x, y + 2.0, x+hw, y + 2.0, 1.0, Color::new(1.0, 0.0, 0.0, 1.0));
            draw_line(x, y + 3.0, x+ew, y + 3.0, 1.0, Color::new(238.0, 252.0, 45.0, 1.0));
        }
    }

    let player = &game_state.player;
    draw_text_ex(player.text, player.position.x, player.position.y, TextParams {
        font: Some(&game_state.font),
        font_size: 20,
        color: player.color,
        ..Default::default()
    });

    // snap to grid
    let (mx, my) = mouse_position();
    let (gx, gy) = grid_pos(mx, my);
    let (wx, wy) = world_pos(gx, gy);

    // draw grid position
    draw_rectangle(wx as f32, wy as f32, GRID_SIZE as f32, GRID_SIZE as f32, Color { r: 0.1, g: 1.0, b: 0.2, a: 0.3 });

    // draw fps
    draw_text(&format!("FPS: {}", game_state.stats.fps), 10.0, 20.0, 20.0, WHITE);
    draw_text(&format!("MP: {:?}", (mx, my)), 10.0, 40.0, 20.0, WHITE);
    draw_text(&format!("GP: {:?}", (gx, gy)), 10.0, 60.0, 20.0, WHITE);

    // draw player health and energy as rectangles
    draw_rectangle(10.0, 65.0, player.health.current, 20.0, Color::new(1.0, 0.0, 0.0, 0.7));
    draw_rectangle(10.0, 85.0, player.energy.current, 20.0, Color::new(238.0, 252.0, 45.0, 0.7));

    // draw game time
    draw_text(&format!("Time: {}", game_state.stats.elapsed as u64), 10.0, 120.0, 20.0, WHITE);

    // draw total creatures
    draw_text(&format!("Creatures: {}", creatures.len()), 10.0, 140.0, 20.0, WHITE);

    // render ui elements
    // draw_rectangle(60.0, 60.0, 60.0*10.0, 60.0*5.0, BLACK);
    // draw_line(60.0, 60.0, 60.0 + 60.0*10.0, 60.0, 1.0, WHITE);
    // draw_line(60.0, 60.0, 60.0, 60.0 + 60.0*5.0, 1.0, WHITE);
    // draw_line(60.0 + 60.0*10.0, 60.0, 60.0 + 60.0*10.0, 60.0 + 60.0*5.0, 1.0, WHITE);
    // draw_line(60.0, 60.0 + 60.0*5.0, 60.0 + 60.0*10.0, 60.0 + 60.0*5.0, 1.0, WHITE);
    // draw_text(
    //     &format!("Hello! Welcome to EvoCraft. A game of survial and evolution."),
    //     80.0, 80.0, 20.0, WHITE
    // );
}

