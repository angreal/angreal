use angreal::utils;
use pyo3::prelude::*;
use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use tempfile::tempdir;
use toml::map::Map;
use toml::Value;

use crate::common;

#[test]
fn test_get_context() {
    let starting_dir = env::current_dir().unwrap();
    let tmp_dir = common::make_tmp_dir();
    env::set_current_dir(&tmp_dir).unwrap_or(());

    // Create .angreal directory
    fs::create_dir(Path::new(".angreal")).unwrap_or(());

    // Create angreal.toml with test data
    let mut toml_path = PathBuf::from(".angreal");
    toml_path.push("angreal.toml");

    // Create sample TOML content
    let mut data = Map::new();
    data.insert("key_1".to_string(), Value::String("value_1".to_string()));
    data.insert("key_2".to_string(), Value::Integer(1));
    data.insert(
        "name".to_string(),
        Value::String("test_project".to_string()),
    );

    let toml_content = toml::to_string(&Value::Table(data.clone())).unwrap();

    let mut file = File::create(&toml_path).unwrap();
    write!(file, "{}", toml_content).unwrap();

    // Test that get_context returns the expected data
    let context = utils::get_context().unwrap();
    let py_context = context
        .extract::<Map<String, Value>>(Python::with_gil(|py| py))
        .unwrap();

    assert_eq!(
        py_context.get("key_1").unwrap().as_str().unwrap(),
        "value_1"
    );
    assert_eq!(py_context.get("key_2").unwrap().as_integer().unwrap(), 1);
    assert_eq!(
        py_context.get("name").unwrap().as_str().unwrap(),
        "test_project"
    );

    // Clean up
    env::set_current_dir(starting_dir).unwrap_or(());
    fs::remove_dir_all(&tmp_dir).unwrap_or(());
}

#[test]
fn test_get_context_no_angreal_dir() {
    let starting_dir = env::current_dir().unwrap();
    let tmp_dir = tempdir().unwrap();
    env::set_current_dir(&tmp_dir).unwrap_or(());

    // Test that get_context returns an empty map when no .angreal directory exists
    let context = utils::get_context().unwrap();
    let py_context = context
        .extract::<Map<String, Value>>(Python::with_gil(|py| py))
        .unwrap();

    assert!(py_context.is_empty());

    // Clean up
    env::set_current_dir(starting_dir).unwrap_or(());
}

#[test]
fn test_get_context_no_angreal_toml() {
    let starting_dir = env::current_dir().unwrap();
    let tmp_dir = common::make_tmp_dir();
    env::set_current_dir(&tmp_dir).unwrap_or(());

    // Create .angreal directory without angreal.toml
    fs::create_dir(Path::new(".angreal")).unwrap_or(());

    // Test that get_context returns an empty map when no angreal.toml exists
    let context = utils::get_context().unwrap();
    let py_context = context
        .extract::<Map<String, Value>>(Python::with_gil(|py| py))
        .unwrap();

    assert!(py_context.is_empty());

    // Clean up
    env::set_current_dir(starting_dir).unwrap_or(());
    fs::remove_dir_all(&tmp_dir).unwrap_or(());
}
