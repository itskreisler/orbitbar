use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum PropertyType {
    Color,
    Slider,
    Checkbox,
    Combo,
    #[default]
    TextInput,
    TextArea,
    Number,
    File,
    Directory,
    Date,
    Time,
    DateTime,
    Font,
    KeyBind,
    ColorGradient,
    Vector2,
    Vector3,
    Range,
    Button,
    Section,
    Label,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(untagged)]
pub enum PropertyValue {
    String(String),
    Number(f64),
    Bool(bool),
    Vector2([f64; 2]),
    Vector3([f64; 3]),
    Gradient(Vec<String>),
    Array(Vec<PropertyValue>),
    Map(HashMap<String, serde_json::Value>),
    #[default]
    Null,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComboOption {
    pub label: String,
    pub value: PropertyValue,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PropertyDefinition {
    #[serde(rename = "type")]
    pub type_: PropertyType,

    /// Label or title displayed in the editor
    #[serde(default)]
    pub text: String,

    /// Optional localized text mapping e.g. {"en": "Color", "es": "Color"}
    #[serde(default)]
    pub text_i18n: Option<HashMap<String, String>>,

    /// Detailed description / tooltip
    #[serde(default)]
    pub description: Option<String>,

    /// Default / current property value
    #[serde(default)]
    pub value: PropertyValue,

    /// Range minimum (for slider, number, range)
    #[serde(default)]
    pub min: Option<f64>,

    /// Range maximum (for slider, number, range)
    #[serde(default)]
    pub max: Option<f64>,

    /// Increment step
    #[serde(default)]
    pub step: Option<f64>,

    /// Combo dropdown options
    #[serde(default)]
    pub options: Option<Vec<ComboOption>>,

    /// Conditional visibility expression e.g. "show_stars == true"
    #[serde(default)]
    pub visible_if: Option<String>,

    /// Enabled state
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Whether this property should be persisted
    #[serde(default = "default_true")]
    pub persist: bool,

    /// File filter extensions e.g. ["png", "jpg"]
    #[serde(default)]
    pub file_filter: Option<Vec<String>>,

    /// Additional custom configuration JSON
    #[serde(default)]
    pub custom_config: Option<serde_json::Value>,
}

fn default_true() -> bool {
    true
}

impl PropertyType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Color => "color",
            Self::Slider => "slider",
            Self::Checkbox => "checkbox",
            Self::Combo => "combo",
            Self::TextInput => "textinput",
            Self::TextArea => "textarea",
            Self::Number => "number",
            Self::File => "file",
            Self::Directory => "directory",
            Self::Date => "date",
            Self::Time => "time",
            Self::DateTime => "datetime",
            Self::Font => "font",
            Self::KeyBind => "keybind",
            Self::ColorGradient => "colorgradient",
            Self::Vector2 => "vector2",
            Self::Vector3 => "vector3",
            Self::Range => "range",
            Self::Button => "button",
            Self::Section => "section",
            Self::Label => "label",
            Self::Custom => "custom",
        }
    }
}

impl PropertyDefinition {
    pub fn validate_definition(&self, key: &str) -> Result<(), String> {
        if key.trim().is_empty() {
            return Err("Property key cannot be empty".to_string());
        }

        // Validate range bounds if present
        if let (Some(min), Some(max)) = (self.min, self.max) {
            if min > max {
                return Err(format!(
                    "Property '{key}' min ({min}) cannot be greater than max ({max})"
                ));
            }
        }

        Ok(())
    }

