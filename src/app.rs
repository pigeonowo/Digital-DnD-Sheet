use egui::Align2;
use egui::Color32;
use egui::Frame;
use egui::Pos2;
use egui::Rect;
use egui::Scene;
use egui::Sense;
use egui::Theme;
use egui::Vec2;
use egui::epaint::PathShape;
use egui::epaint::PathStroke;

use crate::Sheet;
use crate::Tab;

pub type Active = bool;

const AMOUNT_TABS: usize = 3;
const MAP_TILE_SIZE: f32 = 50.;
const MAP_TILE_OFFSET: f32 = 50.;
const MAP_UNEVEN_Y_OFFSET: f32 = 50.;
const MAP_WIDTH: usize = 52;
const MAP_HEIGHT: usize = 34;

#[derive(
    serde::Deserialize, serde::Serialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord,
)]
pub enum Tile {
    Nothing,
    Street,
    Hills,
    Forest,
    Mountains,
    Ocean,
    Temple,
    City,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct DndApp {
    sheet: Sheet,
    active_tabs: Vec<Active>,
    map_rect: Rect,
    map_tiles: Vec<Tile>,
    map_open_select_type: Option<(String, usize)>,
    map_open_select_type_open: bool,
}

impl Default for DndApp {
    fn default() -> Self {
        Self {
            sheet: Sheet::default(),
            active_tabs: vec![false; AMOUNT_TABS],
            map_rect: Rect::ZERO,
            map_tiles: vec![Tile::Nothing; MAP_WIDTH * MAP_HEIGHT],
            map_open_select_type: None,
            map_open_select_type_open: false,
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
        let mut app: DndApp = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        };
        if app.active_tabs.len() != AMOUNT_TABS {
            app.active_tabs = vec![false; AMOUNT_TABS];
        }
        app
    }
}

impl eframe::App for DndApp {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        // if let Ok(json) = serde_json::to_string_pretty(&self) {
        //     println!("Saving State: {}", json); // Check if 'Character' data is here!
        // }
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.tabs_right_panel(ui);
        self.top_panel(ui);
        let central_panel = egui::CentralPanel::default_margins();
        central_panel.show_inside(ui, |ui| {
            self.map(ui);
        });
    }
}

impl DndApp {
    fn reset_map(&mut self) {
        self.map_tiles = vec![Tile::Nothing; MAP_HEIGHT * MAP_WIDTH];
    }

    fn top_panel(&mut self, ui: &mut egui::Ui) {
        egui::Panel::top("top_panel")
            .resizable(true)
            .show_inside(ui, |ui| {
                ui.horizontal_centered(|ui| {
                    let is_web = cfg!(target_arch = "wasm32");
                    if !is_web {
                        ui.menu_button("File", |ui| {
                            if ui.button("Quit").clicked() {
                                ui.send_viewport_cmd(egui::ViewportCommand::Close);
                            }
                            if ui.button("Save").clicked() {
                                // self.save();
                            }
                            if ui.button("Reset Map").clicked() {
                                self.reset_map();
                            }
                        });
                    }
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
                // --
                let stats_active_ref = self.active_tabs.get_mut(0).unwrap();
                ui.toggle_value(stats_active_ref, self.sheet.stats.name());
                if *stats_active_ref {
                    self.sheet.stats.ui(ui, stats_active_ref);
                }
                // --
                let character_active_ref = self.active_tabs.get_mut(1).unwrap();
                ui.toggle_value(character_active_ref, self.sheet.character.name());
                if *character_active_ref {
                    self.sheet.character.ui(ui, character_active_ref);
                }
                // --
                let race_active_ref = self.active_tabs.get_mut(2).unwrap();
                ui.toggle_value(race_active_ref, self.sheet.race.name());
                if *race_active_ref {
                    self.sheet.race.ui(ui, race_active_ref);
                }
            });
        });
    }

