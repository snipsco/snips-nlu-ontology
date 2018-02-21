use rustling_ontology::Lang as RustlingLang;

macro_rules! language_enum {
    ([$($language:ident),*]) => {
        #[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Hash, Eq)]
        pub enum Language {
            $( $language, )*
        }

        impl Language {
            pub fn all() -> &'static [Language] {
                static ALL: &[Language] = &[$( Language::$language ),*];
                ALL
            }
        }
    }
}

language_enum!([DE, EN, ES, FR, KO]);

impl ::std::str::FromStr for Language {
    type Err = String;
    fn from_str(it: &str) -> Result<Self, Self::Err> {
        match &*it.to_lowercase() {
            "en" => Ok(Language::EN),
            "fr" => Ok(Language::FR),
            "es" => Ok(Language::ES),
            "ko" => Ok(Language::KO),
            "de" => Ok(Language::DE),
            _ => Err(format!("Unknown language {}", it)),
        }
    }
}

impl ::std::string::ToString for Language {
    fn to_string(&self) -> String {
        match *self {
            Language::EN => "en",
            Language::FR => "fr",
            Language::ES => "es",
            Language::KO => "ko",
            Language::DE => "de",
        }.to_string()
    }
}

impl From<Language> for RustlingLang {
    fn from(lang: Language) -> Self {
        match lang {
            Language::DE => RustlingLang::DE,
            Language::EN => RustlingLang::EN,
            Language::ES => RustlingLang::ES,
            Language::FR => RustlingLang::FR,
            Language::KO => RustlingLang::KO,
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
