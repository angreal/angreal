---
title: "angreal::integrations::oci"
description: "OCI registry integration."
sidebar:
  order: 27
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>


OCI registry integration.

A synchronous wrapper over the async [`oci_client`] crate. Every public
method blocks on a small current-thread tokio runtime so the rest of angreal
— which is entirely synchronous — can pull and push OCI artifacts without
ever touching `async`. This module is the shared core consumed both by
`angreal init oci://...` (template targets) and by the
`angreal.integrations.oci` Python binding.

## Structs

### <span id="struct.Oci"></span>`struct Oci`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


Synchronous OCI registry client for angreal template artifacts.

#### Methods

##### <span id="Oci.is_available"></span>`is_available`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


```rust
fn is_available () -> bool
```

Whether OCI support is available. Always `true` — the client is bundled (pure Rust), with no external binary or daemon required.

<details>
<summary>Source</summary>

```rust
    pub fn is_available() -> bool {
        true
    }
```

</details>



##### <span id="Oci.pull_artifact"></span>`pull_artifact`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


```rust
fn pull_artifact (reference : & str , dest : & Path , auth : & OciAuth , insecure : bool ,) -> Result < PathBuf >
```

Pull a template artifact and unpack its layer into `dest`.

Accepts a bare reference (`registry/repo:tag`) or one with an `oci://`
scheme prefix. When `insecure` is set, the registry is contacted over
plain HTTP (for self-hosted registries without TLS). Returns the
destination path on success.

<details>
<summary>Source</summary>

```rust
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
```

</details>



##### <span id="Oci.push_artifact"></span>`push_artifact`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


```rust
fn push_artifact (reference : & str , src_dir : & Path , auth : & OciAuth , insecure : bool ,) -> Result < () >
```

Package `src_dir` as an angreal template artifact and push it to `reference`. When `insecure` is set, the registry is contacted over plain HTTP.

<details>
<summary>Source</summary>

```rust
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
```

</details>



##### <span id="Oci.list_tags"></span>`list_tags`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


```rust
fn list_tags (repository : & str , auth : & OciAuth , insecure : bool) -> Result < Vec < String > >
```

List the tags available for a repository. When `insecure` is set, the registry is contacted over plain HTTP.

<details>
<summary>Source</summary>

```rust
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
```

</details>





## Enums

### <span id="enum.OciAuth"></span>`enum OciAuth`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


Authentication for a registry operation.

#### Variants

- **`Anonymous`** - No credentials — for public, read-anonymous repositories.
- **`Basic`** - HTTP Basic credentials (username + password or token).



## Functions

### <span id="fn.docker_config_auth"></span>`fn docker_config_auth`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


```rust
fn docker_config_auth (registry : & str) -> Option < OciAuth >
```

Resolve Basic auth for `registry` from the Docker `~/.docker/config.json` credential store, if present. Returns `None` when no matching entry exists.

<details>
<summary>Source</summary>

```rust
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
```

</details>



### <span id="fn.resolve_auth"></span>`fn resolve_auth`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


```rust
fn resolve_auth (reference : & str) -> OciAuth
```

Resolve auth for a reference: Docker `config.json` credentials for the registry if present, otherwise anonymous. Shared by `angreal init oci://...` and the Python binding so both authenticate the same way.

<details>
<summary>Source</summary>

```rust
pub fn resolve_auth(reference: &str) -> OciAuth {
    parse_reference(reference)
        .ok()
        .and_then(|r| docker_config_auth(r.registry()))
        .unwrap_or(OciAuth::Anonymous)
}
```

</details>



### <span id="fn.reference_registry"></span>`fn reference_registry`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


```rust
fn reference_registry (reference : & str) -> Result < String >
```

The registry host (and port) of a reference, e.g. `ghcr.io` or `localhost:5000`.

<details>
<summary>Source</summary>

```rust
pub fn reference_registry(reference: &str) -> Result<String> {
    Ok(parse_reference(reference)?.registry().to_string())
}
```

</details>



### <span id="fn.reference_basename"></span>`fn reference_basename`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


```rust
fn reference_basename (reference : & str) -> Result < String >
```

The conventional local directory name for a reference — its repository's last path segment — used as the default pull destination.

<details>
<summary>Source</summary>

```rust
pub fn reference_basename(reference: &str) -> Result<String> {
    let r = parse_reference(reference)?;
    let name = r.repository().rsplit('/').next().unwrap_or("template");
    Ok(sanitize_segment(name))
}
```

</details>



### <span id="fn.cache_subpath"></span>`fn cache_subpath`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


```rust
fn cache_subpath (reference : & str) -> Result < PathBuf >
```

Compute a filesystem-safe cache subpath (`<registry>/<repository>/<version>`) for a reference, used to key the local template cache. `version` is the tag, else the digest, else `latest`.

