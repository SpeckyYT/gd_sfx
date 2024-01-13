use std::path::{PathBuf, Path};

use anyhow::Context;
use gdsfx_data::paths;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value, Map};

#[derive(Deserialize)]
struct ProjectSettings {
    #[serde(rename = "json.schemas")]
    schemas: Vec<JSONSchemaSettings>,
}

#[derive(Deserialize)]
struct JSONSchemaSettings {
    url: PathBuf,
    // ignore fileMatch
}

const LOCALE_SCHEMA_TEMPLATE: &str = include_str!("template.json");

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

type LocaleFormat = Map<String, Value>;

pub fn build() {
    gdsfx_build::cargo_rerun_if_changed(paths::build::PROJECT_SETTINGS_FILE);

    let schema_settings = find_locale_schema_settings();

    let mut template: LocaleSchema = serde_json::from_str(LOCALE_SCHEMA_TEMPLATE)
        .context("Incorrect JSON in locale schema template")
        .unwrap();

    let source_locale: LocaleFormat = gdsfx_data::read_json_file(paths::build::LOCALE_SCHEMA_SOURCE_FILE).unwrap();

    for key in source_locale.keys() {
        // skip already defined properties
        if !template.properties.contains_key(key) {
            template.properties.insert(key.to_string(), json!({ "type": "string" }));
            template.required.push(key.clone());
        }
    }

    let destination_file = Path::new(paths::build::CARGO_WORKSPACE_ROOT).join(schema_settings.url);

    let formatted_template = serde_json::to_string_pretty(&template)
        .context("Couldn't serialize locale schema")
        .unwrap();

    gdsfx_data::create_parent_dirs(&destination_file).unwrap();
    gdsfx_data::write_file(destination_file, formatted_template).unwrap();
}

fn find_locale_schema_settings() -> JSONSchemaSettings {
    let project_settings: ProjectSettings = gdsfx_data::read_json_file(paths::build::PROJECT_SETTINGS_FILE).unwrap();

    project_settings.schemas.into_iter()
        .find(|schema| {
            let absolute_path = Path::new(paths::build::CARGO_WORKSPACE_ROOT).join(&schema.url);
            let target_path = Path::new(paths::build::LOCALE_SCHEMA_TARGET_FILE);
            absolute_path == target_path
        })
        .with_context(|| format!(
            "No JSON schema matching {{\"url\": \"{}\"}} found in {}",
            paths::build::LOCALE_SCHEMA_TARGET_FILE, paths::build::PROJECT_SETTINGS_FILE,
        ))
        .unwrap()
}
