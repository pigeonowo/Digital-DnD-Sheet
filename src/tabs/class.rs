use crate::Tab;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct ClassTab {
    pub class: Class,
    pub subclass: SubClass,
    pub classinfo: String,
}

impl Default for ClassTab {
    fn default() -> Self {
        Self {
            class: Class::Druid,
            subclass: SubClass::Druid(DruidSubClass::Dreams),
            classinfo: String::from(""),
        }
    }
}

impl Tab for ClassTab {
    fn ui(&mut self, ui: &mut egui::Ui, active: &mut bool) {
        let name = self.name();
        egui::Window::new(name)
            .vscroll(true)
            .hscroll(true)
            .collapsible(true)
            .open(active)
            .show(ui.ctx(), |ui| {
                ui.horizontal(|ui| {
                    ui.label("Class:");
                    egui::ComboBox::from_id_salt("Class")
                        .selected_text(format!("{:?}", self.class))
                        .show_ui(ui, |ui| {
                            self.ui_classes(ui);
                        });
                });
                ui.horizontal(|ui| {
                    ui.label("SubClass:");
                    egui::ComboBox::from_id_salt("SubClass")
                        .selected_text(format!("{:?}", self.subclass))
                        .show_ui(ui, |ui| {
                            self.ui_subclasses(ui);
                        });
                });
                ui.separator();
                ui.text_edit_multiline(&mut self.classinfo);
            });
    }

    fn name(&self) -> String {
        "Class".to_owned()
    }
}

impl ClassTab {
    fn ui_subclasses(&mut self, ui: &mut egui::Ui) {
        ui.selectable_value(
            &mut self.subclass,
            SubClass::Druid(DruidSubClass::Dreams),
            "Dreams",
        );
    }

    fn ui_classes(&mut self, ui: &mut egui::Ui) {
        ui.selectable_value(&mut self.class, Class::Artificer, "Artificer");
        ui.selectable_value(&mut self.class, Class::Barbarian, "Barbarian");
        ui.selectable_value(&mut self.class, Class::Bard, "Bard");
        ui.selectable_value(&mut self.class, Class::Cleric, "Cleric");
        ui.selectable_value(&mut self.class, Class::Druid, "Druid");
        ui.selectable_value(&mut self.class, Class::Figter, "Figter");
        ui.selectable_value(&mut self.class, Class::Monk, "Monk");
        ui.selectable_value(&mut self.class, Class::Paladin, "Paladin");
        ui.selectable_value(&mut self.class, Class::Ranger, "Ranger");
        ui.selectable_value(&mut self.class, Class::Rogue, "Rogue");
        ui.selectable_value(&mut self.class, Class::Sorcerer, "Sorcerer");
        ui.selectable_value(&mut self.class, Class::Warlock, "Warlock");
        ui.selectable_value(&mut self.class, Class::Wizard, "Wizard");
    }
}

#[derive(Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum Class {
    Artificer,
    Barbarian,
    Bard,
    Cleric,
    Druid,
    Figter,
    Monk,
    Paladin,
    Ranger,
    Rogue,
    Sorcerer,
    Warlock,
    Wizard,
}

#[derive(Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum SubClass {
    Artificer(ArtificerSubclass),
    Barbarian(BarbarianSubclass),
    Bard(BardSubclass),
    Cleric,
    Druid(DruidSubClass),
    Figter,
    Monk,
    Paladin,
    Ranger,
    Rogue,
    Sorcerer,
    Warlock,
    Wizard,
}

#[derive(Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum ArtificerSubclass {
    Alchemist,
    Aromorer,
    Artillerist,
    BattleSmith,
}

#[derive(Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum BarbarianSubclass {
    AncestralGuardian,
    Battlerager,
    Beast,
    Berserker,
    Giant,
    StormHerald,
    TotemWarrior,
    WildMagic,
    Zealot,
}

#[derive(Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum BardSubclass {
    Creation,
    Eloquence,
    Glamour,
    Lore,
    Spirits,
    Swords,
    Valor,
    Tragedy,
    Whispers,
}

#[derive(Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum DruidSubClass {
    Blighted,
    Dreams,
    Land,
    Moon,
    Sea,
    Shepherd,
    Spores,
    Stars,
    Wildfire,
}
