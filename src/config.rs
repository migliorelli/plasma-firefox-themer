use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use ini::Ini;

#[derive(Default, Debug)]
pub struct AppConfig {
    pub profiles: HashMap<String, PathBuf>
}

impl AppConfig {
    fn get_config_path() -> Result<PathBuf, String> {
        let config_dir =
            dirs::config_dir()
                .ok_or_else(|| "Couldn't find user's config directory.")?
                .join("plasma-firefox-themer");

        fs::create_dir_all(&config_dir).map_err(|err| {
           err.to_string()
        })?;

        Ok(config_dir.join("config.ini"))
    }

    pub fn load() -> Self {
        let mut config = Self::default();

        let Ok(path) = Self::get_config_path() else {
          return config
        };

        if !path.exists() {
            return config;
        }

        if let Ok(ini) = Ini::load_from_file(&path) {
            for (section, prop) in &ini {
                if let Some(section_name) = section {
                    if let Some(profile_name) = section_name.strip_prefix("Profile::") {
                        if let Some(profile_path) = prop.get("path") {
                            config.profiles.insert(profile_name.to_string(),PathBuf::from(profile_path));
                        }
                    }
                }
            }
        }

        config
    }

    pub fn save(&self) -> Result<(), String> {
        let config_path = Self::get_config_path()?;
        let mut ini = Ini::new();

        for (profile_name, profile_path) in &self.profiles {
            let section_title = format!("Profile::{}", profile_name);
            ini.with_section(Some(section_title)).set("path", profile_path.to_string_lossy());
        }

        ini.write_to_file(config_path).map_err(|e| e.to_string())?;
        Ok(())
    }
}