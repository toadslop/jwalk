use super::locale::Locale;
use rust_i18n::t;
use serde::Deserialize;
use std::{
    borrow::Cow,
    fmt::{self, Display, Formatter},
};

#[derive(Debug, Clone, Deserialize)]
pub struct TranslationKey(String);

impl TranslationKey {
    pub fn new(key: impl Into<String>) -> Self {
        Self(key.into())
    }

    pub fn not_found() -> Self {
        Self::new("not-found")
    }
}

impl AsRef<str> for TranslationKey {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl TranslationKey {
    pub fn as_locale(&self, locale: Locale) -> Cow<'_, str> {
        t!(self.as_ref(), locale = locale.as_ref())
    }
}

impl Display for TranslationKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
