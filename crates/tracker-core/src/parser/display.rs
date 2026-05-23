use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DisplaySection {
    #[serde(rename = "GPUInfo")]
    pub gpu_info: Option<i32>,
    #[serde(rename = "GPUAdapter")]
    pub gpu_adapter: Option<u32>,
    #[serde(rename = "GPUAdapterInfo")]
    pub gpu_adapter_info: Option<u32>,
    #[serde(rename = "GPUAdapterSelectMode")]
    pub gpu_adapter_select_mode: Option<u8>,
    #[serde(rename = "InitialWindowPositionX")]
    pub initial_window_position_x: Option<i32>,
    #[serde(rename = "InitialWindowPositionY")]
    pub initial_window_position_y: Option<i32>,
    #[serde(rename = "Brightness")]
    pub brightness: Option<f64>,
    #[serde(rename = "FPSLimit")]
    pub fps_limit: Option<u32>,
    #[serde(rename = "NVReflex")]
    pub nv_reflex: Option<u8>,
    #[serde(rename = "NVReflexIndicator")]
    pub nv_reflex_indicator: Option<u8>,
    #[serde(rename = "Console")]
    pub console: Option<u8>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct HardwareInfoSection {
    #[serde(rename = "GPUVendor")]
    pub gpu_vendor: Option<String>,
    #[serde(rename = "GPUDeviceId")]
    pub gpu_device_id: Option<String>,
    #[serde(rename = "GPUSubSysId")]
    pub gpu_sub_sys_id: Option<String>,
    #[serde(rename = "GPUDedicatedMemoryMB")]
    pub gpu_dedicated_memory_mb: Option<u32>,
    #[serde(rename = "GPUScore")]
    pub gpu_score: Option<f64>,
    #[serde(rename = "GPUScoreConf")]
    pub gpu_score_conf: Option<f64>,
    #[serde(rename = "SystemMemoryMB")]
    pub system_memory_mb: Option<u32>,
    #[serde(rename = "CPUScore")]
    pub cpu_score: Option<f64>,
    #[serde(rename = "HardwareNotificationEnable")]
    pub hardware_notification_enable: Option<u8>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DisplaySettingsSection {
    #[serde(rename = "Monitor")]
    pub monitor: Option<u32>,
    #[serde(rename = "ResolutionWidth")]
    pub resolution_width: Option<u32>,
    #[serde(rename = "ResolutionHeight")]
    pub resolution_height: Option<u32>,
    #[serde(rename = "RefreshRate")]
    pub refresh_rate: Option<f64>,
    #[serde(rename = "WindowMode")]
    pub window_mode: Option<u8>,
    #[serde(rename = "AspectRatio")]
    pub aspect_ratio: Option<u8>,
    #[serde(rename = "VSync")]
    pub vsync: Option<u8>,
    #[serde(rename = "MaxGPUBufferedFrame")]
    pub max_gpu_buffered_frame: Option<u32>,
    #[serde(rename = "UseLetterbox")]
    pub use_letterbox: Option<u8>,
    #[serde(rename = "DefaultFOV")]
    pub default_fov: Option<f64>,
    #[serde(rename = "EnableAMDMultiDraw")]
    pub enable_amd_multi_draw: Option<u8>,
    #[serde(rename = "EnableIntelMultiDraw")]
    pub enable_intel_multi_draw: Option<u8>,
    #[serde(rename = "UseAmdAGS")]
    pub use_amd_ags: Option<u8>,
    #[serde(rename = "VulkanWhitelistedLayers")]
    pub vulkan_whitelisted_layers: Option<String>,
}
