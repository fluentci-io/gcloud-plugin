use extism_pdk::*;
use fluentci_pdk::dag;

#[plugin_fn]
pub fn setup(version: String) -> FnResult<String> {
    let version = if version.is_empty() {
        "latest".into()
    } else {
        format!("{}", version)
    };

    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "install", &format!("gcloud@{}", version)])?
        .stdout()?;

    Ok(stdout)
}
