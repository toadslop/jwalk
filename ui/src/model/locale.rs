use std::str::FromStr;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SupportedLocale {
    #[default]
    EnUs,
    JpJa,
}

impl FromStr for SupportedLocale {
    type Err = ParseLocaleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let locale = match s {
            "en" | "en-US" => Self::EnUs,
            "jp" | "jp-JA" => Self::JpJa,
            other => Err(ParseLocaleError::InvalidKey(other.to_string()))?,
        };

        Ok(locale)
    }
}

impl std::fmt::Display for SupportedLocale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let it = match self {
            SupportedLocale::EnUs => "en-US",
            SupportedLocale::JpJa => "jp-JA",
        };

        write!(f, "{it}")
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ParseLocaleError {
    #[error("Received invalid locale key: {0}")]
    InvalidKey(String),
}
