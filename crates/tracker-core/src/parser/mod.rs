mod audio;
mod display;
mod input;
mod misc;
mod quality;

use std::fs;
use std::path::Path;

use anyhow::Context;
use anyhow::Result;
use anyhow::anyhow;
pub use audio::AudioSection;
pub use display::{DisplaySection, DisplaySettingsSection, HardwareInfoSection};
pub use input::InputSection;
pub use misc::{
    AccessibilitySection, GameplaySection, GeneralSection, OnlineSection, ReadOnlySection,
};
pub use quality::{CustomQualitySection, QualitySection};

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SiegeSettings {
    #[serde(rename = "GENERAL")]
    pub general: Option<GeneralSection>,
    #[serde(rename = "DISPLAY")]
    pub display: Option<DisplaySection>,
    #[serde(rename = "HARDWARE_INFO")]
    pub hardware_info: Option<HardwareInfoSection>,
    #[serde(rename = "DISPLAY_SETTINGS")]
    pub display_settings: Option<DisplaySettingsSection>,
    #[serde(rename = "QUALITY")]
    pub quality: Option<QualitySection>,
    #[serde(rename = "CUSTOM_QUALITY")]
    pub custom_quality: Option<CustomQualitySection>,
    #[serde(rename = "READ_ONLY")]
    pub read_only: Option<ReadOnlySection>,
    #[serde(rename = "INPUT")]
    pub input: Option<InputSection>,
    #[serde(rename = "AUDIO")]
    pub audio: Option<AudioSection>,
    #[serde(rename = "GAMEPLAY")]
    pub gameplay: Option<GameplaySection>,
    #[serde(rename = "ACCESSIBILITY")]
    pub accessibility: Option<AccessibilitySection>,
    #[serde(rename = "ONLINE")]
    pub online: Option<OnlineSection>,
}

pub fn parse_path(path: &Path) -> Result<SiegeSettings> {
    let init_content = fs::read_to_string(path).context("Failed to read init File")?;
    let normalized = init_content.replace('\r', "");
    parse_ini_string(&normalized)
}

pub fn parse_ini_string(content: &str) -> Result<SiegeSettings> {
    serde_ini::from_str(content).map_err(|err| anyhow!("Failed to parse ini '{:#}'", err))
}

pub fn serialize_ini_string(settings: &SiegeSettings) -> Result<String> {
    serde_ini::to_string(settings).map_err(|e| anyhow!("Failed to serialize ini: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASIC_INI: &str = "\
[GENERAL]
Version=1

[DISPLAY_SETTINGS]
DefaultFOV=90.000000
";

    #[test]
    fn test_parse_general() {
        let result = parse_ini_string(BASIC_INI).unwrap();
        assert_eq!(result.general, Some(GeneralSection { version: Some(1) }));
    }

    #[test]
    fn test_parse_full_game_settings() {
        let content = include_str!("../../../../sample/GameSettings.ini");
        let result = parse_ini_string(content).unwrap();

        assert_eq!(result.general, Some(GeneralSection { version: Some(1) }));

        let display = result.display.unwrap();
        assert_eq!(display.brightness, Some(60.0));
        assert_eq!(display.nv_reflex, Some(2));
        assert_eq!(display.console, Some(0));

        let hw = result.hardware_info.unwrap();
        assert_eq!(hw.gpu_vendor, Some("NVIDIA".to_owned()));
        assert_eq!(hw.gpu_dedicated_memory_mb, Some(32101));

        let ds = result.display_settings.unwrap();
        assert_eq!(ds.resolution_width, Some(3840));
        assert_eq!(ds.resolution_height, Some(2160));
        assert_eq!(ds.default_fov, Some(90.0));
        assert_eq!(
            ds.vulkan_whitelisted_layers,
            Some("VK_LAYER_OW_OBS_HOOK;VK_LAYER_OW_OVERLAY".to_owned())
        );

        assert_eq!(
            result.quality.unwrap().overall_quality_level_name,
            Some("Custom".to_owned())
        );

        let cq = result.custom_quality.unwrap();
        assert_eq!(cq.atmospheric, Some(-1));
        assert_eq!(cq.texture, Some(3));

        let ro = result.read_only.unwrap();
        assert_eq!(ro.engine_settings_version, Some(1));
        assert_eq!(ro.default_values_version, Some(12));

        let input = result.input.unwrap();
        assert_eq!(input.mouse_sensitivity, Some(14));
        assert_eq!(input.toggle_prone, Some(1));

        let audio = result.audio.unwrap();
        assert_eq!(audio.master_volume, Some(90.0));
        assert_eq!(audio.audio_output_device, Some(-1));

        assert_eq!(result.gameplay.unwrap().gameplay_ping_enable, Some(1));
        assert_eq!(
            result.accessibility.unwrap().accessibility_color_mode,
            Some(1)
        );

        let online = result.online.unwrap();
        assert_eq!(online.data_center_hint, Some("default".to_owned()));
        assert_eq!(online.use_proxy_auto_discovery, Some(0));
    }
}
