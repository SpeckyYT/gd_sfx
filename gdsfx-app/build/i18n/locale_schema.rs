use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value, Map};

const TEMPLATE: &str = include_str!("schema_template.json");

const SOURCE_FILE: &str = gdsfx_files::workspace_path!("locales/en_US.json");
const TARGET_FILE: &str = gdsfx_files::workspace_path!(".vscode/schemas/locale_schema.json");

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
struct LocaleSchema {
    #[serde(rename = "type")] // rust keyword
    json_type: String,
    properties: Map<String, Value>,
    required: Vec<String>,
    additional_properties: bool,
}

pub fn build() {
    let source_file: Map<String, _> = gdsfx_files::read_json_file(SOURCE_FILE).unwrap();
    let mut template: LocaleSchema = serde_json::from_str(TEMPLATE)
        .expect("Incorrect JSON in locale schema template");

    for key in source_file.keys() {
        // skip already defined properties
        if !template.properties.contains_key(key) {
            template.properties.insert(key.to_string(), json!({ "type": "string" }));
            template.required.push(key.clone());
        }
    }

    let target_file = Path::new(TARGET_FILE);
    let formatted_template = serde_json::to_string_pretty(&template)
        .expect("Couldn't serialize locale schema");

    gdsfx_files::create_parent_dirs(target_file).unwrap();
    gdsfx_files::write_file(target_file, formatted_template).unwrap();
}
