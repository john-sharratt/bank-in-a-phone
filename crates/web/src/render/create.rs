use egui::{Align2, Color32, Key, Modifiers, TextEdit, Vec2, Widget};

use crate::{state::local_app::FocusOn, LocalApp};

use super::Mode;

impl LocalApp {
    fn is_ok(&self) -> bool {
        self.username.len() >= 5
    }

    fn create(&mut self, _ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        if self.is_ok() {
            self.new_bank(frame);
        }
    }

    pub fn render_create_account(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        self.try_finish(|app| {
            app.mode = Mode::Summary;
        });

        egui::Window::new("New Account")
            .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
            .default_size(Vec2::new(200.0, 200.0))
            .resizable(false)
            .collapsible(false)
            .show(ui.ctx(), |ui| {
                ui.add_space(5.0);
                ui.horizontal(|ui| {
                    ui.label("Name: ");

                    if ui
                        .ctx()
                        .input_mut(|input| input.consume_key(Modifiers::NONE, Key::Enter))
                    {
                        self.create(ui, frame);
                    }

                    let is_ok = self.is_ok();
                    let res = TextEdit::singleline(&mut self.username)
                        .text_color(if is_ok {
                            Color32::GREEN
                        } else {
                            Color32::LIGHT_RED
                        })
                        .ui(ui);
                    if matches!(self.focus_on, Some(FocusOn::Username)) {
                        res.request_focus();
                        self.focus_on.take();
                        self.save_state(frame);
                    }
                });

                ui.add_space(5.0);

                if ui
                    .add_sized(Vec2::new(100.0, 20.0), egui::Button::new("Create"))
                    .clicked()
                {
                    self.create(ui, frame);
                }
            });
    }
}
