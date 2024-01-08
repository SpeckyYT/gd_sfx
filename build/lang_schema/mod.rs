use std::fs;

use serde_json::{json, Value};

use crate::util;

const PROJECT_SETTINGS_PATH: &str = ".vscode/settings.json";
const LANG_SCHEMA_TEMPLATE: &str = include_str!("template.json");

pub fn build() {
    // read project settings
    let project_settings = util::read_json(PROJECT_SETTINGS_PATH);
    
    // find json schema with {"id": "lang"}
    let lang_schema_settings = project_settings["json.schemas"]
        .as_array().unwrap()
        .iter()
        .find(|schema| {
            schema.get("id")
                .and_then(Value::as_str)
                .map_or(false, |id| id == "lang")
        })
        .unwrap_or_else(|| panic!("No language file JSON schema found.\nAdd {{\"id\": \"lang\"}} to a JSON schema in '{PROJECT_SETTINGS_PATH}' to designate it as a language file JSON schema"));

    // read lang schema template from file
    let mut lang_schema_template = serde_json::from_str::<Value>(LANG_SCHEMA_TEMPLATE)
        .unwrap_or_else(|e| panic!("Invalid JSON in lang schema template: {e}"));

    // get predefined lang schema properties
    let properties = lang_schema_template["properties"].as_object_mut().unwrap();

    // get default keys from source file specified in schema
    let default_lang_path = lang_schema_settings["source"].as_str()
        .unwrap_or_else(|| panic!("Specify a lang file to generate the JSON schema from using {{\"source\": \"path/to/json\"}}"));

    // filter out keys already specified in properties
    let default_lang_json = util::read_json(default_lang_path);
    let default_lang = default_lang_json
        .as_object().unwrap()
        .keys()
        .filter(|&key| !properties.contains_key(key))
        .collect::<Vec<_>>();

    // register translation keys
    for key in &default_lang {
        properties.insert(key.to_string(), json!({ "type": "string" }));
    }

    // set all to be required
    let required = lang_schema_template["required"].as_array_mut().unwrap();
    for key in &default_lang {
        required.push(json!(key));
    }

    // write lang schema to the file specified in the project settings
    let lang_schema_path = lang_schema_settings["url"].as_str().unwrap();
    fs::write(lang_schema_path, lang_schema_template.to_string())
        .unwrap_or_else(|e| panic!("Couldn't write lang schema to file '{lang_schema_path}': {e}"));
}
