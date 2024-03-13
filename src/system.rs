use std::collections::HashMap;

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
    pub fn get_tweaks(&self, gpu: Option<GPU>) -> &Option<SystemTweaks> {
        if let Some(gpu) = gpu {
            match gpu {
                GPU::AMD => &self.amd,
                GPU::NVIDIA => &self.nvidia,
            }
        } else {
            &None
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

        assert_eq!(driver.get_tweaks(Some(GPU::AMD)), &None);
        assert_eq!(driver.get_tweaks(Some(GPU::NVIDIA)), &driver.nvidia);
    }
}
