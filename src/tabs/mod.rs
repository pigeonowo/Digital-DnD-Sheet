pub trait Tab {
    fn ui(&mut self, ui: &mut egui::Ui);
    fn name(&self) -> String;
    fn is_active(&mut self) -> bool;
    fn get_active_mut_ref(&mut self) -> &mut bool;
}

mod stats;
pub use stats::*;
mod character;
pub use character::*;
