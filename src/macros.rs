#[macro_export]
macro_rules! language_enum {
    ([$($language:ident),*]) => {
        #[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Hash, Eq)]
        #[allow(non_camel_case_types)]
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

#[macro_export]
macro_rules! enum_kind {
    ($kindname:ident, [$($varname:ident),*]) => {
        #[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Hash, Eq)]
        pub enum $kindname {
            $( $varname ),*
        }

        impl $kindname {
            pub fn all() -> &'static [$kindname] {
                static ALL: &[$kindname] = &[$( $kindname::$varname ),*];
                ALL
            }
        }

        impl ::std::str::FromStr for $kindname {
            type Err=String;
            fn from_str(s: &str) -> ::std::result::Result<$kindname, Self::Err> {
                match s {
                    $(
                        stringify!($varname) => Ok($kindname::$varname),
                    )*
                    _ => Err(format!("{} is not a known {}", s, stringify!($kindname)))
                }
            }
        }

        impl ::std::string::ToString for $kindname {
            fn to_string(&self) -> String {
                match self {
                    $(
                        &$kindname::$varname => stringify!($varname).to_string(),
                    )*
                }
            }
        }

        impl IntoBuiltinEntityKind for $kindname {
            fn into_builtin_kind(self) -> BuiltinEntityKind {
                self
            }
        }
    }
}

#[macro_export]
macro_rules! sub_entity_kind {
    ($kindname:ident, [$($varname:ident),*]) => {
        #[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Hash, Eq)]
        pub enum $kindname {
            $( $varname ),*
        }

        impl $kindname {
            pub fn all() -> &'static [$kindname] {
                static ALL: &[$kindname] = &[$( $kindname::$varname ),*];
                ALL
            }
        }

        impl ::std::str::FromStr for $kindname {
            type Err=String;
            fn from_str(s: &str) -> ::std::result::Result<$kindname, Self::Err> {
                match s {
                    $(
                        stringify!($varname) => Ok($kindname::$varname),
                    )*
                    _ => Err(format!("{} is not a known {}", s, stringify!($kindname)))
                }
            }
        }

        impl ::std::string::ToString for $kindname {
            fn to_string(&self) -> String {
                match self {
                    $(
                        &$kindname::$varname => stringify!($varname).to_string(),
                    )*
                }
            }
        }

        impl IntoBuiltinEntityKind for $kindname {
            fn into_builtin_kind(self) -> BuiltinEntityKind {
                match self {
                    $(
                        $kindname::$varname => BuiltinEntityKind::$varname,
                    )*
                }
            }
        }

        impl $kindname {
            pub fn from_identifier(identifier: &str) -> Result<Self> {
                $kindname::all()
                    .iter()
                    .find(|kind| kind.identifier() == identifier)
                    .map(|k| k.clone())
                    .ok_or(format_err!("Unknown {} identifier: {}", stringify!($kindname), identifier))
            }
        }
    }
}
