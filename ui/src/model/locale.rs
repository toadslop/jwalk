use strum::{AsRefStr, EnumString};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, EnumString, AsRefStr)]
pub enum Locale {
    #[default]
    #[strum(serialize = "en", serialize = "en-US")]
    EnUs,
    #[strum(serialize = "jp", serialize = "jp-JA")]
    JpJa,
}
