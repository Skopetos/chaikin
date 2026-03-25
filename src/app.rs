use crate::algorithm::build_steps;
use crate::ui;
use macroquad::prelude::*;

const MAX_STEPS: usize = 7;
const STEP_DURATION: f64 = 0.8;

pub struct App {
    control_points: Vec<Vec2>,
    steps: Vec<Vec<Vec2>>,
    is_animating: bool,
    current_step: usize,
    last_step_time: f64,
    show_no_points_message: bool,
    no_points_message_time: f64,
}

impl App {
    pub fn new() -> Self {
        Self {
            control_points: Vec::new(),
            steps: Vec::new(),
            is_animating: false,
            current_step: 0,
            last_step_time: 0.0,
            show_no_points_message: false,
            no_points_message_time: 0.0,
        }
    }

    // returns true if the app should quit
    pub fn handle_input(&mut self) -> bool {
        if is_key_pressed(KeyCode::Escape) {
            return true;
        }
//reset
        if is_key_pressed(KeyCode::C) {
        self.control_points.clear();
        self.steps.clear();
        self.is_animating = false;
        self.current_step = 0;
       }

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            self.control_points.push(vec2(mx, my));

            // reset animation whenever a new point is added
            self.is_animating = false;
            self.current_step = 0;
            self.steps.clear();
        }

        if is_key_pressed(KeyCode::Enter) {
            match self.control_points.len() {
                0 => {
                    self.show_no_points_message = true;
                    self.no_points_message_time = get_time();
                }
                1 | 2 => {
                    self.is_animating = false;
                    self.current_step = 0;
                    self.steps.clear();
                }
                _ => {
                    self.steps = build_steps(&self.control_points, MAX_STEPS);
                    self.is_animating = true;
                    self.current_step = 0;
                    self.last_step_time = get_time();
                }
            }
        }

        false
    }

    pub fn update(&mut self) {
        if self.show_no_points_message && get_time() - self.no_points_message_time > 1.5 {
            self.show_no_points_message = false;
        }

        if self.is_animating && !self.steps.is_empty() {
            let now = get_time();

            if now - self.last_step_time >= STEP_DURATION {
                self.current_step += 1;

                if self.current_step > MAX_STEPS {
                    self.current_step = 0;
                }

                self.last_step_time = now;
            }
        }
    }

    pub fn draw(&self) {
        let shown_points = self.points_to_draw();

        // draw current shape
        match shown_points.len() {
            0 => {}
            1 => {
                ui::draw_small_point(shown_points[0], BLUE);
            }
            2 => {
                ui::draw_polyline(shown_points, 2.0, BLUE);
                ui::draw_points_as_circles(shown_points, 4.0, 1.5, BLUE);
            }
            _ => {
                ui::draw_polyline(shown_points, 2.0, BLUE);
            }
        }

        // always show original control points
        ui::draw_points_as_circles(&self.control_points, 4.0, 1.5, BLACK);

        // optional helper lines between control points
        //if self.control_points.len() >= 2 {
         //   ui::draw_polyline(&self.control_points, 1.0, GRAY);
       // }

        ui::draw_instructions(
            self.control_points.len(),
            self.current_step,
            self.is_animating,
        );

        if self.show_no_points_message {
            ui::draw_message("Draw at least one point first");
        }
    }

    fn points_to_draw(&self) -> &[Vec2] {
        if self.steps.is_empty() {
            &self.control_points
        } else {
            &self.steps[self.current_step]
        }
    }
}