use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GeneralSection {
    #[serde(rename = "Version")]
    pub version: Option<u8>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ReadOnlySection {
    #[serde(rename = "EngineSettingsVersion")]
    pub engine_settings_version: Option<u32>,
    #[serde(rename = "DefaultValuesVersion")]
    pub default_values_version: Option<u32>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GameplaySection {
    #[serde(rename = "GameplayPingEnable")]
    pub gameplay_ping_enable: Option<u8>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessibilitySection {
    #[serde(rename = "AccessibilityColorMode")]
    pub accessibility_color_mode: Option<u8>,
    #[serde(rename = "PositiveColorIndex")]
    pub positive_color_index: Option<u32>,
    #[serde(rename = "NegativeColorIndex")]
    pub negative_color_index: Option<u32>,
    #[serde(rename = "ObjectiveColorIndex")]
    pub objective_color_index: Option<u32>,
    #[serde(rename = "PingColorIndex")]
    pub ping_color_index: Option<u32>,
    #[serde(rename = "TeamColorAllyIndex")]
    pub team_color_ally_index: Option<u32>,
    #[serde(rename = "TeamColorEnemyIndex")]
    pub team_color_enemy_index: Option<u32>,
    #[serde(rename = "StunVFXMode")]
    pub stun_vfx_mode: Option<u8>,
    #[serde(rename = "TinnitusSFXMode")]
    pub tinnitus_sfx_mode: Option<u8>,
    #[serde(rename = "HearingFatigueAid")]
    pub hearing_fatigue_aid: Option<u8>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct OnlineSection {
    #[serde(rename = "DataCenterHint")]
    pub data_center_hint: Option<String>,
    #[serde(rename = "UseProxyAutoDiscovery")]
    pub use_proxy_auto_discovery: Option<u8>,
}
