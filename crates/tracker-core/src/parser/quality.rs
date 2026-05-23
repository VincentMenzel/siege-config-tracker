use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct QualitySection {
    #[serde(rename = "OverallQualityLevelName")]
    pub overall_quality_level_name: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomQualitySection {
    #[serde(rename = "AntiAliasing")]
    pub anti_aliasing: Option<i32>,
    #[serde(rename = "Atmospheric")]
    pub atmospheric: Option<i32>,
    #[serde(rename = "Geometry")]
    pub geometry: Option<i32>,
    #[serde(rename = "Lighting")]
    pub lighting: Option<i32>,
    #[serde(rename = "Shadow")]
    pub shadow: Option<i32>,
    #[serde(rename = "Sharpness")]
    pub sharpness: Option<i32>,
    #[serde(rename = "Texture")]
    pub texture: Option<i32>,
    #[serde(rename = "VFX")]
    pub vfx: Option<i32>,
    #[serde(rename = "TextureFiltering")]
    pub texture_filtering: Option<i32>,
    #[serde(rename = "Reflection")]
    pub reflection: Option<i32>,
    #[serde(rename = "AO")]
    pub ao: Option<i32>,
    #[serde(rename = "LensEffects")]
    pub lens_effects: Option<i32>,
    #[serde(rename = "DOF")]
    pub dof: Option<i32>,
    #[serde(rename = "AdaptiveRenderScalingTargetFPS")]
    pub adaptive_render_scaling_target_fps: Option<i32>,
    #[serde(rename = "RenderScalingFactor")]
    pub render_scaling_factor: Option<i32>,
    #[serde(rename = "DLSSPerfQual")]
    pub dlss_perf_qual: Option<i32>,
    #[serde(rename = "FSR2PerfQual")]
    pub fsr2_perf_qual: Option<i32>,
    #[serde(rename = "FSRPerfQual")]
    pub fsr_perf_qual: Option<i32>,
    #[serde(rename = "TextureStreaming")]
    pub texture_streaming: Option<i32>,
    #[serde(rename = "TextureVRAMLimit")]
    pub texture_vram_limit: Option<i32>,
}
