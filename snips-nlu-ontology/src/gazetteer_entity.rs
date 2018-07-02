use std::convert::From;

use builtin_entity::BuiltinEntityKind;
use errors::*;

enum_kind!(
    GazetteerEntityKind,
    [
        MusicArtist,
        MusicAlbum,
        MusicTrack
    ]
);

impl GazetteerEntityKind {
    pub fn identifier(&self) -> &'static str {
        BuiltinEntityKind::from(*self).identifier()
    }

    pub fn from_identifier(identifier: &str) -> Result<Self> {
        GazetteerEntityKind::all()
            .iter()
            .find(|kind| kind.identifier() == identifier)
            .map(|k| k.clone())
            .ok_or(format_err!("Unknown GazetteerEntityKind identifier: {}", identifier))
    }
}

impl From<GazetteerEntityKind> for BuiltinEntityKind {
    fn from(kind: GazetteerEntityKind) -> Self {
        macro_rules! convert_gazetteer_entity {
            ($($varname:ident),*) => {
                match kind {
                    $(
                        GazetteerEntityKind::$varname => BuiltinEntityKind::$varname,
                    )*
                }
            }
        };
        convert_gazetteer_entity!(MusicArtist, MusicAlbum, MusicTrack)
    }
}
