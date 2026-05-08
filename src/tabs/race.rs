use crate::Tab;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct RaceTab {
    pub race: String,
    pub raceinfo: String,
}

impl Default for RaceTab {
    fn default() -> Self {
        Self {
            race: String::from(""),
            raceinfo: String::from(""),
        }
    }
}

impl Tab for RaceTab {
    fn ui(&mut self, ui: &mut egui::Ui, active: &mut bool) {
        let name = self.name();
        egui::Window::new(name)
            .vscroll(true)
            .hscroll(true)
            .collapsible(true)
            .open(active)
            .show(ui.ctx(), |ui| {
                ui.horizontal(|ui| {
                    ui.label("Race:");
                    ui.text_edit_singleline(&mut self.race);
                });
                ui.label("Info:");
                ui.separator();
                ui.text_edit_multiline(&mut self.raceinfo);
            });
    }

    fn name(&self) -> String {
        "Race".to_owned()
    }
}
