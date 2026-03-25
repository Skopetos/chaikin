use macroquad::prelude::*;

// Builds step 0 through step max_steps.
// Step 0 is the original control polygon.
pub fn build_steps(control_points: &[Vec2], max_steps: usize) -> Vec<Vec<Vec2>> {
    let mut steps = Vec::new();
    steps.push(control_points.to_vec());

    let mut current = control_points.to_vec();

    for _ in 0..max_steps {
        current = chaikin_step_placeholder(&current);
        steps.push(current.clone());
    }

    steps
}

// Placeholder for now.
fn chaikin_step_placeholder(points: &[Vec2]) -> Vec<Vec2> {
    points.to_vec()
}