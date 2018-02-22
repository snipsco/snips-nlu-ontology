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

        impl ::std::str::FromStr for Language {
            type Err=String;
            fn from_str(s: &str) -> ::std::result::Result<Language, Self::Err> {
                match &*s.to_uppercase() {
                    $(
                        stringify!($language) => Ok(Language::$language),
                    )*
                    _ => Err(format!("Unknown language: {}", s))
                }
            }
        }

        impl ::std::string::ToString for Language {
            fn to_string(&self) -> String {
                match self {
                    $(
                        &Language::$language => stringify!($language).to_lowercase(),
                    )*
                }
            }
        }

        impl From<Language> for RustlingLang {
            fn from(lang: Language) -> Self {
                match lang {
                    $(
                        Language::$language => RustlingLang::$language,
                    )*
                }
            }
        }

    }
}

language_enum!([DE, EN, ES, FR, KO]);

impl Language {
    pub fn full_name(&self) -> &'static str {
        match *self {
            Language::EN => "English",
            Language::FR => "French",
            Language::ES => "Spanish",
            Language::KO => "Korean",
            Language::DE => "German",
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
