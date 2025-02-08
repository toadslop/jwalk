use std::str::FromStr;

use leptos::logging::warn;
use web_sys::{Navigator, Window};

#[derive(Debug, Clone, Default)]
pub struct Context {
    pub locale: SupportedLocale,
}

// TODO: need to make this isomorphic -- and invoking on the server side, should be possible to
// pass in a locale which could be extracted from a path param
impl Context {
    pub fn init() -> Self {
        let locale = web_sys::window()
            .as_ref()
            .map(Window::navigator)
            .as_ref()
            .and_then(Navigator::language)
            .as_deref()
            .map(SupportedLocale::from_str);

        let locale = if let Some(locale) = locale {
            locale
        } else {
            warn!("No locale extracted from browser. Defaulting to en-US");
            Ok(SupportedLocale::default())
        };

        let locale = match locale {
            Ok(locale) => locale,
            Err(err) => {
                warn!("{err}. Using default locale.");
                SupportedLocale::default()
            }
        };

        Self { locale }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum SupportedLocale {
    #[default]
    EnUs,
    JpJa,
}

impl FromStr for SupportedLocale {
    type Err = ParseLocaleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let locale = match s {
            "en-US" => Self::EnUs,
            "jp-JA" => Self::JpJa,
            other => Err(ParseLocaleError::InvalidKey(other.to_string()))?,
        };

        Ok(locale)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ParseLocaleError {
    #[error("Received invalid locale key: {0}")]
    InvalidKey(String),
}
