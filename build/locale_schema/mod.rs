use std::{fs, path::PathBuf};

use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value, Map};

use crate::util;

#[serde_with::serde_as]
#[derive(Deserialize)]
struct ProjectSettings {
    #[serde(rename = "json.schemas")]
    #[serde_as(as = "serde_with::VecSkipError<_>")] // ignore non-matching entries
    schemas: Vec<LocaleSchemaSettings>,
}

const PROJECT_SETTINGS_PATH: &str = ".vscode/settings.json";

#[derive(Deserialize)]
struct LocaleSchemaSettings {
    #[serde(rename = "url")]
    schema_file: PathBuf,

    // custom key for identifying the schema to use as locale schema, see LOCALE_SCHEMA_ID below
    id: String,
    // custom key for specifying the locale file to generate the schema from
    source: PathBuf,
}

const LOCALE_SCHEMA_ID: &str = "locales";

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
struct LocaleSchema {
    #[serde(rename = "type")]
    json_type: String,
    properties: Map<String, Value>,
    required: Vec<String>,
    additional_properties: bool,
}

const LOCALE_SCHEMA_TEMPLATE: &str = include_str!("template.json");

pub fn build() -> Result<()> {
    let settings = find_locale_schema_settings()?;

    let mut template: LocaleSchema = serde_json::from_str(LOCALE_SCHEMA_TEMPLATE)
        .context("Incorrect JSON in locale schema template")?;
    
    let source_locale: Map<String, Value> = util::read_json_file(settings.source);

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
                .with_context(|| format!("Couldn't create directories to path {path:?}"))?;
        }
    }
    
    fs::write(&destination_file, serde_json::to_string_pretty(&template)?)
        .with_context(|| format!("Couldn't write locale schema to file {destination_file:?}"))?;

    Ok(())
}

fn find_locale_schema_settings() -> Result<LocaleSchemaSettings> {
    let project_settings: ProjectSettings = util::read_json_file(PROJECT_SETTINGS_PATH);
    
    project_settings.schemas
        .into_iter()
        .find(|schema| schema.id == LOCALE_SCHEMA_ID)
        .with_context(|| format!(
            "No locale schema found.\nAdd {{\"id\": \"{LOCALE_SCHEMA_ID}\"}} to a JSON schema in {PROJECT_SETTINGS_PATH:?} to designate it as a locale schema,\nand specify a locale file to generate the JSON schema from using {{\"source\": \"path/to/file\"}}"
        ))
}
