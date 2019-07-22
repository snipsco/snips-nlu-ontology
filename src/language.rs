use crate::language_enum;
use failure::bail;

language_enum!([DE, EN, ES, FR, IT, PT_PT, PT_BR, JA, KO]);

impl Language {
    pub fn full_name(&self) -> &'static str {
        match *self {
            Language::DE => "German",
            Language::EN => "English",
            Language::ES => "Spanish",
            Language::FR => "French",
            Language::IT => "Italian",
            Language::PT_PT => "Portuguese - Europe",
            Language::PT_BR => "Portuguese - Brazil",
            Language::JA => "Japanese",
            Language::KO => "Korean",
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn init_from_lowercased_string_works() {
        let lang = Language::from_str("en");
        assert!(lang.is_ok());
    }

    #[test]
    fn init_from_uppercased_string_works() {
        let lang = Language::from_str("EN");
        assert!(lang.is_ok());
    }
}
