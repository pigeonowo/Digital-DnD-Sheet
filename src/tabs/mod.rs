pub trait Tab {
    fn ui(&mut self, ui: &mut egui::Ui, active: &mut bool);
    fn name(&self) -> String;
}

mod stats;
pub use stats::*;
mod class;
pub use class::*;
mod race;
pub use race::*;
