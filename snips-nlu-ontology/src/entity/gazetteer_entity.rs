use entity::builtin_entity::{BuiltinEntityKind, IntoBuiltinEntityKind};
use errors::*;

sub_entity_kind!(
    BuiltinGazetteerEntityKind,
    [
        MusicArtist,
        MusicAlbum,
        MusicTrack
    ]
);

pub trait TryIntoBuiltinGazetteerEntityKind {
    fn try_into_gazetteer_kind(&self) -> Result<BuiltinGazetteerEntityKind>;
}

impl TryIntoBuiltinGazetteerEntityKind for BuiltinEntityKind {
    fn try_into_gazetteer_kind(&self) -> Result<BuiltinGazetteerEntityKind> {
        BuiltinGazetteerEntityKind::from_identifier(self.identifier())
    }
}
