use crate::database::{self, complexity_class::Tag};

use super::complexity_class::ComplexityClass;

pub struct Filter {
    has_changed: bool,
    selected_tags: Vec<bool>,
    pub show_complements: bool,
}

impl Filter {
    pub fn new() -> Self {
        Self {
            selected_tags: vec![
                true;
                database::complexity_class::Tag::tags()
                    .into_iter()
                    .map(|t| t as usize)
                    .max()
                    .unwrap()
                    + 1
            ],
            has_changed: false,
            show_complements: false,
        }
    }

    pub fn tag_get_mut(&mut self, tag: &Tag) -> &mut bool {
        &mut self.selected_tags[tag.clone() as usize]
    }

    pub fn redraw(&mut self) {
        self.has_changed = true;
    }

    pub fn redrawn(&mut self) {
        self.has_changed = false;
    }

    pub fn should_redraw(&self) -> bool {
        self.has_changed
    }

    pub fn apply_classes(&self, class: &ComplexityClass) -> bool {
        class
            .tags
            .iter()
            .filter(|a| !matches!(a, Tag::Complement))
            .any(|tag| self.selected_tags[tag.clone() as usize])
        && (!class.tags.contains(&Tag::Complement) || self.show_complements)
    }

    pub fn apply_relations(&self, from: &ComplexityClass, to: &ComplexityClass) -> bool {
        self.apply_classes(from) && self.apply_classes(to)
    }
}
