use nlu_ontology::*;
use nlu_utils::string::substring_with_char_range;
use gazetteer_entity_parser::ParsedValue;


pub fn convert_to_builtin(
    input: String,
    value: ParsedValue,
    entity_kind: GazetteerEntityKind,
) -> BuiltinEntity {
    macro_rules! match_entity_kind_to_slot_value {
        ($($varname:ident),*) => {
            match entity_kind {
                $(
                    GazetteerEntityKind::$varname => SlotValue::$varname(
                        StringValue {
                            value: value.resolved_value
                        }),
                )*
            }
        }
    };
    let slot_value = match_entity_kind_to_slot_value!(MusicArtist, MusicAlbum, MusicTrack);
    BuiltinEntity {
        value: substring_with_char_range(input, &value.range),
        range: value.range,
        entity: slot_value,
        entity_kind: entity_kind.into(),
    }
}

