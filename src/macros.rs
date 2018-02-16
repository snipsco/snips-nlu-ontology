#[macro_export]
macro_rules! enum_kind {
    ($kindname:ident, [$($varname:ident),*]) => {
        #[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Hash, Eq)]
        pub enum $kindname {
            $( $varname ),*
        }

        impl $kindname {
            pub fn all() -> &'static [$kindname] {
                static ALL: &'static [$kindname] = &[$( $kindname::$varname ),*];
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
