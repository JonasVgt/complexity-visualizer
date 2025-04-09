use crate::database;

use super::complexity_class::ComplexityClassId;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Subset {
    pub from: ComplexityClassId,
    pub to: ComplexityClassId,
}

impl Subset {
    pub fn inversed(self) -> Self {
        Subset {
            from: self.to,
            to: self.from,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Relation {
    Subset(Subset),
    Equal(Subset, Subset),
}

impl From<database::relation::Relation> for Relation {
    fn from(value: database::relation::Relation) -> Self {
        match value.relation_type {
            database::relation::RelationType::Subset => Self::Subset(Subset {
                from: value.from.into(),
                to: value.to.into(),
            }),
        }
    }
}
