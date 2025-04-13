use crate::database;

use super::complexity_class::ComplexityClassId;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Relation {
    pub relation_type: RelationType,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum RelationType {
    Subset(Subset),
    Equal(Subset, Subset),
}

impl From<database::relation::Relation> for Relation {
    fn from(value: database::relation::Relation) -> Self {
        match value.relation_type {
            database::relation::RelationType::Subset => Relation {
                relation_type: RelationType::Subset(Subset {
                    from: value.from.into(),
                    to: value.to.into(),
                }),
            },
        }
    }
}

pub enum RelationComposition {
    Subset(Vec<Relation>),
    Equalily(Vec<Relation>),
}

impl RelationComposition {
    pub fn get_from(&self) -> ComplexityClassId {
        match self {
            RelationComposition::Subset(rs) => match rs.first().unwrap().relation_type {
                RelationType::Equal(s, _) => s.from,
                RelationType::Subset(s) => s.from,
            },
            RelationComposition::Equalily(rs) => match rs.first().unwrap().relation_type {
                RelationType::Equal(s, _) => s.from,
                RelationType::Subset(s) => s.from,
            },
        }
    }

    pub fn get_to(&self) -> ComplexityClassId {
        match self {
            RelationComposition::Subset(rs) => match rs.last().unwrap().relation_type {
                RelationType::Equal(s, _) => s.to,
                RelationType::Subset(s) => s.to,
            },
            RelationComposition::Equalily(rs) => match rs.last().unwrap().relation_type {
                RelationType::Equal(s, _) => s.to,
                RelationType::Subset(s) => s.to,
            },
        }
    }
}
