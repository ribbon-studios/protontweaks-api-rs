use std::collections::HashMap;

use log::info;
use serde::Deserialize;

use crate::app::TweakSettings;

#[derive(Debug, Deserialize)]
pub struct System {
    pub gpu_driver: GpuDriver,
}

#[derive(Debug, Deserialize)]
pub struct GpuDriver {
    pub amd: Option<SystemTweaks>,
    pub nvidia: Option<SystemTweaks>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct SystemTweaks {
    pub tricks: Vec<String>,
    pub env: HashMap<String, String>,
    pub settings: TweakSettings,
}

pub enum GPU {
    AMD,
    NVIDIA,
}

impl GPU {
    pub fn as_str(&self) -> &'static str {
        match self {
            GPU::AMD => "amd",
            GPU::NVIDIA => "nvidia",
        }
    }
}

impl GpuDriver {
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

    pub async fn get_tweaks(&self) -> &Option<SystemTweaks> {
        self.get_gpu_info().await.map_or(&None, |gpu| {
            info!("Detected gpu as '{}'", gpu.as_str());
            self.get_tweaks_for_gpu(gpu)
        })
    }

    pub fn get_tweaks_for_gpu(&self, gpu: GPU) -> &Option<SystemTweaks> {
        match gpu {
            GPU::AMD => &self.amd,
            GPU::NVIDIA => &self.nvidia,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn get_tweaks() {
        let driver = GpuDriver {
            amd: None,
            nvidia: Some(SystemTweaks {
                env: HashMap::new(),
                tricks: vec![],
                settings: TweakSettings {
                    gamemode: None,
                    mangohud: None,
                },
            }),
        };

        assert_eq!(driver.get_tweaks_for_gpu(GPU::AMD), &None);
        assert_eq!(driver.get_tweaks_for_gpu(GPU::NVIDIA), &driver.nvidia);
    }
}
