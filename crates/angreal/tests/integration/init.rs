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
        false,
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
        false,
    );
    // clean up rendered
    let mut rendered_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    rendered_root.push(Path::new("angreal_test_project"));
    let _ = fs::remove_dir_all(&rendered_root);
    // use the long version

    init("angreal/angreal_test_template", true, false, None, false);
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
        false,
    );
    // clean up rendered
    let mut rendered_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    rendered_root.push(Path::new("folder_name"));
    let _ = fs::remove_dir_all(&rendered_root);
    // use the long version
    init("angreal/angreal_test_template", true, false, None, false);

    let mut rendered_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    rendered_root.push(Path::new("angreal_test_project"));
    let _ = fs::remove_dir_all(&rendered_root);
    let _ = fs::remove_dir_all(create_home_dot_angreal());
}

#[test]
fn test_init_short() {
    // clone
    init("angreal_test_template", true, false, None, false);

    let mut rendered_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    rendered_root.push(Path::new("angreal_test_project"));
    let _ = fs::remove_dir_all(&rendered_root);
    let _ = fs::remove_dir_all(create_home_dot_angreal());
}

#[test]
fn test_render_template() {
    let mut template_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    template_root.push(Path::new("tests/common/test_assets/test_template"));
    render_template(&template_root, false, true, None, false);

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
    render_template(&template_root, false, true, values_toml.to_str(), false);

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

// ---------------------------------------------------------------------------
// In-place rendering (ANG-I-0009)
//
// These tests exercise `render_dir` (via `render_template`) directly rather than
// the full `init` entrypoint so they can run hermetically against a temp cwd
// without cloning a remote template. `render_template` renders relative to the
// process current working directory, so each test changes into a unique temp
// directory, renders, and asserts on the resulting layout.
// ---------------------------------------------------------------------------

/// Build a temporary in-place fixture template:
///
/// ```text
/// <root>/
///   angreal.toml
///   {{ folder_variable }}/
///     README.rst
///     src/main.txt
///     .angreal/init.py
/// ```
///
/// Returns the template root directory.
fn make_in_place_template(root: &Path, extra_top_level_templated_dir: bool) -> PathBuf {
    let template = root.join("template");
    let inner = template.join("{{ folder_variable }}");
    fs::create_dir_all(inner.join("src")).unwrap();
    fs::create_dir_all(inner.join(".angreal")).unwrap();

    fs::write(
        template.join("angreal.toml"),
        "folder_variable = \"folder_name\"\n",
    )
    .unwrap();
    fs::write(inner.join("README.rst"), "# {{ folder_variable }}\n").unwrap();
    fs::write(inner.join("src").join("main.txt"), "hello\n").unwrap();
    fs::write(
        inner.join(".angreal").join("init.py"),
        "def init():\n    pass\n",
    )
    .unwrap();

    if extra_top_level_templated_dir {
        fs::create_dir_all(template.join("{{ folder_variable }}_extra")).unwrap();
    }

    template
}

/// TC-001: in-place render strips the single templated root and writes its
/// contents directly into the destination cwd.
#[test]
fn test_in_place_strips_root() {
    let tmp = env::temp_dir().join(format!("angreal_inplace_ok_{}", std::process::id()));
    let _ = fs::remove_dir_all(&tmp);
    let template = make_in_place_template(&tmp, false);

    let cwd = tmp.join("dest");
    fs::create_dir_all(&cwd).unwrap();
    let original = env::current_dir().unwrap();
    env::set_current_dir(&cwd).unwrap();

    render_template(&template, false, true, None, true);

    env::set_current_dir(&original).unwrap();

    // Contents land directly in cwd; the `{{ folder_variable }}` root is gone.
    assert!(cwd.join("README.rst").is_file());
    assert!(cwd.join("src").join("main.txt").is_file());
    assert!(cwd.join(".angreal").is_dir());
    assert!(cwd.join(".angreal").join("angreal.toml").is_file());
    assert!(cwd.join("folder_name").exists().not());

    let _ = fs::remove_dir_all(&tmp);
}

// NOTE on error paths (TC-002 zero roots, TC-003 multiple roots, TC-004
// collision without --force): `render_dir` aborts the process with `exit(1)`
// in these cases, which cannot be caught in-process by `#[test]`. They are
// covered end-to-end (with exit-code assertions) by the Python functional
// suite in `py_tests/test_functional.py`, which drives the `angreal init`
// CLI in a subprocess. The Rust tests here cover the success and --force
// overwrite paths.

/// TC-005: rendering in-place into a cwd that already contains one of the
/// template's files overwrites it when --force is set.
#[test]
fn test_in_place_force_overwrites_existing() {
    let tmp = env::temp_dir().join(format!("angreal_inplace_force_{}", std::process::id()));
    let _ = fs::remove_dir_all(&tmp);
    let template = make_in_place_template(&tmp, false);

    let cwd = tmp.join("dest");
    fs::create_dir_all(&cwd).unwrap();
    // Pre-existing colliding file with sentinel content.
    fs::write(cwd.join("README.rst"), "OLD CONTENT").unwrap();

    let original = env::current_dir().unwrap();
    env::set_current_dir(&cwd).unwrap();

    render_template(&template, false, true, None, true);

    env::set_current_dir(&original).unwrap();

    let contents = fs::read_to_string(cwd.join("README.rst")).unwrap();
    assert!(
        contents.contains("folder_name"),
        "expected --force to overwrite the existing README.rst, got: {contents:?}"
    );

    let _ = fs::remove_dir_all(&tmp);
}
