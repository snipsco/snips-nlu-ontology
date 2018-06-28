use std::convert::From;

use builtin_entity::BuiltinEntityKind;
use errors::*;

enum_kind!(
    GazetteerEntityKind,
    [
        MusicArtist
    ]
);

impl GazetteerEntityKind {
    pub fn identifier(&self) -> &str {
        match *self {
            GazetteerEntityKind::MusicArtist => "snips/musicArtist",
        }
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
        match kind {
            GazetteerEntityKind::MusicArtist => BuiltinEntityKind::MusicArtist
        }
    }
}
