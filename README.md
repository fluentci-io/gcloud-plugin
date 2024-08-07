# gcloud plugin

[![fluentci pipeline](https://shield.fluentci.io/x/gcloud)](https://pkg.fluentci.io/gcloud)
[![ci](https://github.com/fluentci-io/gcloud-plugin/actions/workflows/ci.yml/badge.svg)](https://github.com/fluentci-io/gcloud-plugin/actions/workflows/ci.yml)

This plugin sets up your CI/CD pipeline with a specific version of [gcloud cli](https://cloud.google.com/sdk/gcloud).

## 🚀 Usage

Add the following command to your CI configuration file:

```bash
fluentci run --wasm gcloud setup
```

## Functions

| Name   | Description                               |
| ------ | ----------------------------------------- |
| setup  | Installs a specific version of gcloud cli.  |

## Code Usage

Add `fluentci-pdk` crate to your `Cargo.toml`:

```toml
[dependencies]
fluentci-pdk = "0.2.1"
```

Use the following code to call the plugin:

```rust
use fluentci_pdk::dag;

// ...

dag().call("https://pkg.fluentci.io/gcloud@v0.1.0?wasm=1", "setup", vec!["latest"])?;
```

## 📚 Examples

Github Actions:

```yaml
- name: Setup Fluent CI CLI
  uses: fluentci-io/setup-fluentci@v5
  with:
    wasm: true
    plugin: gcloud
    args: |
      setup
  env:
    GITHUB_ACCESS_TOKEN: ${{ secrets.GITHUB_TOKEN }}
- name: Show gcloud CLI version
  run: gcloud --version
```
