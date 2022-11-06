use serde_json::Value;

pub(crate) struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    pub(crate) fn new(x: i32, y: i32) -> Vec2 {
        Vec2 { x, y }
    }

    pub(crate) fn distance(&self, other: Vec2) -> i32 {
        (((self.x as f32) - (other.x as f32)).powf(2.)
            + ((self.y as f32) - (other.y as f32)).powf(2.))
        .sqrt() as i32
    }
}

pub(crate) fn get_targeted_action_positions(packet: Value) -> Option<(f64, f64, f64, f64)> {
    if !matches!(packet["start-x"].clone(), Value::Number(_))
        || !matches!(packet["start-y"].clone(), Value::Number(_))
        || !matches!(packet["end-x"].clone(), Value::Number(_))
        || !matches!(packet["end-y"].clone(), Value::Number(_))
    {
        return None;
    }
    let start_x: f64 = packet["start-x"].clone().as_f64().unwrap_or(f64::MAX);
    let start_y: f64 = packet["start-y"].clone().as_f64().unwrap_or(f64::MAX);
    let end_x: f64 = packet["end-x"].clone().as_f64().unwrap_or(f64::MAX);
    let end_y: f64 = packet["end-y"].clone().as_f64().unwrap_or(f64::MAX);

    if !(start_x as usize) < 5
        || !(start_y as usize) < 9
        || !(end_x as usize) < 5
        || !(end_y as usize) < 9
    {
        return None;
    }
    return Some((start_x, start_y, end_x, end_y));
}