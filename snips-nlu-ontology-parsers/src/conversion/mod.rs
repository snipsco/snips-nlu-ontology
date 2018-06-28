pub mod rustling;
pub mod gazetteer_entities;

use errors::*;

pub trait OntologyFrom<T> {
    fn ontology_from(_: T) -> Self;
}

pub trait OntologyInto<T> {
    fn ontology_into(self) -> T;
}

pub trait TryOntologyInto<T>: Sized {
    /// Performs the conversion.
    fn try_ontology_into(self) -> Result<T>;
}

/// Attempt to construct `Self` via a conversion.
pub trait TryOntologyFrom<T>: Sized {
    /// Performs the conversion.
    fn try_ontology_from(value: T) -> Result<Self>;
}

impl<T, U> OntologyInto<U> for T where U: OntologyFrom<T> {
    fn ontology_into(self) -> U {
        U::ontology_from(self)
    }
}

// From (and thus Into) is reflexive
impl<T> OntologyFrom<T> for T {
    fn ontology_from(t: T) -> T {
        t
    }
}

// TryFrom implies TryInto
impl<T, U> TryOntologyInto<U> for T where U: TryOntologyFrom<T> {
    fn try_ontology_into(self) -> Result<U> {
        U::try_ontology_from(self)
    }
}
