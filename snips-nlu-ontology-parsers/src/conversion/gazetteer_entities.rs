use nlu_ontology::{BuiltinGazetteerEntityKind, SlotValue, StringValue};

pub fn convert_to_slot_value(
    resolved_value: String,
    entity_kind: BuiltinGazetteerEntityKind,
) -> SlotValue {
    macro_rules! match_entity_kind_to_slot_value {
        ($($varname:ident),*) => {
            match entity_kind {
                $(
                    BuiltinGazetteerEntityKind::$varname => SlotValue::$varname(
                        StringValue {value: resolved_value}),
                )*
            }
        }
    };
    return match_entity_kind_to_slot_value!(MusicAlbum, MusicArtist, MusicTrack);
}
