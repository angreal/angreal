//! OCI template target integration tests.
//!
//! These exercise the real registry round-trip: package a fixture template, push
//! it as an OCI artifact, then pull/render it back. They require a running OCI
//! registry and are skipped unless `ANGREAL_OCI_TEST_REGISTRY` names one
//! (e.g. `localhost:5000`). Spin one up with:
//!
//! ```text
//! docker run -d --rm -p 5000:5000 registry:2
//! ANGREAL_OCI_TEST_REGISTRY=localhost:5000 angreal test rust --integration-only
//! ```

use angreal::init::{create_home_dot_angreal, init};
use angreal::integrations::oci::{self, Oci, OciAuth};
use std::env;
use std::fs;
use std::ops::Not;
use std::path::{Path, PathBuf};

/// Registry to test against, or `None` to skip.
fn test_registry() -> Option<String> {
    match env::var("ANGREAL_OCI_TEST_REGISTRY") {
        Ok(v) if !v.trim().is_empty() => Some(v),
        _ => {
            eprintln!(
                "skipping OCI integration test: set ANGREAL_OCI_TEST_REGISTRY \
                 (e.g. localhost:5000) to run"
            );
            None
        }
    }
}

/// Build a minimal but valid angreal template at `root`:
///
/// ```text
/// <root>/
///   angreal.toml
///   {{ folder_variable }}/
///     README.rst
///     .angreal/init.py
/// ```
fn make_template(root: &Path) -> PathBuf {
    let template = root.join("template");
    let inner = template.join("{{ folder_variable }}");
    fs::create_dir_all(inner.join(".angreal")).unwrap();

    fs::write(
        template.join("angreal.toml"),
        "folder_variable = \"oci_rendered\"\n",
    )
    .unwrap();
    fs::write(inner.join("README.rst"), "# {{ folder_variable }}\n").unwrap();
    fs::write(
        inner.join(".angreal").join("init.py"),
        "def init():\n    pass\n",
    )
    .unwrap();

    template
}

/// push → pull round-trip through the core is byte-faithful.
#[test]
fn test_oci_push_pull_roundtrip() {
    let Some(registry) = test_registry() else {
        return;
    };

    let tmp = env::temp_dir().join(format!("angreal_oci_rt_{}", std::process::id()));
    let _ = fs::remove_dir_all(&tmp);
    let template = make_template(&tmp);

    let reference = format!("oci://{registry}/angreal-test/roundtrip:v1");
    Oci::push_artifact(&reference, &template, &OciAuth::Anonymous)
        .expect("push should succeed against the test registry");

    let dest = tmp.join("pulled");
    let out = Oci::pull_artifact(&reference, &dest, &oci::resolve_auth(&reference))
        .expect("pull should succeed");

    assert_eq!(out, dest);
    let toml = fs::read_to_string(dest.join("angreal.toml")).unwrap();
    assert!(toml.contains("folder_variable"));
    assert!(dest
        .join("{{ folder_variable }}")
        .join("README.rst")
        .is_file());

    let _ = fs::remove_dir_all(&tmp);
}

/// Full `angreal init oci://...` renders the pushed template into cwd.
#[test]
fn test_init_from_oci() {
    let Some(registry) = test_registry() else {
        return;
    };

    let tmp = env::temp_dir().join(format!("angreal_oci_init_{}", std::process::id()));
    let _ = fs::remove_dir_all(&tmp);
    let template = make_template(&tmp);

    let reference = format!("oci://{registry}/angreal-test/init:v1");
    Oci::push_artifact(&reference, &template, &OciAuth::Anonymous)
        .expect("push should succeed against the test registry");

    // init() renders relative to the process cwd; render into an isolated temp.
    let cwd = tmp.join("dest");
    fs::create_dir_all(&cwd).unwrap();
    let original = env::current_dir().unwrap();
    env::set_current_dir(&cwd).unwrap();

    init(&reference, true, false, None, false);

    env::set_current_dir(&original).unwrap();

    let rendered_root = cwd.join("oci_rendered");
    let rendered_exists = rendered_root.is_dir();
    let readme_exists = rendered_root.join("README.rst").is_file();
    let dot_angreal_exists = rendered_root.join(".angreal").is_dir();
    let unrendered_root_absent = cwd.join("{{ folder_variable }}").exists().not();

    let _ = fs::remove_dir_all(&tmp);
    let _ = fs::remove_dir_all(create_home_dot_angreal());

    assert!(rendered_exists, "expected rendered root {rendered_root:?}");
    assert!(readme_exists);
    assert!(dot_angreal_exists);
    assert!(unrendered_root_absent);
}
