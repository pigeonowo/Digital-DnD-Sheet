use crate::Character;
use crate::Sheet;
use crate::Stats;
use crate::Tab;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct DndApp {
    sheet: Sheet,
    #[serde(skip)]
    tabs: Vec<Box<dyn Tab>>,
}

impl Default for DndApp {
    fn default() -> Self {
        Self {
            sheet: Sheet::default(),
            tabs: vec![Box::<Stats>::default(), Box::<Character>::default()],
        }
    }
}

impl DndApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }
}

impl eframe::App for DndApp {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.tabs_right_panel(ui);
        self.top_panel(ui);
    }
}

impl DndApp {
    fn top_panel(&mut self, ui: &mut egui::Ui) {
        egui::Panel::top("top_panel")
            .resizable(true)
            .show_inside(ui, |ui| {
                ui.horizontal_centered(|ui| {
                    egui::widgets::global_theme_preference_switch(ui);
                    ui.text_edit_singleline(&mut self.sheet.name);
                });
            });
    }

    fn tabs_right_panel(&mut self, ui: &mut egui::Ui) {
        egui::Panel::right("tabs").show_inside(ui, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.heading("Tabs");
                ui.separator();
                for tab in &mut self.tabs {
                    let name = tab.name();
                    ui.toggle_value(tab.get_active_mut_ref(), name);
                    if tab.is_active() {
                        tab.ui(ui);
                    }
                }
            });
        });
    }
}
