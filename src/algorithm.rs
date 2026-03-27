use macroquad::prelude::*;

// Builds step 0 through step max_steps.
// Step 0 is the original control polygon.
pub fn build_steps(control_points: &[Vec2], max_steps: usize) -> Vec<Vec<Vec2>> {
    let mut steps = Vec::new();
    steps.push(control_points.to_vec());

    let mut current = control_points.to_vec();

    for _ in 0..max_steps {
        current = chaikin_step(&current);
        steps.push(current.clone());
    }

    steps
}

// Applies one iteration of the Chaikin algorithm.
// For each pair of adjacent points, creates two new points:
// - One at 1/4 of the distance from the first point
// - One at 3/4 of the distance from the first point
// Preserves the first and last points to keep the curveδ anchored
fn chaikin_step(points: &[Vec2]) -> Vec<Vec2> {
    // Need at least 2 points to create a line segment
    if points.len() < 2 {
        return points.to_vec();
    }

    let mut new_points = Vec::new();

    // Always keep the first point
    new_points.push(points[0]);

    // For each segment between adjacent points
    for segment_index in 0..points.len() - 1 {
        let start_point = points[segment_index];
        let end_point = points[segment_index + 1];

        // First cut point at 1/4 of the way from start to end
        let quarter_point = start_point * 0.75 + end_point * 0.25;

        // Second cut point at 3/4 of the way from start to end
        let three_quarter_point = start_point * 0.25 + end_point * 0.75;

        new_points.push(quarter_point);
        new_points.push(three_quarter_point);
    }

    // Always keep the last point
    new_points.push(points[points.len() - 1]);

    new_points
}