    fn map(&mut self, ui: &mut egui::Ui) {
        let scene = Scene::new().zoom_range(0.2..=3.0);
        let canvas = Frame::canvas(ui.style()).outer_margin(-10);
        let theme = ui.ctx().theme();
        canvas.show(ui, |ui| {
            let response = scene.show(ui, &mut self.map_rect, |ui| {
                ui.request_repaint();

                let mut shapes = vec![];
                let mut labels: Vec<(Rect, String)> = vec![];
                let mut context_rects = vec![];
                let mut map_index = 0;

                for x in 0..MAP_WIDTH {
                    for y in 0..MAP_HEIGHT {
                        // shapes.push(epaint::Shape::rect_filled(
                        //     rect,
                        //     3,
                        //     Color32::from_rgb(0, 255, 0),
                        // ));
                        let y_offset = if x % 2 != 0 {
                            MAP_UNEVEN_Y_OFFSET
                        } else {
                            0f32
                        };
                        let pos = Pos2::new(
                            (x as f32 * MAP_TILE_SIZE) + x as f32 * MAP_TILE_OFFSET
                                - (15.0 * x as f32),
                            (y as f32 * MAP_TILE_SIZE) + y as f32 * MAP_TILE_OFFSET + y_offset,
                        );
                        let rect = Rect {
                            min: Pos2::new(pos.x - 25.0, pos.y - 45.0),
                            max: Pos2::new(pos.x + 25.0, pos.y - 20.0),
                        };
                        let ctx_rect = Rect {
                            min: Pos2::new(pos.x - 50.0, pos.y - 50.0),
                            max: Pos2::new(pos.x + 50.0, pos.y + 50.0),
                        };
                        shapes.push(create_hexagon(pos, theme, self.map_tiles[map_index]));
                        labels.push((rect, format!("{x:02}{y:02}")));
                        context_rects.push(ctx_rect);

                        map_index += 1;
                    }
                }

                ui.painter().extend(shapes);
                for (i, ((label_rect, label), ctx_rect)) in
                    (labels.iter()).zip(context_rects).enumerate()
                {
                    // label
                    ui.put(
                        label_rect.clone(),
                        egui::Label::new(
                            egui::RichText::new(label)
                                .color(if ui.theme() == Theme::Light {
                                    Color32::from_rgb(0, 0, 0)
                                } else {
                                    Color32::from_rgb(255, 255, 255)
                                })
                                .background_color(if ui.theme() == Theme::Light {
                                    Color32::from_rgb(255, 255, 255)
                                } else {
                                    Color32::from_rgb(0, 0, 0)
                                })
                                .strong(),
                        ),
                    );

                    // context
                    let tile_type = self.map_tiles.get_mut(i).unwrap();
                    let res = ui.interact(ctx_rect, label.clone().into(), Sense::click());
                    if res.clicked_by(egui::PointerButton::Secondary) {
                        println!("Tile {label} got clicked (secondary) of type {tile_type:?}");
                        self.map_open_select_type = Some((label.clone(), i));
                        self.map_open_select_type_open = true;
                    }
                }
                if let Some((st_label, tile_index)) = &self.map_open_select_type {
                    let tile_ref = self.map_tiles.get_mut(*tile_index).unwrap();
                    egui::Window::new(format!("Set Tiletype {st_label}"))
                        .open(&mut self.map_open_select_type_open)
                        .collapsible(false)
                        .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
                        .show(ui.ctx(), |ui| {
                            ui.radio_value(tile_ref, Tile::Nothing, "Nothing");
                            ui.radio_value(tile_ref, Tile::City, "City");
                            ui.radio_value(tile_ref, Tile::Forest, "Forest");
                            ui.radio_value(tile_ref, Tile::Hills, "Hills");
                            ui.radio_value(tile_ref, Tile::Mountains, "Mountains");
                            ui.radio_value(tile_ref, Tile::Ocean, "Ocean");
                            ui.radio_value(tile_ref, Tile::Street, "Street");
                            ui.radio_value(tile_ref, Tile::Temple, "Temple");
                        });
                }
            });
            if response.response.double_clicked() {
                self.map_rect = Rect::ZERO;
            }
        });
    }
}

fn create_hexagon(pos: Pos2, theme: Theme, tile: Tile) -> egui::Shape {
    let center = pos;
    let radius = MAP_TILE_SIZE;
    let num_sides = 6;
    let mut points = Vec::with_capacity(num_sides);

    // Calculate vertices
    for i in 0..num_sides {
        let angle = (i as f32) * std::f32::consts::TAU / (num_sides as f32);
        let point = center + radius * Vec2::new(angle.cos(), angle.sin());
        points.push(point);
    }

    // Add closed loop for the hexagon
    points.push(points[0]);

    // fill color
    let fill_color = if theme == Theme::Light {
        match tile {
            Tile::Nothing => Color32::from_rgba_unmultiplied(255, 255, 255, 0),
            Tile::Street => Color32::from_rgb(30, 30, 30),
            Tile::Hills => Color32::from_rgb(200, 255, 200),
            Tile::Forest => Color32::from_rgb(20, 255, 20),
            Tile::Mountains => Color32::from_rgb(180, 180, 180),
            Tile::Ocean => Color32::from_rgb(50, 50, 255),
            Tile::Temple => Color32::from_rgb(200, 200, 100),
            Tile::City => Color32::from_rgb(255, 100, 100),
        }
    } else {
        match tile {
            Tile::Nothing => Color32::from_rgba_unmultiplied(255, 255, 255, 0),
            Tile::Street => Color32::from_rgb(80, 80, 80),
            Tile::Hills => Color32::from_rgb(200, 255, 200),
            Tile::Forest => Color32::from_rgb(100, 255, 100),
            Tile::Mountains => Color32::from_rgb(230, 230, 230),
            Tile::Ocean => Color32::from_rgb(150, 150, 255),
            Tile::Temple => Color32::from_rgb(255, 255, 100),
            Tile::City => Color32::from_rgb(255, 200, 200),
        }
    };

    egui::Shape::Path(PathShape {
        points,
        fill: fill_color,
        stroke: PathStroke::new(
            2.0,
            if theme == Theme::Light {
                Color32::BLACK
            } else {
                Color32::WHITE
            },
        ),
        closed: true,
    })
}
