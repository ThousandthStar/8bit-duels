use bevy::prelude::*;

pub fn screen_to_world_position(
    screen_pos: Vec2,
    camera: &Camera,
    camera_transform: &GlobalTransform,
    window: &Window,
) -> Vec2 {
    //***********************************************************************/
    //Found on the unofficial Bevy cheat book (https://bevy-cheatbook.github.io/cookbook/cursor2world.html)
    let window_size = Vec2::new(window.width() as f32, window.height() as f32);

    // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
    let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

    // matrix for undoing the projection and camera transform
    let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

    // use it to convert ndc to world-space coordinates
    let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

    // reduce it to a 2D value
    let world_pos: Vec2 = world_pos.truncate();
    //***********************************************************************/
    return world_pos;
}

// found here: https://answers.unity.com/questions/414829/any-one-know-maths-behind-this-movetowards-functio.html
// and adapted for Rust and Bevy
pub fn move_towards(current: Vec2, target: Vec2, speed: f32, _delta_time: f32) -> Vec2 {
    let d = target - current;
    let magnitude = (d.x.powi(2) + d.y.powi(2)).sqrt();
    if magnitude <= 10.0 || magnitude == 0.0 {
        return target;
    }
    return current + d / magnitude * speed;
}
