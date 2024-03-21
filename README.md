[![Crates.io][crates-image]][crates-url] [![docs.rs][docsrs-image]][docsrs-url]

[![Coverage][coverage-image]][coverage-url] [![CI Build][github-actions-image]][github-actions-url]

## protontweaks-api-rs

Rust api for interacting with https://api.protontweaks.com

### Usage

```rs
// ...

const api = Protontweaks::new();

async fn my_code() -> Result<(), String> {
    let apps: Vec<MicroApp> = api.try_apps().await?;
    let apps: Vec<MicroApp> = api.apps().await;

    let app: App = api.try_app("644930").await?;
    let app: App = api.app("644930").await;

    let system_tweaks: SystemTweaks = app.flatten(); // This detects the local gpu and merges gpu-specific tweaks into the top level tweaks

    Ok(())
}
```

[github-actions-image]: https://img.shields.io/github/actions/workflow/status/rain-cafe/protontweaks-api-rs/ci.yml?event=push
[github-actions-url]: https://github.com/rain-cafe/protontweaks-api-rs/actions/workflows/ci.yml?query=branch%3Amain
[coverage-image]: https://img.shields.io/codecov/c/github/rain-cafe/protontweaks-api-rs
[coverage-url]: https://app.codecov.io/gh/rain-cafe/protontweaks-api-rs
[crates-image]: https://img.shields.io/crates/v/protontweaks-api.svg
[crates-url]: https://crates.io/crates/protontweaks-api
[docsrs-image]: https://docs.rs/protontweaks-api/badge.svg
[docsrs-url]: https://docs.rs/protontweaks-api/
