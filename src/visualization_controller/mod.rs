use std::collections::HashMap;

use egui::Pos2;

use crate::database::data::Data;

pub struct VisualizationController<'a> {
    data: &'a Data,
}

impl<'a> VisualizationController<'a> {
    pub fn new(data: &'a Data) -> Self {
        Self { data }
    }
    pub fn arrange(&self) -> HashMap<u32, Pos2> {
        let pos = |id| egui::pos2((200 * id) as f32, (100 * id) as f32);
        self.data.classes.iter().map(|class| (class.id, pos(class.id))).collect()
    }
}
