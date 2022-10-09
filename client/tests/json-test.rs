
use serde_json::json;
use serde_json::{Value, Number};

fn main(){
    let mut json: Value = serde_json::from_str(r#"
        {
            "packet":"player",
            "names": [
                "foo",
                "bar"
            ]
        }
    "#).unwrap();

    if let Value::Object(ref mut map) = json{
        map.insert("added".to_string(), Value::String("this".to_string()));
    }

    assert_eq!(json["packet"], Value::String("player".to_string()));
    assert_eq!(json["something"], Value::Null);
    assert_eq!(json["names"], Value::Array(vec![Value::String("foo".to_string()), Value::String("bar".to_string())]));
    assert_eq!(json["added"], "this")
}