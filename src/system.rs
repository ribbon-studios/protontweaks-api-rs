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
    pub args: Vec<String>,
    pub settings: TweakSettings,
}

#[derive(PartialEq, Debug)]
pub enum GPU {
    AMD,
    NVIDIA,
    UNKNOWN,
}

impl GPU {
    pub fn as_str(&self) -> &'static str {
        match self {
            GPU::AMD => "amd",
            GPU::NVIDIA => "nvidia",
            GPU::UNKNOWN => "unknown",
        }
    }
}

pub struct SystemInfo {
    pub driver: String,
    pub driver_type: GPU,
}

pub async fn get_system_info() -> Option<SystemInfo> {
    let instance = wgpu::Instance::default();

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await?;

    Some(SystemInfo {
        driver: adapter.get_info().driver,
        driver_type: match adapter.get_info().driver.as_ref() {
            "NVIDIA" => GPU::NVIDIA,
            // TODO: Need someone with an AMD GPU to verify this is correct
            "AMD" => GPU::AMD,
            _ => GPU::UNKNOWN,
        },
    })
}

impl GpuDriver {
    async fn get_gpu_info(&self) -> GPU {
        if let Some(system_info) = get_system_info().await {
            system_info.driver_type
        } else {
            GPU::UNKNOWN
        }
    }

    pub async fn get_tweaks(&self) -> &Option<SystemTweaks> {
        match self.get_gpu_info().await {
            GPU::UNKNOWN => &None,
            gpu => {
                info!("Detected gpu as '{}'", gpu.as_str());
                self.get_tweaks_for_gpu(gpu)
            }
        }
    }

    pub fn get_tweaks_for_gpu(&self, gpu: GPU) -> &Option<SystemTweaks> {
        match gpu {
            GPU::AMD => &self.amd,
            GPU::NVIDIA => &self.nvidia,
            GPU::UNKNOWN => &None,
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
                args: vec![],
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
