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
            fn into_bek(self) -> BuiltinEntityKind {
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
