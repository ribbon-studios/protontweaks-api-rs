use app::App;
use log::trace;
use reqwest::{StatusCode, Url};
use serde::{de::DeserializeOwned, Deserialize};

pub mod app;
pub mod system;

pub struct Protontweaks {
    url: String,
}

#[derive(Debug, Deserialize)]
pub struct AppsList {
    pub sha: String,
    pub short_sha: String,
    pub apps: Vec<MicroApp>,
}

#[derive(Debug, Deserialize)]
pub struct MicroApp {
    pub id: String,
    pub name: String,
}

impl Default for Protontweaks {
    fn default() -> Self {
        Self {
            url: "https://api.protontweaks.com/v4".to_string(),
        }
    }
}

impl Protontweaks {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_url(url: &str) -> Self {
        Self {
            url: url.to_string(),
            ..Self::default()
        }
    }

    pub fn to_url(&self, endpoint: &str) -> Url {
        let url = Url::parse(&self.url).unwrap();

        return url.join(endpoint).unwrap();
    }

    async fn get<T: DeserializeOwned>(&self, url: &str) -> Result<T, String> {
        let url = self.to_url(url);

        trace!("Requesting apps from '{url}'...");

        let response = reqwest::get(url.clone())
            .await
            .map_err(|error| error.to_string())?;

        if response.status().is_success() {
            response
                .json::<T>()
                .await
                .map_err(|_| "Failed to parse apps.json".to_string())
        } else {
            match response.status() {
                StatusCode::NOT_FOUND => {
                    Err(format!("Unable to locate file at '{}'.", url.to_string()))
                }
                _ => Err(response.error_for_status().unwrap_err().to_string()),
            }
        }
    }

    pub async fn try_apps_list(&self) -> Result<AppsList, String> {
        self.get::<AppsList>("apps.json").await
    }

    pub async fn apps_list(&self) -> AppsList {
        self.try_apps_list().await.unwrap()
    }

    pub async fn try_apps(&self) -> Result<Vec<MicroApp>, String> {
        self.try_apps_list().await.map(|apps_list| apps_list.apps)
    }

    pub async fn apps(&self) -> Vec<MicroApp> {
        self.try_apps().await.unwrap()
    }

    pub async fn try_app_ids(&self) -> Result<Vec<String>, String> {
        self.try_apps()
            .await
            .map(|apps| apps.iter().map(|app| app.id.clone()).collect())
    }

    pub async fn app_ids(&self) -> Vec<String> {
        self.try_app_ids().await.unwrap()
    }

    pub async fn try_app(&self, app_id: &str) -> Result<App, String> {
        self.get::<App>(&format!("{app_id}.json")).await
    }

    pub async fn app(&self, app_id: &str) -> App {
        self.try_app(app_id).await.unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn apps_list() {
        let api = Protontweaks::new();

        assert!(
            api.try_apps_list().await.unwrap().apps.len() > 0,
            "Should contain a list of apps"
        );
        assert!(
            api.apps_list().await.apps.len() > 0,
            "Should contain a list of apps"
        );
    }

    #[tokio::test]
    async fn apps() {
        let api = Protontweaks::new();

        assert!(
            api.try_apps().await.unwrap().len() > 0,
            "Should be a list of apps"
        );
        assert!(api.apps().await.len() > 0, "Should be a list of apps");
    }

    #[tokio::test]
    async fn app_ids() {
        let api = Protontweaks::new();

        assert!(
            api.try_app_ids().await.unwrap().len() > 0,
            "Should be a list of app ids"
        );
        assert!(api.app_ids().await.len() > 0, "Should be a list of app ids");
    }

    #[tokio::test]
    async fn try_app() {
        let expected_id = "644930";
        let api = Protontweaks::new();

        let app = api.try_app(expected_id).await.unwrap();

        assert_eq!(app.id, expected_id);
        assert_eq!(app.issues.len(), 1);
        assert_eq!(app.tweaks.tricks.len(), 1);
        assert_eq!(app.tweaks.env.len(), 0);
        assert_eq!(app.tweaks.settings.gamemode, Some(true));
        assert_eq!(app.tweaks.settings.mangohud, Some(true));
    }

    #[tokio::test]
    async fn app() {
        let expected_id = "644930";
        let api = Protontweaks::new();

        let app = api.app(expected_id).await;

        assert_eq!(app.id, expected_id);
        assert_eq!(app.issues.len(), 1);
        assert_eq!(app.tweaks.tricks.len(), 1);
        assert_eq!(app.tweaks.env.len(), 0);
        assert_eq!(app.tweaks.settings.gamemode, Some(true));
        assert_eq!(app.tweaks.settings.mangohud, Some(true));
    }
}
