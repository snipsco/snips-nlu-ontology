use entity::builtin_entity::{BuiltinEntityKind, IntoBuiltinEntityKind};
use errors::*;

sub_entity_kind!(
    GrammarEntityKind,
    [
        AmountOfMoney,
        Duration,
        Number,
        Ordinal,
        Temperature,
        Time,
        Percentage
    ]
);
