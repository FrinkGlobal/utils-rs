//! Fractal Global Relations module
//!
//! This module holds the Fractal Global Relationships, and there associated byte value.

/// Defined relationships
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Relationship {
    /// A stranger to the user
    Stranger,
    /// An Acquaintance to the uesr
    Acquaintance,
    /// A CoWorker to the user
    CoWorker,
    /// A friend to the uesr
    Friend,
    /// A Family member to the user
    Family,
}

/// Translates relationsip enum
pub fn get_relationship_id(rel: Relationship) -> u8 {
    match rel {
        Relationship::Stranger => 0,
        Relationship::Acquaintance => 1,
        Relationship::CoWorker => 2,
        Relationship::Friend => 3,
        Relationship::Family => 4,
    }
}

/// Grabs the relationship from the given id
pub fn get_relationship(id: u8) -> Relationship {
    match id {
        0 => Relationship::Stranger,
        1 => Relationship::Acquaintance,
        2 => Relationship::CoWorker,
        3 => Relationship::Friend,
        4 => Relationship::Family,
        _ => unreachable!(),
    }
}
