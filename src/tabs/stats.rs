use crate::Tab;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Stats {
    pub health: i32,
    pub max_health: i32,
    pub temp_health: i32,
    pub armor_class: i32,
    pub initiative: i32,
    pub inspiration: i32,
    pub movementrate: i32, // foot
    pub practice_bonus: i32,
    pub successful_saving_throws: i32,
    pub failed_saving_throws: i32,
    pub hitdice: String,
    pub skills: Vec<Skill>,
    pub stats: Vec<Stat>,
    pub saving_throws: Vec<SavingThrow>,
}
impl Default for Stats {
    fn default() -> Self {
        let default_stats = vec![
            Stat::new("Strength".to_string()),
            Stat::new("Agility".to_string()),
            Stat::new("Constitution".to_string()),
            Stat::new("Intelligence".to_string()),
            Stat::new("Wise".to_string()),
            Stat::new("Charisma".to_string()),
        ];
        let default_saving_throws = vec![
            SavingThrow::new("Strength".to_string()),
            SavingThrow::new("Agility".to_string()),
            SavingThrow::new("Constitution".to_string()),
            SavingThrow::new("Intelligence".to_string()),
            SavingThrow::new("Wise".to_string()),
            SavingThrow::new("Charisma".to_string()),
        ];
        let default_skills = vec![
            Skill::new("Acrobatics".to_string()),
            Skill::new("Arcane_art".to_string()),
            Skill::new("Athletics".to_string()),
            Skill::new("Appearance".to_string()),
            Skill::new("Intimidate".to_string()),
            Skill::new("Dexterity".to_string()),
            Skill::new("History".to_string()),
            Skill::new("Medicine".to_string()),
            Skill::new("Stealth".to_string()),
            Skill::new("Animals".to_string()),
            Skill::new("Detect Motives".to_string()),
            Skill::new("Investigation".to_string()),
            Skill::new("Natural History".to_string()),
            Skill::new("Religion".to_string()),
            Skill::new("Fake".to_string()),
            Skill::new("Deceive".to_string()),
            Skill::new("Survival".to_string()),
            Skill::new("Convince".to_string()),
            Skill::new("Perception".to_string()),
        ];
        Self {
            health: 10,
            max_health: 10,
            temp_health: 0,
            armor_class: 10,
            initiative: 1,
            inspiration: 0,
            movementrate: 30,
            hitdice: "1D8".to_string(),
            practice_bonus: 2,
            stats: default_stats,
            skills: default_skills,
            saving_throws: default_saving_throws,
            successful_saving_throws: 0,
            failed_saving_throws: 0,
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Skill {
    name: String,
    value: i32,
    selected: bool,
}
impl Default for Skill {
    fn default() -> Self {
        Self {
            name: "Some Skill".to_string(),
            value: 0,
            selected: false,
        }
    }
}
impl Skill {
    fn new(name: String) -> Self {
        Self {
            name: name,
            ..Default::default()
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
/// idk how to really call it
pub struct Stat {
    name: String,
    value: i32,
}
impl Default for Stat {
    fn default() -> Self {
        Self {
            name: "Some Stat".to_string(),
            value: 0,
        }
    }
}
impl Stat {
    fn new(name: String) -> Self {
        Self {
            name: name,
            ..Default::default()
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct SavingThrow {
    name: String,
    value: i32,
    selected: bool,
}
impl Default for SavingThrow {
    fn default() -> Self {
        Self {
            name: "Some SavingThrow".to_string(),
            value: 0,
            selected: false,
        }
    }
}
impl SavingThrow {
    fn new(name: String) -> Self {
        Self {
            name: name,
            ..Default::default()
        }
    }
}

impl Tab for Stats {
    fn ui(&mut self, ui: &mut egui::Ui, active: &mut bool) {
        let name = self.name();
        egui::Window::new(name)
            .resizable(true)
            .open(active)
            .vscroll(true)
            .hscroll(true)
            .show(ui.ctx(), |ui| {
                ui.vertical_centered_justified(|ui| {
                    ui.heading("General Stats");
                });
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.label("Health:");
                            ui.add(egui::DragValue::new(&mut self.health));
                        });
                        ui.horizontal(|ui| {
                            ui.label("Max Health:");
                            ui.add(egui::DragValue::new(&mut self.max_health));
                        });
                        ui.horizontal(|ui| {
                            ui.label("Temporary Health:");
                            ui.add(egui::DragValue::new(&mut self.temp_health));
                        });
                        ui.horizontal(|ui| {
                            ui.label("Armor Class:");
                            ui.add(egui::DragValue::new(&mut self.armor_class));
                        });
                    });
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.label("Initiative:");
                            ui.add(egui::DragValue::new(&mut self.initiative));
                        });
                        ui.horizontal(|ui| {
                            ui.label("Inspiration:");
                            ui.add(egui::DragValue::new(&mut self.inspiration));
                        });
                        ui.horizontal(|ui| {
                            ui.label("Movementrate:");
                            ui.add(egui::DragValue::new(&mut self.movementrate));
                        });
                        ui.horizontal(|ui| {
                            ui.label("Practice Bonus:");
                            ui.add(egui::DragValue::new(&mut self.practice_bonus));
                        });
                    });
                });
                ui.horizontal(|ui| {
                    ui.label("Hitdice:");
                    ui.text_edit_singleline(&mut self.hitdice);
                });
                ui.separator();
                ui.horizontal(|ui| {
                    // stats + saving throws
                    ui.vertical(|ui| {
                        ui.heading("Stats");
                        for stat in &mut self.stats {
                            ui.horizontal(|ui| {
                                ui.label(stat.name.as_str());
                                ui.add(egui::DragValue::new(&mut stat.value))
                            });
                        }
                        // ui.separator();
                        ui.add_space(32.0);
                        ui.heading("Saving Throws");
                        for saving_throw in &mut self.saving_throws {
                            ui.horizontal(|ui| {
                                ui.checkbox(&mut saving_throw.selected, "");
                                ui.label(saving_throw.name.as_str());
                                ui.add(egui::DragValue::new(&mut saving_throw.value))
                            });
                        }
                        ui.add_space(48.0);
                        ui.horizontal(|ui| {
                            ui.label("+ Saving Throws:");
                            ui.add(egui::DragValue::new(&mut self.successful_saving_throws));
                        });
                        ui.horizontal(|ui| {
                            ui.label("- Saving Throws:");
                            ui.add(egui::DragValue::new(&mut self.failed_saving_throws));
                        });
                    });
                    // skills
                    ui.vertical(|ui| {
                        ui.heading("Skills");
                        for skill in &mut self.skills {
                            ui.horizontal(|ui| {
                                ui.checkbox(&mut skill.selected, "");
                                ui.label(skill.name.as_str());
                                ui.add(egui::DragValue::new(&mut skill.value))
                            });
                        }
                    });
                });
            });
    }

    fn name(&self) -> String {
        "Stats".to_owned()
    }
}
