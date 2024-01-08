use std::{fs, path::Path};

use serde_json::{json, Value};

use crate::util;

const PROJECT_SETTINGS_PATH: &str = ".vscode/settings.json";
const LOCALE_SCHEMA_ID: &str = "locales";

pub fn build() {
    // read project settings
    let project_settings = util::read_json(PROJECT_SETTINGS_PATH);
    
    // find json schema with {"id": LOCALE_SCHEMA_ID}
    let locale_schema_settings = project_settings
        .get("json.schemas")
        .and_then(Value::as_array)
        .and_then(|schemas| {
            schemas.iter().find(|schema| schema
                .get("id")
                .and_then(Value::as_str)
                .map_or(false, |id| id == LOCALE_SCHEMA_ID))
        })
        .unwrap_or_else(|| panic!("No locale schema found.\nAdd {{\"id\": \"{LOCALE_SCHEMA_ID}\"}} to a JSON schema in '{PROJECT_SETTINGS_PATH}' to designate it as a locale schema"));

    // define locale schema template
    let mut locale_schema_template = json!({
        "type": "object",
        "properties": {
            "language.translators": {
                "type": "array",
                "items": {
                    "type": "string"
                }
            }
        },
        "required": ["language.translators"],
        "additionalProperties": false
    });

    // get predefined locale schema properties
    let properties = locale_schema_template
        .get_mut("properties")
        .and_then(Value::as_object_mut)
        .unwrap_or_else(|| panic!("Locale schema template does not contain a 'properties' object"));

    // get default keys from source file specified in schema
    let default_locale_path = locale_schema_settings
        .get("source")
        .and_then(Value::as_str)
        .unwrap_or_else(|| panic!("Specify a locale file to generate the JSON schema from using {{\"source\": \"path/to/file\"}}"));

    // filter out keys already specified in properties
    let default_locale_json = util::read_json(default_locale_path);
    let default_locale = default_locale_json
        .as_object()
        .unwrap_or_else(|| panic!("JSON in file '{default_locale_path}' is not an object"))
        .keys()
        .filter(|&key| !properties.contains_key(key))
        .collect::<Vec<_>>();

    // register translation keys
    for key in &default_locale {
        properties.insert(key.to_string(), json!({ "type": "string" }));
    }

    // set all to be required
    let required = locale_schema_template
        .get_mut("required")
        .and_then(Value::as_array_mut)
        .unwrap_or_else(|| panic!("Locale schema template does not contain a 'required' array"));

    for key in &default_locale {
        required.push(json!(key));
    }

    // get destination file for locale schema specified in the project settings
    let locale_schema_path = locale_schema_settings
        .get("url")
        .and_then(Value::as_str)
        .unwrap_or_else(|| panic!("Specify a destination file for the locale schema using {{\"url\": \"path/to/file\"}}"));

    // create directories to destination if they don't exist yet
    if let Some(path) = Path::new(locale_schema_path).parent() {
        if !path.exists() {
            std::fs::create_dir_all(path).unwrap();
        }
    }
    
    // write locale schema to destination
    fs::write(locale_schema_path, locale_schema_template.to_string())
        .unwrap_or_else(|e| panic!("Couldn't write locale schema to file '{locale_schema_path}': {e}"));
}
