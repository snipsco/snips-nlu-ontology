use failure::bail;

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
            type Err=::failure::Error;
            fn from_str(s: &str) -> ::std::result::Result<Language, Self::Err> {
                match &*s.to_uppercase() {
                    $(
                        stringify!($language) => Ok(Language::$language),
                    )*
                    _ => bail!("Unknown language: {}", s)
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
    }
}

language_enum!([D, EN, ES, FR, IT, PT, JA, KO]);

impl Language {
    pub fn full_name(&self) -> &'static str {
        match *self {
            Language::DE => "German",
            Language::EN => "English",
            Language::ES => "Spanish",
            Language::FR => "French",
            Language::IT => "Italian",
            Language::PT => "Portuguese",
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
