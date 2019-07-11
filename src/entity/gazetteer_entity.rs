use crate::entity::builtin_entity::{BuiltinEntityKind, IntoBuiltinEntityKind};
use crate::errors::*;
use crate::sub_entity_kind;
use failure::format_err;

sub_entity_kind!(
    BuiltinGazetteerEntityKind,
    [City, Country, MusicAlbum, MusicArtist, MusicTrack, Region]
);

pub trait TryIntoBuiltinGazetteerEntityKind {
    fn try_into_gazetteer_kind(&self) -> Result<BuiltinGazetteerEntityKind>;
}

impl TryIntoBuiltinGazetteerEntityKind for BuiltinEntityKind {
    fn try_into_gazetteer_kind(&self) -> Result<BuiltinGazetteerEntityKind> {
        BuiltinGazetteerEntityKind::from_identifier(self.identifier())
    }
}
