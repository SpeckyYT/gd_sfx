use std::{fs, env};
use std::path::Path;

use serde_json::{json, Value};

// intentionally overusing .unwrap() because there's no other way to find out
// if anything goes wrong in build scripts other than through panics
fn main() {
    // schema for json files in the lang folder
    let mut lang_schema = json!({
        "type": "object",
        "properties": {},
        "required": [],
        "additionalProperties": false
    });

    // get default keys from en_US
    let default_lang = serde_json::from_str::<Value>(include_str!("lang/en_US.json")).unwrap();
    let default_lang = default_lang.as_object().unwrap().clone();

    // register translation keys
    let properties = lang_schema["properties"].as_object_mut().unwrap();
    for key in default_lang.keys() {
        properties.insert(key.to_string(), json!({ "type": "string" }));
    }

    // set all to be required
    let required = lang_schema["required"].as_array_mut().unwrap();
    for key in default_lang.keys() {
        required.push(json!(key));
    }

    // write the json schema to the file specified in .vscode/settings.json
    let json_string = lang_schema.to_string();
    fs::write(Path::new("lang_schema.json"), json_string).unwrap();

    // write new i18n!(...) macro invocation to build output so that main.rs can include it
    let out_dir = env::var_os("OUT_DIR").unwrap();
    fs::write(Path::new(&out_dir).join("i18n.rs"), r#"i18n!("lang", fallback = "en_US");"#).unwrap();

    // rerun if any file in the lang folder changes
    Path::new("lang")
        .read_dir().unwrap()
        .flatten()
        .map(|entry| entry.path())
        .for_each(build_script::cargo_rerun_if_changed);
}