    pub fn default_value(&self) -> PropertyValue {
        if self.value != PropertyValue::Null {
            return self.value.clone();
        }

        match self.type_ {
            PropertyType::Color => PropertyValue::String("#ffffff".to_string()),
            PropertyType::Slider | PropertyType::Number => {
                PropertyValue::Number(self.min.unwrap_or(0.0))
            }
            PropertyType::Checkbox => PropertyValue::Bool(false),
            PropertyType::Combo => self
                .options
                .as_ref()
                .and_then(|opts| opts.first())
                .map(|opt| opt.value.clone())
                .unwrap_or(PropertyValue::String(String::new())),
            PropertyType::TextInput | PropertyType::TextArea => {
                PropertyValue::String(String::new())
            }
            PropertyType::File | PropertyType::Directory => PropertyValue::String(String::new()),
            PropertyType::Date => PropertyValue::String("2025-01-01".to_string()),
            PropertyType::Time => PropertyValue::String("12:00".to_string()),
            PropertyType::DateTime => PropertyValue::String("2025-01-01T12:00:00".to_string()),
            PropertyType::Font => PropertyValue::String("Arial".to_string()),
            PropertyType::KeyBind => PropertyValue::String(String::new()),
            PropertyType::ColorGradient => {
                PropertyValue::Gradient(vec!["#000000".to_string(), "#ffffff".to_string()])
            }
            PropertyType::Vector2 => PropertyValue::Vector2([0.0, 0.0]),
            PropertyType::Vector3 => PropertyValue::Vector3([0.0, 0.0, 0.0]),
            PropertyType::Range => PropertyValue::Vector2([
                self.min.unwrap_or(0.0),
                self.max.unwrap_or(100.0),
            ]),
            PropertyType::Button | PropertyType::Section | PropertyType::Label => PropertyValue::Null,
            PropertyType::Custom => PropertyValue::Null,
        }
    }

    pub fn validate_value(&self, val: &PropertyValue) -> Result<PropertyValue, String> {
        match (&self.type_, val) {
            (PropertyType::Number | PropertyType::Slider, PropertyValue::Number(num)) => {
                let mut sanitized = *num;
                if let Some(min) = self.min {
                    if sanitized < min {
                        sanitized = min;
                    }
                }
                if let Some(max) = self.max {
                    if sanitized > max {
                        sanitized = max;
                    }
                }
                Ok(PropertyValue::Number(sanitized))
            }
            (PropertyType::Checkbox, PropertyValue::Bool(b)) => Ok(PropertyValue::Bool(*b)),
            (PropertyType::Color, PropertyValue::String(hex)) => {
                if hex.starts_with('#') || hex.starts_with("rgb") {
                    Ok(PropertyValue::String(hex.clone()))
                } else {
                    Ok(PropertyValue::String(format!("#{hex}")))
                }
            }
            (PropertyType::TextInput | PropertyType::TextArea, PropertyValue::String(s)) => {
                Ok(PropertyValue::String(s.clone()))
            }
            _ => Ok(val.clone()),
        }
    }
}

/// Formats properties into Wallpaper Engine's expected `applyUserProperties` JSON payload.
/// Format: `{ "prop_name": { "value": val } }`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WallpaperPropertyPayload {
    pub value: PropertyValue,
}

pub fn format_apply_user_properties(
    properties: &HashMap<String, PropertyValue>,
) -> HashMap<String, WallpaperPropertyPayload> {
    let mut payload = HashMap::new();
    for (k, v) in properties {
        payload.insert(k.clone(), WallpaperPropertyPayload { value: v.clone() });
    }
    payload
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_all_22_property_types() {
        let types = vec![
            PropertyType::Color,
            PropertyType::Slider,
            PropertyType::Checkbox,
            PropertyType::Combo,
            PropertyType::TextInput,
            PropertyType::TextArea,
            PropertyType::Number,
            PropertyType::File,
            PropertyType::Directory,
            PropertyType::Date,
            PropertyType::Time,
            PropertyType::DateTime,
            PropertyType::Font,
            PropertyType::KeyBind,
            PropertyType::ColorGradient,
            PropertyType::Vector2,
            PropertyType::Vector3,
            PropertyType::Range,
            PropertyType::Button,
            PropertyType::Section,
            PropertyType::Label,
            PropertyType::Custom,
        ];

        assert_eq!(types.len(), 22);
        for t in types {
            let prop = PropertyDefinition {
                type_: t.clone(),
                text: format!("Test {}", t.as_str()),
                ..Default::default()
            };
            assert!(prop.validate_definition("test_key").is_ok());
            let default_val = prop.default_value();
            assert!(
                default_val != PropertyValue::Null || matches!(t, PropertyType::Button | PropertyType::Section | PropertyType::Label | PropertyType::Custom)
            );
        }
    }

    #[test]
    fn formats_wallpaper_engine_payload() {
        let mut props = HashMap::new();
        props.insert("customcolor".to_string(), PropertyValue::String("#ff0000".to_string()));
        props.insert("intensity".to_string(), PropertyValue::Number(85.0));

        let formatted = format_apply_user_properties(&props);
        assert_eq!(formatted.len(), 2);
        assert_eq!(
            formatted.get("customcolor").unwrap().value,
            PropertyValue::String("#ff0000".to_string())
        );
    }
}
