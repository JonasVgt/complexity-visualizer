use crate::database::{complexity_class::ComplexityClass, data::Data, MyDatabase};

pub struct Model {
    db: Option<MyDatabase>,
    data: Data,
}

impl Model {
    pub fn new(db: MyDatabase) -> Self {
        return Model {
            data: Data::new(),
            db: Some(db),
        };
    }

    pub fn classes(&mut self) -> &Vec<ComplexityClass> {
        if let Some(mut db) = self.db.take() {
            if db.finish() {
                self.data = db.data;
            } else {
                self.db = Some(db);
            }
        }
        return &self.data.classes;
    }

    pub fn get_class(&self, id: u32) -> Option<&ComplexityClass> {
        self.data.classes.iter().find(|e| e.id == id)
    }
}
