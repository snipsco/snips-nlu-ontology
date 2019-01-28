use crate::entity::builtin_entity::{BuiltinEntityKind, IntoBuiltinEntityKind};
use crate::errors::*;
use crate::sub_entity_kind;
use failure::format_err;

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
