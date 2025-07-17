use angreal::init::{create_home_dot_angreal, init, render_template};
use std::env;
use std::fs;
use std::ops::Not;
use std::path::{Path, PathBuf};

#[test]
fn test_init_from_git() {
    init(
        "https://github.com/angreal/angreal_test_template.git",
        true,
        false,
        None,
    );
    let mut rendered_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    rendered_root.push(Path::new("angreal_test_project"));
    let _ = fs::remove_dir_all(&rendered_root);

    let _ = fs::remove_dir_all(create_home_dot_angreal());
}

#[test]
fn test_init_long() {
    // clone
    init(
        "https://github.com/angreal/angreal_test_template.git",
        true,
        false,
        None,
    );
    // clean up rendered
    let mut rendered_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    rendered_root.push(Path::new("angreal_test_project"));
    let _ = fs::remove_dir_all(&rendered_root);
    // use the long version

    init("angreal/angreal_test_template", true, false, None);
    let mut rendered_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    rendered_root.push(Path::new("angreal_test_project"));
    let _ = fs::remove_dir_all(&rendered_root);
    let _ = fs::remove_dir_all(create_home_dot_angreal());
}

#[test]
fn test_init_values() {
    let mut values_toml = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    values_toml.push("tests/common/test_assets/values.toml");

    // clone
    init(
        "https://github.com/angreal/angreal_test_template.git",
        true,
        false,
        values_toml.to_str(),
    );
    // clean up rendered
    let mut rendered_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    rendered_root.push(Path::new("folder_name"));
    let _ = fs::remove_dir_all(&rendered_root);
    // use the long version
    init("angreal/angreal_test_template", true, false, None);

    let mut rendered_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    rendered_root.push(Path::new("angreal_test_project"));
    let _ = fs::remove_dir_all(&rendered_root);
    let _ = fs::remove_dir_all(create_home_dot_angreal());
}

#[test]
fn test_init_short() {
    // clone
    init("angreal_test_template", true, false, None);

    let mut rendered_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    rendered_root.push(Path::new("angreal_test_project"));
    let _ = fs::remove_dir_all(&rendered_root);
    let _ = fs::remove_dir_all(create_home_dot_angreal());
}

#[test]
fn test_render_template() {
    let mut template_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    template_root.push(Path::new("tests/common/test_assets/test_template"));
    render_template(&template_root, false, true, None);

    let mut angreal_toml = template_root.clone();
    angreal_toml.push("angreal.toml");

    let mut assets = template_root.clone();
    assets.push("assets");

    let mut rendered_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    rendered_root.push(Path::new("folder_name"));

    let mut dot_angreal = rendered_root.clone();
    dot_angreal.push(Path::new(".angreal"));

    let mut readme_rst = rendered_root.clone();
    readme_rst.push("README.rst");

    let assets_no_exists = assets.is_dir().not();
    let dot_angreal_exists = dot_angreal.is_dir();
    let readme_rst_exists = readme_rst.is_file();
    let rendered_root_exists = rendered_root.is_dir();

    fs::remove_dir_all(&rendered_root).unwrap_or(());

    assert!(assets_no_exists);
    assert!(rendered_root_exists);
    assert!(dot_angreal_exists);
    assert!(readme_rst_exists);
}

#[test]
fn test_render_template_values() {
    let mut values_toml = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    values_toml.push("tests/common/test_assets/values.toml");

    let mut template_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    template_root.push(Path::new("tests/common/test_assets/test_template"));
    render_template(&template_root, false, true, values_toml.to_str());

    let mut angreal_toml = template_root.clone();
    angreal_toml.push("angreal.toml");

    let mut assets = template_root.clone();
    assets.push("assets");

    let mut rendered_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    rendered_root.push(Path::new("folder_name"));

    let mut dot_angreal = rendered_root.clone();
    dot_angreal.push(Path::new(".angreal"));

    let mut readme_rst = rendered_root.clone();
    readme_rst.push("README.rst");

    let assets_no_exists = assets.is_dir().not();
    let dot_angreal_exists = dot_angreal.is_dir();
    let readme_rst_exists = readme_rst.is_file();
    let rendered_root_exists = rendered_root.is_dir();

    fs::remove_dir_all(&rendered_root).unwrap_or(());

    assert!(assets_no_exists);
    assert!(rendered_root_exists);
    assert!(dot_angreal_exists);
    assert!(readme_rst_exists);
}
