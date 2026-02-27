use crate::database;

use super::complexity_class::ComplexityClassId;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Subset {
    pub from: ComplexityClassId,
    pub to: ComplexityClassId,
}

pub type RelationId = u32;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Relation {
    id: RelationId,
    pub relation_type: RelationType,
}

impl Relation {
    pub fn from_db(id: RelationId, value: database::relation::Relation) -> Self {
        match value.relation_type {
            database::relation::RelationType::Subset => Relation {
                relation_type: RelationType::Subset(Subset {
                    from: value.from.into(),
                    to: value.to.into(),
                }),
                id,
            },
        }
    }

    pub fn id(&self) -> RelationId {
        self.id
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[allow(dead_code)]
pub enum RelationType {
    Subset(Subset),
    Equal(Subset, Subset),
}

pub type RelationCompositionId = Vec<RelationId>;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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

    pub fn id(&self) -> RelationCompositionId {
        match self {
            RelationComposition::Subset(r) => r,
            RelationComposition::Equalily(r) => r,
        }
        .iter()
        .map(|r| r.id())
        .collect()
    }
}
