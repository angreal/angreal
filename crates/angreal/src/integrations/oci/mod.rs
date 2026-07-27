//! OCI registry integration.
//!
//! A synchronous wrapper over the async [`oci_client`] crate. Every public
//! method blocks on a small current-thread tokio runtime so the rest of angreal
//! — which is entirely synchronous — can pull and push OCI artifacts without
//! ever touching `async`. This module is the shared core consumed both by
//! `angreal init oci://...` (template targets) and by the
//! `angreal.integrations.oci` Python binding.
//!
//! ## Template artifact convention (v1)
//!
//! An angreal template is distributed as a standard OCI image manifest with:
//! - a config blob of media type [`ANGREAL_CONFIG_MEDIA_TYPE`], and
//! - one layer of media type [`ANGREAL_LAYER_MEDIA_TYPE`] — a gzipped tar of the
//!   template directory (the same tree `angreal init` renders).
//!
//! Because it is a spec-compliant manifest, `oras`/`skopeo` and any OCI registry
//! interoperate with artifacts angreal pushes.

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, bail, Context, Result};
use base64::Engine;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use oci_client::client::{ClientConfig, ClientProtocol, Config, ImageLayer};
use oci_client::secrets::RegistryAuth;
use oci_client::{Client, Reference};
use tar::Archive;

/// Media type for the angreal template artifact config blob.
pub const ANGREAL_CONFIG_MEDIA_TYPE: &str = "application/vnd.angreal.template.config.v1+json";

/// Media type for the angreal template artifact layer: a gzipped tar of the
/// template directory.
pub const ANGREAL_LAYER_MEDIA_TYPE: &str = "application/vnd.angreal.template.layer.v1.tar+gzip";

/// Authentication for a registry operation.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum OciAuth {
    /// No credentials — for public, read-anonymous repositories.
    #[default]
    Anonymous,
    /// HTTP Basic credentials (username + password or token).
    Basic { username: String, password: String },
}

impl OciAuth {
    fn to_registry_auth(&self) -> RegistryAuth {
        match self {
            OciAuth::Anonymous => RegistryAuth::Anonymous,
            OciAuth::Basic { username, password } => {
                RegistryAuth::Basic(username.clone(), password.clone())
            }
        }
    }
}

/// Synchronous OCI registry client for angreal template artifacts.
pub struct Oci;

impl Oci {
    /// Whether OCI support is available. Always `true` — the client is bundled
    /// (pure Rust), with no external binary or daemon required.
    pub fn is_available() -> bool {
        true
    }

    /// Pull a template artifact and unpack its layer into `dest`.
    ///
    /// Accepts a bare reference (`registry/repo:tag`) or one with an `oci://`
    /// scheme prefix. When `insecure` is set, the registry is contacted over
    /// plain HTTP (for self-hosted registries without TLS). Returns the
    /// destination path on success.
    pub fn pull_artifact(
        reference: &str,
        dest: &Path,
        auth: &OciAuth,
        insecure: bool,
    ) -> Result<PathBuf> {
        let reference = parse_reference(reference)?;
        let client = build_client(&reference, insecure);
        let registry_auth = auth.to_registry_auth();

        let image_data = run_async(async {
            client
                .pull(&reference, &registry_auth, vec![ANGREAL_LAYER_MEDIA_TYPE])
                .await
        })
        .map_err(|e| anyhow!("failed to pull OCI artifact '{reference}': {e}"))?;

        fs::create_dir_all(dest)
            .with_context(|| format!("failed to create destination '{}'", dest.display()))?;

        let mut unpacked = false;
        for layer in &image_data.layers {
            if layer.media_type == ANGREAL_LAYER_MEDIA_TYPE {
                unpack_targz(&layer.data, dest).with_context(|| {
                    format!("failed to unpack template layer into '{}'", dest.display())
                })?;
                unpacked = true;
            }
        }

        if !unpacked {
            bail!(
                "OCI artifact '{reference}' contains no angreal template layer \
                 (expected media type '{ANGREAL_LAYER_MEDIA_TYPE}')"
            );
        }

        Ok(dest.to_path_buf())
    }

    /// Package `src_dir` as an angreal template artifact and push it to
    /// `reference`. When `insecure` is set, the registry is contacted over plain
    /// HTTP.
    pub fn push_artifact(
        reference: &str,
        src_dir: &Path,
        auth: &OciAuth,
        insecure: bool,
    ) -> Result<()> {
        if !src_dir.is_dir() {
            bail!(
                "cannot push OCI template: '{}' is not a directory",
                src_dir.display()
            );
        }

        let reference = parse_reference(reference)?;
        let client = build_client(&reference, insecure);
        let registry_auth = auth.to_registry_auth();

        let tar_gz = pack_targz(src_dir).with_context(|| {
            format!(
                "failed to package template directory '{}'",
                src_dir.display()
            )
        })?;

        let layer = ImageLayer::new(tar_gz, ANGREAL_LAYER_MEDIA_TYPE.to_string(), None);
        let config = Config::new(
            template_config_blob(),
            ANGREAL_CONFIG_MEDIA_TYPE.to_string(),
            None,
        );

        run_async(async {
            client
                .push(&reference, &[layer], config, &registry_auth, None)
                .await
        })
        .map_err(|e| anyhow!("failed to push OCI artifact '{reference}': {e}"))?;

        Ok(())
    }

