//! Application-specific settings mapping built on the reusable file store.
//!
//! Unknown keys are ignored for forward compatibility. Unknown values fall
//! back to a safe default and an incompatible format version is not applied.
use crate::{
    backdrop::Backdrop,
    design::{Appearance, Density, MaterialClarity},
    foundation::settings_store::SettingsStore,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Preferences {
    pub appearance: Appearance,
    pub density: Density,
    pub backdrop: Backdrop,
    pub clarity: MaterialClarity,
    pub show_details: bool,
    pub animations: bool,
}

impl Default for Preferences {
    fn default() -> Self {
        Self {
            appearance: Appearance::Light,
            density: Density::Comfortable,
            backdrop: Backdrop::auto(),
            clarity: MaterialClarity::Crisp,
            show_details: true,
            animations: true,
        }
    }
}

impl Preferences {
    pub fn parse(contents: &str) -> Self {
        let mut result = Self::default();
        if !contents.lines().any(|line| line.trim() == "version=1") {
            return result;
        }
        for line in contents.lines() {
            let Some((key, value)) = line.trim().split_once('=') else {
                continue;
            };
            let value = value.trim();
            match key.trim() {
                "appearance" => {
                    result.appearance = match value {
                        "light" => Appearance::Light,
                        "dark" => Appearance::Dark,
                        _ => result.appearance,
                    };
                }
                "density" => {
                    result.density = match value {
                        "comfortable" => Density::Comfortable,
                        "compact" => Density::Compact,
                        _ => result.density,
                    };
                }
                "backdrop" => {
                    result.backdrop = match value {
                        "opaque" => Backdrop::Opaque,
                        "mica" => Backdrop::Mica,
                        "mica-alt" => Backdrop::MicaAlt,
                        "acrylic" => Backdrop::Acrylic,
                        "clear" => Backdrop::Clear,
                        _ => result.backdrop,
                    };
                }
                "clarity" => {
                    result.clarity = match value {
                        "crisp" => MaterialClarity::Crisp,
                        "glass" => MaterialClarity::Glass,
                        "sharp" => MaterialClarity::Sharp,
                        _ => result.clarity,
                    };
                }
                "show_details" => {
                    result.show_details = parse_bool(value).unwrap_or(result.show_details);
                }
                "animations" => {
                    result.animations = parse_bool(value).unwrap_or(result.animations);
                }
                _ => {}
            }
        }
        result
    }

    pub fn encode(self) -> String {
        let appearance = match self.appearance {
            Appearance::Light => "light",
            Appearance::Dark => "dark",
        };
        let density = match self.density {
            Density::Comfortable => "comfortable",
            Density::Compact => "compact",
        };
        format!(
            "version=1\nappearance={appearance}\ndensity={density}\nbackdrop={}\nclarity={}\nshow_details={}\nanimations={}\n",
            self.backdrop.id(),
            self.clarity.id(),
            self.show_details,
            self.animations
        )
    }

    /// Respect platform constraints and the manual opaque-window override,
    /// but retain the requested material in the saved preferences.
    pub fn resolved_backdrop(self) -> Backdrop {
        if self.clarity == MaterialClarity::Sharp
            || std::env::var_os("ARGUS_UI_OPAQUE").is_some()
            || !self.backdrop.is_available()
        {
            Backdrop::Opaque
        } else {
            self.backdrop
        }
    }

    /// Return the preferences, their persistent store, and any startup warning.
    /// Applications still launch with defaults when the user profile is missing
    /// or the file cannot be read.
    pub fn load_user() -> (Self, Option<SettingsStore>, Option<String>) {
        let Some(path) = SettingsStore::user_path("Argus-UI", "ARGUS_UI_CONFIG_DIR") else {
            return (
                Self::default(),
                None,
                Some("没有可用的用户配置目录；本次设置不会保存".to_string()),
            );
        };
        let store = SettingsStore::new(path);
        match store.read() {
            Ok(Some(contents)) => (Self::parse(&contents), Some(store), None),
            Ok(None) => (Self::default(), Some(store), None),
            Err(error) => (
                Self::default(),
                Some(store),
                Some(format!("读取设置失败：{error}")),
            ),
        }
    }
}

fn parse_bool(value: &str) -> Option<bool> {
    match value {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preferences_round_trip() {
        let input = Preferences {
            appearance: Appearance::Dark,
            density: Density::Compact,
            backdrop: Backdrop::MicaAlt,
            clarity: MaterialClarity::Glass,
            show_details: false,
            animations: false,
        };
        assert_eq!(Preferences::parse(&input.encode()), input);
    }

    #[test]
    fn unsupported_version_does_not_apply_any_values() {
        let loaded = Preferences::parse("version=2\nappearance=dark\nshow_details=false\n");
        assert_eq!(loaded, Preferences::default());
    }

    #[test]
    fn invalid_fields_preserve_defaults() {
        let loaded = Preferences::parse(
            "version=1\nappearance=unknown\ndensity=invalid\nshow_details=maybe\nfuture_key=yes\n",
        );
        assert_eq!(loaded, Preferences::default());
    }

    #[test]
    fn valid_fields_do_not_depend_on_file_order() {
        let loaded = Preferences::parse("animations=false\nappearance=dark\nversion=1\n");
        assert_eq!(loaded.appearance, Appearance::Dark);
        assert!(!loaded.animations);
        assert!(loaded.show_details);
    }

    #[test]
    fn opaque_material_is_always_safe() {
        let prefs = Preferences {
            clarity: MaterialClarity::Sharp,
            backdrop: Backdrop::Mica,
            ..Preferences::default()
        };
        assert_eq!(prefs.resolved_backdrop(), Backdrop::Opaque);
    }
}
