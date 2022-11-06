use serde_json::Value;
use serde_json::{self, Number};

pub(crate) fn move_packet(start_x: i32, start_y: i32, end_x: i32, end_y: i32) -> String {
    let mut json = serde_json::from_str(
        r#"
        {
            "packet-type": "move-troop"
        }
    "#,
    )
    .unwrap();
    if let Value::Object(ref mut map) = json {
        map.insert(
            "start-x".to_string(),
            Value::Number(Number::from_f64(start_x as f64).unwrap()),
        );
        map.insert(
            "start-y".to_string(),
            Value::Number(Number::from_f64(start_y as f64).unwrap()),
        );

        map.insert(
            "end-x".to_string(),
            Value::Number(Number::from_f64(end_x as f64).unwrap()),
        );
        map.insert(
            "end-y".to_string(),
            Value::Number(Number::from_f64(end_y as f64).unwrap()),
        );
    }
    return serde_json::to_string(&json).unwrap();
}
