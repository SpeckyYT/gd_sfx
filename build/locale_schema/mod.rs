use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value, Map};

use crate::util;

const PROJECT_SETTINGS_PATH: &str = ".vscode/settings.json";

#[serde_with::serde_as]
#[derive(Deserialize)]
struct ProjectSettings {
    #[serde(rename = "json.schemas")]
    #[serde_as(as = "serde_with::VecSkipError<_>")] // ignore non-matching entries
    schemas: Vec<LocaleSchemaSettings>,
}

#[derive(Deserialize)]
struct LocaleSchemaSettings {
    #[serde(rename = "url")]
    schema_file: PathBuf,

    // custom key for identifying the schema to use as locale schema
    source_locale: PathBuf,
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

pub fn build() {
    let settings = find_locale_schema_settings();

    let mut template: LocaleSchema = serde_json::from_str(LOCALE_SCHEMA_TEMPLATE)
        .unwrap_or_else(|e| panic!("Incorrect JSON in locale schema template: {e}"));
    
    let source_locale: Map<String, Value> = util::read_json_file(settings.source_locale);

    for key in source_locale.keys() {
        // skip already defined properties
        if !template.properties.contains_key(key) {
            template.properties.insert(key.to_string(), json!({ "type": "string" }));
            template.required.push(key.clone());
        }
    }

    let destination_file = settings.schema_file;

    if let Some(path) = destination_file.parent() {
        if !path.exists() {
            std::fs::create_dir_all(path)
                .unwrap_or_else(|e| panic!("Couldn't create directories to path {path:?}: {e}"));
        }
    }

    let formatted_template = serde_json::to_string_pretty(&template)
        .unwrap_or_else(|e| panic!("Couldn't serialize locale schema: {e}"));
    
    fs::write(&destination_file, formatted_template)
        .unwrap_or_else(|e| panic!("Couldn't write locale schema to file {destination_file:?}: {e}"));
}

fn find_locale_schema_settings() -> LocaleSchemaSettings {
    let project_settings: ProjectSettings = util::read_json_file(PROJECT_SETTINGS_PATH);
    
    project_settings.schemas.into_iter().next()
        .unwrap_or_else(|| panic!(
            "No locale schema found.\nAdd {{\"source_locale\": \"path/to/file\"}} to a JSON schema in {PROJECT_SETTINGS_PATH:?} to specify a locale file to generate the locale schema from"
        ))
}
