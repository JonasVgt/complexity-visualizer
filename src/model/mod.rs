use crate::database::{complexity_class::ComplexityClass, MyDatabase};

pub struct Model {
    db: Option<MyDatabase>,
    classes: Vec<ComplexityClass>,
}

impl Model {
    pub fn new(db: MyDatabase) -> Self {
        return Model {
            classes: vec![],
            db: Some(db),
        };
    }

    pub fn classes(&mut self) -> &Vec<ComplexityClass> {
        if let Some(mut db) = self.db.take() {
            if db.finish() {
                self.classes = db.classes;
            } else {
                self.db = Some(db);
            }
        }
        return &self.classes;
    }

    pub fn get_class(&self, id: u32) -> Option<&ComplexityClass> {
        self.classes.iter().find(|e| e.id == id)
    }
}
