use nlu_ontology::*;
use gazetteer_entity_parser::ParsedValue;


pub fn convert_to_builtin(
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
    let slot_value = match_entity_kind_to_slot_value!(MusicArtist);
    BuiltinEntity {
        value: value.raw_value,
        range: value.range,
        entity: slot_value,
        entity_kind: entity_kind.into(),
    }
}

