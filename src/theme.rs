use ini::Ini;
use std::collections::HashMap;
use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;
use crate::utils::{parse_color, read_or_empty, strip_block, END, BEGIN};

const CSS_TEMPLATE: &str = include_str!("../assets/userChrome.template.css");
const THEME_FILE: &str = "plasma-firefox-themer.css";

/// (theme key, color scheme section, color scheme prop)
/// this and .template was AI generated
const COLOR_MAP: &[(&str, &str, &str)] = &[
    // Colors:Window — toolbars, painéis, menus, sidebar
    ("window_bg", "Colors:Window", "BackgroundNormal"),
    ("window_bg_alt", "Colors:Window", "BackgroundAlternate"), // bordas/separadores
    ("window_fg", "Colors:Window", "ForegroundNormal"),
    ("window_fg_inactive", "Colors:Window", "ForegroundInactive"),
    ("link", "Colors:Window", "ForegroundLink"),
    ("visited", "Colors:Window", "ForegroundVisited"),
    // Colors:View — campo de URL, fundo da página, popups de autocomplete
    ("view_bg", "Colors:View", "BackgroundNormal"),
    ("view_fg", "Colors:View", "ForegroundNormal"),
    ("view_fg_inactive", "Colors:View", "ForegroundInactive"),
    // Colors:Button — aba selecionada, botões
    ("button_bg", "Colors:Button", "BackgroundNormal"),
    ("button_fg", "Colors:Button", "ForegroundNormal"),
    // // Colors:Complementary — destaque/acento (item selecionado, foco)
    // ("complementary_bg", "Colors:Complementary", "BackgroundNormal"),
    // ("complementary_fg", "Colors:Complementary", "ForegroundNormal"),
    // Colors:Header — titlebar / barra de abas
    ("header_bg", "Colors:Header", "BackgroundNormal"),
    ("header_fg", "Colors:Header", "ForegroundNormal"),
    ("header_fg_inactive", "Colors:Header", "ForegroundInactive"),
    // Colors:Tooltip
    ("tooltip_bg", "Colors:Tooltip", "BackgroundNormal"),
    ("tooltip_fg", "Colors:Tooltip", "ForegroundNormal"),
];

#[derive(Default, Debug)]
pub struct Theme {
    colors: HashMap<String, String>,
}

impl Theme {
    pub fn load() -> Result<Self, String> {
        let mut theme = Self::default();

        let kdeglobals_path = dirs::config_dir()
            .ok_or_else(|| "Couldn't find user's config directory.".to_string())?
            .join("kdeglobals");

        let ini_kdeglobals = Ini::load_from_file(&kdeglobals_path)
            .map_err(|e| format!("Couldn't read {}: {}", kdeglobals_path.display(), e))?;

        let colorscheme_name = ini_kdeglobals
            .section(Some("General"))
            .and_then(|s| s.get("ColorScheme"))
            .ok_or_else(|| "kdeglobals has no [General] ColorScheme.".to_string())?;

        let file_name = format!("{}.colors", colorscheme_name);
        let mut candidates: Vec<PathBuf> = Vec::new();
        if let Some(data) = dirs::data_dir() {
            candidates.push(data.join("color-schemes").join(&file_name));
        }

        candidates.push(PathBuf::from("/usr/share/color-schemes").join(&file_name));

        let colorscheme = candidates
            .iter()
            .find(|p| p.exists())
            .ok_or_else(|| format!("Color scheme file '{}' not found.", file_name))?;

        let ini_colorscheme = Ini::load_from_file(colorscheme)
            .map_err(|e| format!("Couldn't read {}: {}", colorscheme.display(), e))?;

        for (name, section, key) in COLOR_MAP {
            let raw = ini_colorscheme
                .section(Some(*section))
                .and_then(|s| s.get(*key))
                .ok_or_else(|| format!("Missing [{}] {} in {}.", section, key, file_name))?;

            let (r, g, b) = parse_color(raw)?;
            theme
                .colors
                .insert(name.to_string(), format!("#{:02x}{:02x}{:02x}", r, g, b));
        }

        theme
            .colors
            .insert("scheme".to_string(), colorscheme_name.to_string());

        // set dark or light theme
        let (r, g, b) = parse_color(&theme.colors["window_bg"])?;
        let luminance = 0.2126 * r as f32 + 0.7152 * g as f32 + 0.0722 * b as f32;
        theme.colors.insert(
            "color_scheme".to_string(),
            if luminance < 128.0 { "dark" } else { "light" }.to_string(),
        );

        Ok(theme)
    }

    // replace {{key}} on .template file
    fn render(&self, template: &str) -> String {
        self.colors.iter().fold(template.to_string(), |css, (k, v)| {
            css.replace(&format!("{{{{{}}}}}", k), v)
        })
    }

    pub fn apply_theme(&mut self, profile: &PathBuf) -> Result<(), String> {
        let chrome_dir = profile.join("chrome");
        fs::create_dir_all(&chrome_dir)
            .map_err(|e| format!("Coudln't create {}: {}", chrome_dir.display(), e))?;

        let theme_path = chrome_dir.join(THEME_FILE);
        fs::write(&theme_path, self.render(CSS_TEMPLATE))
            .map_err(|e| format!("Couldn't write {}: {}", theme_path.display(), e))?;

        let userchrome_path = chrome_dir.join("userChrome.css");
        let current = read_or_empty(&userchrome_path)?;
        let cleaned = strip_block(&current);
        let updated = format!("{BEGIN}\n@import \"{THEME_FILE}\";\n{END}\n{cleaned}");

        fs::write(&userchrome_path, updated).
            map_err(|e| format!("Couldn't write {}: {}", userchrome_path.display(), e))
    }

    pub fn remove_theme(&mut self, profile: &PathBuf) -> Result<(), String> {
        let chrome_dir = profile.join("chrome");

        let theme_path = chrome_dir.join(THEME_FILE);
        match fs::remove_file(&theme_path) {
            Ok(()) => {}
            Err(e) if e.kind() == ErrorKind::NotFound => {}
            Err(e) => return Err(format!("Couldn't remove {}: {}", theme_path.display(), e)),
        }

        let userchrome_path = chrome_dir.join("userChrome.css");
        let current = read_or_empty(&userchrome_path)?;
        let cleaned = strip_block(&current);

        if cleaned.trim().is_empty() {
            let _ = fs::remove_file(&userchrome_path); // empty file
        } else {
            fs::write(&userchrome_path, cleaned).
                map_err(|e| format!("Couldn't write {}: {}", userchrome_path.display(), e))?;
        }

        Ok(())
    }
}