    /// List the tags available for a repository. When `insecure` is set, the
    /// registry is contacted over plain HTTP.
    pub fn list_tags(repository: &str, auth: &OciAuth, insecure: bool) -> Result<Vec<String>> {
        let reference = parse_reference(repository)?;
        let client = build_client(&reference, insecure);
        let registry_auth = auth.to_registry_auth();

        let response = run_async(async {
            client
                .list_tags(&reference, &registry_auth, None, None)
                .await
        })
        .map_err(|e| anyhow!("failed to list tags for '{reference}': {e}"))?;

        Ok(response.tags)
    }
}

/// Resolve Basic auth for `registry` from the Docker `~/.docker/config.json`
/// credential store, if present. Returns `None` when no matching entry exists.
pub fn docker_config_auth(registry: &str) -> Option<OciAuth> {
    let config_path = home::home_dir()?.join(".docker").join("config.json");
    let contents = fs::read_to_string(config_path).ok()?;
    let json: serde_json::Value = serde_json::from_str(&contents).ok()?;
    let auths = json.get("auths")?.as_object()?;

    // Registries appear under several conventional key spellings.
    let entry = auths
        .get(registry)
        .or_else(|| auths.get(&format!("https://{registry}")))
        .or_else(|| auths.get(&format!("https://{registry}/v1/")))?;

    let encoded = entry.get("auth")?.as_str()?;
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(encoded)
        .ok()?;
    let pair = String::from_utf8(decoded).ok()?;
    let (username, password) = pair.split_once(':')?;

    Some(OciAuth::Basic {
        username: username.to_string(),
        password: password.to_string(),
    })
}

/// Resolve auth for a reference: Docker `config.json` credentials for the
/// registry if present, otherwise anonymous. Shared by `angreal init oci://...`
/// and the Python binding so both authenticate the same way.
pub fn resolve_auth(reference: &str) -> OciAuth {
    parse_reference(reference)
        .ok()
        .and_then(|r| docker_config_auth(r.registry()))
        .unwrap_or(OciAuth::Anonymous)
}

/// The registry host (and port) of a reference, e.g. `ghcr.io` or
/// `localhost:5000`.
pub fn reference_registry(reference: &str) -> Result<String> {
    Ok(parse_reference(reference)?.registry().to_string())
}

/// The conventional local directory name for a reference — its repository's last
/// path segment — used as the default pull destination.
pub fn reference_basename(reference: &str) -> Result<String> {
    let r = parse_reference(reference)?;
    let name = r.repository().rsplit('/').next().unwrap_or("template");
    Ok(sanitize_segment(name))
}

/// Compute a filesystem-safe cache subpath (`<registry>/<repository>/<version>`)
/// for a reference, used to key the local template cache. `version` is the tag,
/// else the digest, else `latest`.
pub fn cache_subpath(reference: &str) -> Result<PathBuf> {
    let r = parse_reference(reference)?;
    let version = r.tag().or_else(|| r.digest()).unwrap_or("latest");

    let mut path = PathBuf::from(sanitize_segment(r.registry()));
    for segment in r.repository().split('/') {
        path.push(sanitize_segment(segment));
    }
    path.push(sanitize_segment(version));
    Ok(path)
}

/// Reduce a reference component to a safe single path segment (registries carry
/// ports/colons, digests carry colons — neither is portable in a path).
fn sanitize_segment(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '_') {
                c
            } else {
                '_'
            }
        })
        .collect()
}

/// Parse a reference, tolerating an optional `oci://` scheme prefix.
fn parse_reference(reference: &str) -> Result<Reference> {
    let stripped = reference.strip_prefix("oci://").unwrap_or(reference);
    stripped
        .parse::<Reference>()
        .map_err(|e| anyhow!("invalid OCI reference '{reference}': {e}"))
}

/// Whether to talk to `registry` over plain HTTP: always for `localhost` /
/// `127.0.0.1` (test/loopback registries), or when the caller opts into
/// `insecure` (a self-hosted registry without TLS).
fn use_http_for(registry: &str, insecure: bool) -> bool {
    insecure || registry.starts_with("localhost") || registry.starts_with("127.0.0.1")
}

/// Build a client for the given reference. HTTPS by default, riding the
/// vendored-OpenSSL native-tls backend already in the tree; plain HTTP for
/// loopback registries or when `insecure` is set.
fn build_client(reference: &Reference, insecure: bool) -> Client {
    let registry = reference.registry();
    let protocol = if use_http_for(registry, insecure) {
        ClientProtocol::HttpsExcept(vec![registry.to_string()])
    } else {
        ClientProtocol::Https
    };

    Client::new(ClientConfig {
        protocol,
        ..Default::default()
    })
}