<details>
<summary>Source</summary>

```rust
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
```

</details>



### <span id="fn.sanitize_segment"></span>`fn sanitize_segment`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-gray-6); color: var(--sl-color-gray-2);">private</span>


```rust
fn sanitize_segment (s : & str) -> String
```

Reduce a reference component to a safe single path segment (registries carry ports/colons, digests carry colons — neither is portable in a path).

<details>
<summary>Source</summary>

```rust
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
```

</details>



### <span id="fn.parse_reference"></span>`fn parse_reference`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-gray-6); color: var(--sl-color-gray-2);">private</span>


```rust
fn parse_reference (reference : & str) -> Result < Reference >
```

Parse a reference, tolerating an optional `oci://` scheme prefix.

<details>
<summary>Source</summary>

```rust
fn parse_reference(reference: &str) -> Result<Reference> {
    let stripped = reference.strip_prefix("oci://").unwrap_or(reference);
    stripped
        .parse::<Reference>()
        .map_err(|e| anyhow!("invalid OCI reference '{reference}': {e}"))
}
```

</details>



### <span id="fn.use_http_for"></span>`fn use_http_for`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-gray-6); color: var(--sl-color-gray-2);">private</span>


```rust
fn use_http_for (registry : & str , insecure : bool) -> bool
```

Whether to talk to `registry` over plain HTTP: always for `localhost` / `127.0.0.1` (test/loopback registries), or when the caller opts into `insecure` (a self-hosted registry without TLS).

<details>
<summary>Source</summary>

```rust
fn use_http_for(registry: &str, insecure: bool) -> bool {
    insecure || registry.starts_with("localhost") || registry.starts_with("127.0.0.1")
}
```

</details>



### <span id="fn.build_client"></span>`fn build_client`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-gray-6); color: var(--sl-color-gray-2);">private</span>


```rust
fn build_client (reference : & Reference , insecure : bool) -> Client
```

Build a client for the given reference. HTTPS by default, riding the vendored-OpenSSL native-tls backend already in the tree; plain HTTP for loopback registries or when `insecure` is set.

<details>
<summary>Source</summary>

```rust
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
```

</details>



### <span id="fn.run_async"></span>`fn run_async`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-gray-6); color: var(--sl-color-gray-2);">private</span>


```rust
fn run_async < F : std :: future :: Future > (fut : F) -> F :: Output
```

Block on `fut` using a lightweight current-thread runtime. The async surface of `oci-client` is fully contained here.

<details>
<summary>Source</summary>

```rust
fn run_async<F: std::future::Future>(fut: F) -> F::Output {
    // A fresh current-thread runtime per operation keeps the async footprint
    // minimal and avoids holding a runtime across the synchronous API boundary.
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("failed to build tokio runtime for OCI operation");
    runtime.block_on(fut)
}
```

</details>



### <span id="fn.template_config_blob"></span>`fn template_config_blob`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-gray-6); color: var(--sl-color-gray-2);">private</span>


```rust
fn template_config_blob () -> Vec < u8 >
```

The config blob embedded in a pushed template artifact.

<details>
<summary>Source</summary>

```rust
fn template_config_blob() -> Vec<u8> {
    let config = serde_json::json!({
        "angrealVersion": env!("CARGO_PKG_VERSION"),
        "artifactType": "application/vnd.angreal.template",
    });
    serde_json::to_vec(&config).unwrap_or_else(|_| b"{}".to_vec())
}
```

</details>



### <span id="fn.pack_targz"></span>`fn pack_targz`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-gray-6); color: var(--sl-color-gray-2);">private</span>


```rust
fn pack_targz (src_dir : & Path) -> Result < Vec < u8 > >
```

gzip+tar the contents of `src_dir` (rooted at `.`) into an in-memory buffer.

<details>
<summary>Source</summary>

```rust
fn pack_targz(src_dir: &Path) -> Result<Vec<u8>> {
    let encoder = GzEncoder::new(Vec::new(), Compression::default());
    let mut builder = tar::Builder::new(encoder);
    builder.append_dir_all(".", src_dir)?;
    let encoder = builder.into_inner()?;
    let bytes = encoder.finish()?;
    Ok(bytes)
}
```

</details>



### <span id="fn.unpack_targz"></span>`fn unpack_targz`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-gray-6); color: var(--sl-color-gray-2);">private</span>


```rust
fn unpack_targz (data : & [u8] , dest : & Path) -> Result < () >
```

Unpack a gzipped tar buffer into `dest`.

<details>
<summary>Source</summary>

```rust
fn unpack_targz(data: &[u8], dest: &Path) -> Result<()> {
    let decoder = GzDecoder::new(data);
    let mut archive = Archive::new(decoder);
    archive.unpack(dest)?;
    Ok(())
}
```

</details>
