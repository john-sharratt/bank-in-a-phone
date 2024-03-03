use egui::{Align2, Color32, Key, TextEdit, Vec2, Widget};

use crate::{state::local_app::FocusOn, LocalApp};

use super::{login::is_mobile, Mode};

fn is_ok(app: &LocalApp) -> bool {
    is_username_ok(app) && is_password_ok(app) && is_confirm_password_ok(app)
}

pub fn is_username_ok(app: &LocalApp) -> bool {
    app.username.len() >= 5
}

fn is_password_ok(app: &LocalApp) -> bool {
    app.password.len() >= 5
}

fn is_confirm_password_ok(app: &LocalApp) -> bool {
    app.confirm_password == app.password
}

impl LocalApp {
    fn create(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        if is_ok(self) {
            self.new_bank(ui, frame);
        } else {
            self.show_dialog(ui, "Invalid", "Inputs are invalid, check the fields");
        }
    }

    pub fn render_create_account(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        egui::Window::new("New Account")
            .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
            .default_size(Vec2::new(200.0, 200.0))
            .resizable(false)
            .collapsible(false)
            .show(ui.ctx(), |ui| {
                ui.add_enabled_ui(self.pending.is_none(), |ui| {
                    ui.add_space(5.0);

                    let mut enter_pressed =
                        ui.ctx().input_mut(|input| input.key_pressed(Key::Enter));

                    let mut focus_password = false;
                    let mut focus_confirm_password = false;

                    ui.horizontal(|ui| {
                        ui.label("Bank Name: ");

                        let is_ok = is_username_ok(self);
                        let res = TextEdit::singleline(&mut self.username)
                            .text_color(if is_ok {
                                if ui.style().visuals.dark_mode {
                                    Color32::GREEN
                                } else {
                                    Color32::DARK_GREEN
                                }
                            } else {
                                if ui.style().visuals.dark_mode {
                                    Color32::LIGHT_RED
                                } else {
                                    Color32::DARK_RED
                                }
                            })
                            .ui(ui);
                        self.username = self.username.to_lowercase();

                        if matches!(self.focus_on, Some(FocusOn::Username)) {
                            if !is_mobile(ui) {
                                res.request_focus();
                            }
                            self.focus_on.take();
                            self.save_state(frame);
                        }

                        if enter_pressed && res.lost_focus() {
                            enter_pressed = false;
                            focus_password = true;
                        }
                    });

                    ui.horizontal(|ui| {
                        ui.label("Password: ");

                        let is_ok = is_password_ok(self);
                        let res = TextEdit::singleline(&mut self.password)
                            .password(true)
                            .text_color(if is_ok {
                                if ui.style().visuals.dark_mode {
                                    Color32::GREEN
                                } else {
                                    Color32::DARK_GREEN
                                }
                            } else {
                                if ui.style().visuals.dark_mode {
                                    Color32::LIGHT_RED
                                } else {
                                    Color32::DARK_RED
                                }
                            })
                            .ui(ui);

                        if focus_password {
                            if !is_mobile(ui) {
                                res.request_focus();
                            }
                        }

                        if enter_pressed && res.lost_focus() {
                            enter_pressed = false;
                            focus_confirm_password = true;
                        }
                    });

                    ui.horizontal(|ui| {
                        ui.label("Confirm: ");

                        let is_ok = is_confirm_password_ok(self);
                        let res = TextEdit::singleline(&mut self.confirm_password)
                            .password(true)
                            .text_color(if is_ok {
                                if ui.style().visuals.dark_mode {
                                    Color32::GREEN
                                } else {
                                    Color32::DARK_GREEN
                                }
                            } else {
                                if ui.style().visuals.dark_mode {
                                    Color32::LIGHT_RED
                                } else {
                                    Color32::DARK_RED
                                }
                            })
                            .ui(ui);

                        if focus_confirm_password {
                            if !is_mobile(ui) {
                                res.request_focus();
                            }
                        }

                        if enter_pressed && res.lost_focus() {
                            enter_pressed = false;
                            self.create(ui, frame);
                        }
                    });

                    ui.add_space(5.0);

                    ui.horizontal(|ui| {
                        if ui
                            .add_sized(Vec2::new(100.0, 20.0), egui::Button::new("Create"))
                            .clicked()
                        {
                            self.create(ui, frame);
                        }

                        if ui
                            .add_sized(Vec2::new(100.0, 20.0), egui::Button::new("Back"))
                            .clicked()
                        {
                            self.username = Default::default();
                            self.password = Default::default();
                            self.confirm_password = Default::default();
                            self.mode = Mode::Login;
                            self.focus_on.replace(FocusOn::Username);
                        }
                    });
                });
            });
    }
}
