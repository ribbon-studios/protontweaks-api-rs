use std::collections::HashMap;

use serde::Deserialize;

use crate::system::{System, SystemTweaks, GPU};

#[derive(PartialEq, Debug, Deserialize)]
pub struct TweakSettings {
    pub gamemode: Option<bool>,
    pub mangohud: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct Issue {
    pub description: String,
    pub solution: String,
}

#[derive(Debug, Deserialize)]
pub struct Tweaks {
    pub tricks: Vec<String>,
    pub env: HashMap<String, String>,
    pub settings: TweakSettings,
    pub system: System,
}

#[derive(Debug, Deserialize)]
pub struct App {
    pub id: String,
    pub name: String,
    pub tweaks: Tweaks,
    pub issues: Vec<Issue>,
}

impl Clone for TweakSettings {
    fn clone(&self) -> Self {
        Self {
            gamemode: self.gamemode.clone(),
            mangohud: self.mangohud.clone(),
        }
    }
}

impl App {
    async fn get_gpu_info(&self) -> Option<GPU> {
        let instance = wgpu::Instance::default();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .unwrap();

        match adapter.get_info().driver.as_ref() {
            "NVIDIA" => Some(GPU::NVIDIA),
            // TODO: Need someone with an AMD GPU to verify this is correct
            "AMD" => Some(GPU::AMD),
            _ => None,
        }
    }

    /// Utilizes system info to flatten out the App into a collection of applicable tweaks
    pub async fn flatten(&self) -> SystemTweaks {
        let mut env = self.tweaks.env.clone();
        let mut tricks = self.tweaks.tricks.clone();
        let mut settings = self.tweaks.settings.clone();

        if let Some(gpu_tweaks) = self
            .tweaks
            .system
            .gpu_driver
            .get_tweaks(self.get_gpu_info().await)
        {
            // gpu-level settings overwrite global settings
            env.extend(gpu_tweaks.env.clone());
            tricks.extend(gpu_tweaks.tricks.clone());

            if let Some(gamemode) = gpu_tweaks.settings.gamemode {
                settings.gamemode = Some(gamemode);
            }

            if let Some(mangohud) = gpu_tweaks.settings.mangohud {
                settings.mangohud = Some(mangohud);
            }
        }

        SystemTweaks {
            env,
            tricks,
            settings,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Protontweaks;

    #[tokio::test]
    async fn flatten() {
        let api = Protontweaks::new();

        let app = api.app("644930").await;

        assert_eq!(app.tweaks.tricks.len(), app.flatten().await.tricks.len());
    }
}
