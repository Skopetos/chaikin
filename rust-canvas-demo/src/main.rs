use macroquad::prelude::*;

#[macroquad::main("Quick Canvas Demo")]
async fn main() {
    let mut points: Vec<Vec2> = Vec::new();

    loop {
        clear_background(WHITE);

        if is_mouse_button_pressed(MouseButton::Left) {
            let (x, y) = mouse_position();
            points.push(vec2(x, y));
        }

        if is_key_pressed(KeyCode::C) {
            points.clear();
        }

        for i in 0..points.len().saturating_sub(1) {
            let a = points[i];
            let b = points[i + 1];
            draw_line(a.x, a.y, b.x, b.y, 2.0, DARKGRAY);
        }

        for point in &points {
            draw_circle(point.x, point.y, 5.0, RED);
        }

        draw_text("Left click: add point", 20.0, 30.0, 28.0, BLACK);
        draw_text("C: clear points", 20.0, 60.0, 28.0, BLACK);
        draw_text("Esc: quit", 20.0, 90.0, 28.0, BLACK);

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}
