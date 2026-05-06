use crate::Tab;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Stats {
    active: bool,
}

impl Default for Stats {
    fn default() -> Self {
        Self { active: false }
    }
}

impl Tab for Stats {
    fn ui(&mut self, ui: &mut egui::Ui) {
        egui::Window::new(self.name())
            .open(self.get_active_mut_ref())
            .show(ui.ctx(), |ui| {
                ui.heading("Helllooo");
            });
    }

    fn name(&self) -> String {
        "Stats".to_owned()
    }

    fn is_active(&mut self) -> bool {
        self.active
    }

    fn get_active_mut_ref(&mut self) -> &mut bool {
        &mut self.active
    }
}
