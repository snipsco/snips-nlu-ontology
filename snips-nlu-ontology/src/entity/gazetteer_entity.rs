use entity::builtin_entity::{BuiltinEntityKind, IntoBuiltinEntityKind};
use errors::*;

sub_entity_kind!(
    GazetteerEntityKind,
    [
        MusicArtist,
        MusicAlbum,
        MusicTrack
    ]
);