/// Block on `fut` using a lightweight current-thread runtime. The async surface
/// of `oci-client` is fully contained here.
fn run_async<F: std::future::Future>(fut: F) -> F::Output {
    // A fresh current-thread runtime per operation keeps the async footprint
    // minimal and avoids holding a runtime across the synchronous API boundary.
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("failed to build tokio runtime for OCI operation");
    runtime.block_on(fut)
}

/// The config blob embedded in a pushed template artifact.
fn template_config_blob() -> Vec<u8> {
    let config = serde_json::json!({
        "angrealVersion": env!("CARGO_PKG_VERSION"),
        "artifactType": "application/vnd.angreal.template",
    });
    serde_json::to_vec(&config).unwrap_or_else(|_| b"{}".to_vec())
}

/// gzip+tar the contents of `src_dir` (rooted at `.`) into an in-memory buffer.
fn pack_targz(src_dir: &Path) -> Result<Vec<u8>> {
    let encoder = GzEncoder::new(Vec::new(), Compression::default());
    let mut builder = tar::Builder::new(encoder);
    builder.append_dir_all(".", src_dir)?;
    let encoder = builder.into_inner()?;
    let bytes = encoder.finish()?;
    Ok(bytes)
}

/// Unpack a gzipped tar buffer into `dest`.
fn unpack_targz(data: &[u8], dest: &Path) -> Result<()> {
    let decoder = GzDecoder::new(data);
    let mut archive = Archive::new(decoder);
    archive.unpack(dest)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn media_types_are_stable() {
        assert_eq!(
            ANGREAL_CONFIG_MEDIA_TYPE,
            "application/vnd.angreal.template.config.v1+json"
        );
        assert_eq!(
            ANGREAL_LAYER_MEDIA_TYPE,
            "application/vnd.angreal.template.layer.v1.tar+gzip"
        );
    }

    #[test]
    fn oci_auth_defaults_to_anonymous() {
        assert_eq!(OciAuth::default(), OciAuth::Anonymous);
    }

    #[test]
    fn parse_reference_strips_oci_scheme() {
        let a = parse_reference("oci://ghcr.io/angreal/python:latest").unwrap();
        let b = parse_reference("ghcr.io/angreal/python:latest").unwrap();
        assert_eq!(a.registry(), "ghcr.io");
        assert_eq!(a.repository(), b.repository());
        assert_eq!(a.tag(), Some("latest"));
    }

    #[test]
    fn parse_reference_rejects_garbage() {
        assert!(parse_reference("not a reference !!").is_err());
    }

    #[test]
    fn is_available_is_true() {
        assert!(Oci::is_available());
    }

    #[test]
    fn use_http_for_loopback_and_insecure() {
        // Loopback registries are always plain HTTP.
        assert!(use_http_for("localhost:5000", false));
        assert!(use_http_for("127.0.0.1:5000", false));
        // Real registries are HTTPS unless the caller opts into insecure.
        assert!(!use_http_for("ghcr.io", false));
        assert!(use_http_for("ghcr.io", true));
        assert!(use_http_for("registry.internal:5000", true));
    }

    #[test]
    fn cache_subpath_maps_registry_repo_tag() {
        let p = cache_subpath("oci://ghcr.io/angreal/python:latest").unwrap();
        assert_eq!(p, PathBuf::from("ghcr.io/angreal/python/latest"));
    }

    #[test]
    fn cache_subpath_defaults_tag_to_latest() {
        let p = cache_subpath("ghcr.io/angreal/python").unwrap();
        assert_eq!(p, PathBuf::from("ghcr.io/angreal/python/latest"));
    }

    #[test]
    fn cache_subpath_sanitizes_registry_port() {
        let p = cache_subpath("localhost:5000/demo:v1").unwrap();
        assert_eq!(p, PathBuf::from("localhost_5000/demo/v1"));
    }

    #[test]
    fn pack_unpack_roundtrip_is_faithful() {
        let src = TempDir::new().unwrap();
        fs::write(src.path().join("angreal.toml"), "name = \"demo\"\n").unwrap();
        fs::create_dir(src.path().join("sub")).unwrap();
        fs::write(src.path().join("sub").join("file.txt"), "hello\n").unwrap();

        let packed = pack_targz(src.path()).unwrap();

        let dest = TempDir::new().unwrap();
        unpack_targz(&packed, dest.path()).unwrap();

        let toml = fs::read_to_string(dest.path().join("angreal.toml")).unwrap();
        assert_eq!(toml, "name = \"demo\"\n");
        let nested = fs::read_to_string(dest.path().join("sub").join("file.txt")).unwrap();
        assert_eq!(nested, "hello\n");
    }
}
