use crate::database;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Subset {
    pub from: String,
    pub to: String,
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
    Unknown,
}

impl From<database::relation::Relation> for Relation {
    fn from(value: database::relation::Relation) -> Self {
        match value.relation_type {
            database::relation::RelationType::Subset => Self::Subset(Subset {
                from: value.from,
                to: value.to,
            }),
            database::relation::RelationType::Unknown => Self::Unknown,
        }
    }
}
