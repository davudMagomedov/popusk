/// The `style` module contains all the types which can be used to describe the style of the text.

use once_cell::sync::OnceCell;
use mlua::{FromLua, Value as LuaValue, Lua};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Default,
    RGB([u8; 3]),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StyleTag {
    pub fore: Color,
    pub back: Color,
    /// =0bBIU, where B is 'Bold', I is 'Italic', U is 'Underlined'.
    pub biu: u8,
}

impl StyleTag {
    const DEFAULT: Self = StyleTag {
        fore: Color::Default,
        back: Color::Default,
        biu: 0,
    };
}

#[derive(Debug, Clone)]
pub struct TextBurst {
    pub text: String,
    pub style_tag: StyleTag,
}

#[derive(Debug, Clone)]
pub struct StyledText(Vec<TextBurst>);

impl FromLua for StyledText {
    fn from_lua(value: LuaValue, _lua: &Lua) -> mlua::Result<Self> {
        let LuaValue::String(s) = value else {
            return Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "styled_text",
                message: Some("expected string".into()),
            });
        };

        Ok(styled_text_from_string(&s.to_string_lossy()))
    }
}

/// If there's a buf in this regular expression I will go through the window.
const TEXTBURST_REGEX_STRING: &str = r"\[\[(?:(1?[0-9]{1,2}|2[0-4][0-9]|25[0-5])\:(1?[0-9]{1,2}|2[0-4][0-9]|25[0-5])\:(1?[0-9]{1,2}|2[0-4][0-9]|25[0-5]))?\|(?:(1?[0-9]{1,2}|2[0-4][0-9]|25[0-5])\:(1?[0-9]{1,2}|2[0-4][0-9]|25[0-5])\:(1?[0-9]{1,2}|2[0-4][0-9]|25[0-5]))?\|([01]{3})?\]\]\{\{((?:.|\n)*?)\}\}";
static TEXTBURST_REGEX: OnceCell<regex::Regex> = OnceCell::new();

fn parse_captures_into_textburst(captures: regex::Captures) -> TextBurst {
    let fore = match captures.get(1) {
        Some(fore_r_match) => {
            // SAFETY: the regular expression parses only u8.
            let fore_r = unsafe { fore_r_match.as_str().parse::<u8>().unwrap_unchecked() };
            let fore_g = unsafe {
                captures
                    .get(2)
                    // SAFETY: `captures.get(1).is_some()` => `captures.get(2|3).is_some()`.
                    .unwrap_unchecked()
                    .as_str()
                    .parse::<u8>()
                    // SAFETY: the regular expression parses only u8.
                    .unwrap_unchecked()
            };
            let fore_b = unsafe {
                captures
                    .get(3)
                    // SAFETY: `captures.get(1).is_some()` => `captures.get(2|3).is_some()`.
                    .unwrap_unchecked()
                    .as_str()
                    .parse::<u8>()
                    // SAFETY: the regular expression parses only u8.
                    .unwrap_unchecked()
            };
            Color::RGB([fore_r, fore_g, fore_b])
        }
        None => Color::Default,
    };
    let back = match captures.get(4) {
        Some(fore_r_match) => {
            // SAFETY: the regular expression parses only u8.
            let back_r = unsafe { fore_r_match.as_str().parse::<u8>().unwrap_unchecked() };
            let back_g = unsafe {
                captures
                    .get(5)
                    // SAFETY: `captures.get(4).is_some()` => `captures.get(5|6).is_some()`.
                    .unwrap_unchecked()
                    .as_str()
                    .parse::<u8>()
                    // SAFETY: the regular expression parses only u8.
                    .unwrap_unchecked()
            };
            let back_b = unsafe {
                captures
                    .get(6)
                    // SAFETY: `captures.get(4).is_some()` => `captures.get(5|6).is_some()`.
                    .unwrap_unchecked()
                    .as_str()
                    .parse::<u8>()
                    // SAFETY: the regular expression parses only u8.
                    .unwrap_unchecked()
            };
            Color::RGB([back_r, back_g, back_b])
        }
        None => Color::Default,
    };
    let biu = (&captures[7][0..1] == "1") as u8 * 4
        + (&captures[7][1..2] == "1") as u8 * 2
        + (&captures[7][2..3] == "1") as u8;

    TextBurst {
        style_tag: StyleTag { fore, back, biu },
        text: captures[8].to_string(),
    }
}

/// Uses special syntax to turn regular text into stylyzed one.
pub fn styled_text_from_string(s: &str) -> StyledText {
    let reg = TEXTBURST_REGEX.get_or_init(
        || unsafe { regex::Regex::new(TEXTBURST_REGEX_STRING).unwrap_unchecked() },
    );
    let mut styled_text = Vec::new();

    let mut previous_end = 0;
    for captures in reg.captures_iter(s) {
        let capt_match = unsafe { captures.get(0).unwrap_unchecked() };
        let (captures_start, captures_end) = (capt_match.start(), capt_match.end());
        if previous_end != captures_start {
            styled_text.push(TextBurst {
                style_tag: StyleTag::DEFAULT,
                text: s[previous_end..captures_start].into(),
            });
        }
        styled_text.push(parse_captures_into_textburst(captures));
        previous_end = captures_end;
    }

    if previous_end != s.len() {
        styled_text.push(TextBurst {
            style_tag: StyleTag::DEFAULT,
            text: s[previous_end..].into(),
        });
    }

    StyledText(styled_text)
}
