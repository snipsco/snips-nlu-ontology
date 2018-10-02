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

pub trait TryIntoGrammarEntityKind {
    fn try_into_grammar_kind(self) -> Result<GrammarEntityKind>;
}


impl TryIntoGrammarEntityKind for BuiltinEntityKind {
    fn try_into_grammar_kind(self) -> Result<GrammarEntityKind> {
        GrammarEntityKind::from_identifier(self.identifier())
    }
}
