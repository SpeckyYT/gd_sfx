use std::fs;
use std::path::Path;

use serde_json::{json, Value};

macro_rules! default_lang_path {
    () => { "lang/en_US.json" }
}

fn main() {
    let mut lang_schema = json!({
        "type": "object",
        "properties": {},
        "required": [],
        "additionalProperties": false
    });

    let default_lang = serde_json::from_str::<Value>(include_str!(default_lang_path!())).unwrap();
    let default_lang = default_lang.as_object().unwrap().clone();

    let properties = lang_schema["properties"].as_object_mut().unwrap();
    for key in default_lang.keys() {
        properties.insert(key.to_string(), json!({ "type": "string" }));
    }

    let required = lang_schema["required"].as_array_mut().unwrap();
    for key in default_lang.keys() {
        required.push(json!(key));
    }

    let json_string = lang_schema.to_string();
    fs::write(Path::new("lang_schema.json"), json_string).unwrap();

    build_script::cargo_rerun_if_changed(default_lang_path!());
}
