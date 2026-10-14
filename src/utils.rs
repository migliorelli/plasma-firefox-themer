use std::{fs, io::ErrorKind, path::PathBuf};

pub const BEGIN: &str = "/* DO NOT REMOVE - BEGIN plasma-firefox-themer */";
pub const END: &str = "/* DO NOT REMOVE - END plasma-firefox-themer */";

pub fn read_or_empty(path: &PathBuf) -> Result<String, String> {
    match fs::read_to_string(path) {
        Ok(s) => Ok(s),
        Err(e) if e.kind() == ErrorKind::NotFound => Ok(String::new()),
        Err(e) => Err(format!("Couldn't read {}: {}", path.display(), e)),
    }
}

pub fn strip_block(content: &str) -> String {
    match (content.find(BEGIN), content.find(END)) {
        (Some(start), Some(end)) if start < end => {
            let after = &content[end + END.len()..];
            format!("{}{}", &content[..start], after.trim_start_matches('\n'))
        }
        _ => content.to_string(),
    }
}

/// AI generated
/// accept these color formats:
/// "r,g,b" / "r,g,b,a"
/// "#rrggbb"
/// "#aarrggbb" (formato Qt — alpha vem primeiro)
pub fn parse_color(raw: &str) -> Result<(u8, u8, u8), String> {
    let raw = raw.trim();
    let err = || format!("Invalid color value: '{}'", raw);

    if let Some(hex) = raw.strip_prefix('#') {
        let hex = match hex.len() {
            6 => hex,
            8 => hex.get(2..).ok_or_else(err)?, // descarta o alpha
            _ => return Err(err()),
        };
        let v = u32::from_str_radix(hex, 16).map_err(|_| err())?;
        Ok(((v >> 16) as u8, (v >> 8) as u8, v as u8))
    } else {
        let parts = raw
            .split(',')
            .map(|p| p.trim().parse::<u8>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| err())?;
        if parts.len() < 3 {
            return Err(err());
        }
        Ok((parts[0], parts[1], parts[2]))
    }
}