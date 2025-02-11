use std::fmt::{self, Display, Formatter};

pub struct TranslationKey(String);

impl Display for TranslationKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
