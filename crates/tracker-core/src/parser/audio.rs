use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AudioSection {
    #[serde(rename = "SubtitleType")]
    pub subtitle_type: Option<u8>,
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<u8>,
    #[serde(rename = "MasterVolume")]
    pub master_volume: Option<f64>,
    #[serde(rename = "MenuMusicVolume")]
    pub menu_music_volume: Option<f64>,
    #[serde(rename = "InGameMusicVolume")]
    pub in_game_music_volume: Option<f64>,
    #[serde(rename = "MenuSFXVolume")]
    pub menu_sfx_volume: Option<f64>,
    #[serde(rename = "InGameSFXVolume")]
    pub in_game_sfx_volume: Option<f64>,
    #[serde(rename = "VoiceVolume")]
    pub voice_volume: Option<f64>,
    #[serde(rename = "DynamicRangeMode")]
    pub dynamic_range_mode: Option<u8>,
    #[serde(rename = "MonoOutput")]
    pub mono_output: Option<u8>,
    #[serde(rename = "VoiceChatPlaybackLevel")]
    pub voice_chat_playback_level: Option<u32>,
    #[serde(rename = "VoiceChatCaptureThresholdV2")]
    pub voice_chat_capture_threshold_v2: Option<u32>,
    #[serde(rename = "VoiceChatCaptureMode")]
    pub voice_chat_capture_mode: Option<u8>,
    #[serde(rename = "VoiceChatCaptureLevel")]
    pub voice_chat_capture_level: Option<u32>,
    #[serde(rename = "Mute")]
    pub mute: Option<u8>,
    #[serde(rename = "VoiceChatMuteAll")]
    pub voice_chat_mute_all: Option<u8>,
    #[serde(rename = "VoiceChatTeamOnly")]
    pub voice_chat_team_only: Option<u8>,
    #[serde(rename = "AudioInputVoiceChatDevice")]
    pub audio_input_voice_chat_device: Option<i32>,
    #[serde(rename = "AudioOutputVoiceChatDevice")]
    pub audio_output_voice_chat_device: Option<i32>,
    #[serde(rename = "AudioOutputDevice")]
    pub audio_output_device: Option<i32>,
}
