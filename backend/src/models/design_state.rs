use serde::{Deserialize, Serialize};

/// Root structure for the canvas design state stored as JSONB.
/// This is the core data model — the "Canva engine" of Digitaria.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignState {
    pub canvas: CanvasSettings,
    pub layers: Vec<Layer>,
    pub config: DesignConfig,
}

/// Global canvas settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CanvasSettings {
    pub width: u32,
    pub height: u32,
    pub background_color: String,
    pub global_font: String,
    pub theme_color: String,
}

/// A single element/layer on the canvas.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layer {
    pub id: String,
    #[serde(rename = "type")]
    pub layer_type: LayerType,
    /// Text content (for text layers) or URL (for image layers).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    pub props: LayerProps,
}

/// Supported layer/element types on the canvas.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum LayerType {
    Text,
    Image,
    Shape,
    Video,
}

/// Visual properties of a layer (position, style, etc).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayerProps {
    pub x: f64,
    pub y: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,

    // Text-specific
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_weight: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_align: Option<String>,

    // Transform
    #[serde(default)]
    pub rotation: f64,
    #[serde(default = "default_opacity")]
    pub opacity: f64,
    #[serde(default)]
    pub z_index: i32,

    // Image-specific
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_radius: Option<f64>,
}

fn default_opacity() -> f64 {
    1.0
}

/// Behavioral configuration for the invitation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DesignConfig {
    #[serde(default)]
    pub auto_play_music: bool,
    #[serde(default = "default_show_watermark")]
    pub show_watermark: bool,
}

fn default_show_watermark() -> bool {
    true // Free tier shows watermark by default
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_design_state() {
        let json = r##"{
            "canvas": {
                "width": 1080,
                "height": 1920,
                "backgroundColor": "#FFFFFF",
                "globalFont": "Inter",
                "themeColor": "#D4AF37"
            },
            "layers": [
                {
                    "id": "elem_1",
                    "type": "text",
                    "content": "The Wedding of",
                    "props": {
                        "x": 540,
                        "y": 200,
                        "fontSize": 24,
                        "fontWeight": "bold",
                        "fontFamily": "Playfair Display",
                        "color": "#000000",
                        "textAlign": "center",
                        "rotation": 0,
                        "opacity": 1,
                        "zIndex": 1
                    }
                }
            ],
            "config": {
                "autoPlayMusic": true,
                "showWatermark": false
            }
        }"##;

        let state: DesignState = serde_json::from_str(json).unwrap();
        assert_eq!(state.canvas.width, 1080);
        assert_eq!(state.layers.len(), 1);
        assert_eq!(state.layers[0].layer_type, LayerType::Text);
        assert!(!state.config.show_watermark);
    }
}
