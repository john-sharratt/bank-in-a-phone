use egui::{Align2, RichText, TextEdit, Vec2, Widget};

use crate::state::local_app::LocalApp;

use super::Mode;

impl LocalApp {
    pub fn render_send_money(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        self.try_finish(|app| {
            app.mode = Mode::Summary;
        });

        let mut is_open = true;
        let mut should_transfer = false;
        egui::Window::new("Send Money")
            .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
            .default_size(Vec2::new(200.0, 200.0))
            .resizable(false)
            .collapsible(false)
            .open(&mut is_open)
            .show(ui.ctx(), |ui| {
                ui.horizontal(|ui| {
                    let max = self
                        .bank
                        .as_ref()
                        .and_then(|b| b.accounts.iter().find(|a| a.type_ == self.from_account))
                        .map(|a| a.balance_cents)
                        .unwrap_or_default() as f64;
                    let max = max / 100.0;
                    egui::Label::new(RichText::new("Amount: ").strong())
                        .selectable(false)
                        .ui(ui);

                    let mut transfer_amount = self.transfer_amount as f64 / 100.0;
                    ui.add(
                        egui::Slider::new(&mut transfer_amount, 0.0f64..=max)
                            .max_decimals(2)
                            .min_decimals(2),
                    );
                    self.transfer_amount = (transfer_amount * 100.0) as u64;
                });

                ui.horizontal(|ui| {
                    egui::Label::new(RichText::new("From: ").strong())
                        .selectable(false)
                        .ui(ui);
                    ui.label(format!("{}", self.from_account));
                });

                ui.horizontal(|ui| {
                    egui::Label::new(RichText::new("To: ").strong())
                        .selectable(false)
                        .ui(ui);
                    ui.label(format!("{}", self.to_account));
                });

                egui::TextEdit::singleline(&mut self.to_user).ui(ui);

                ui.add_space(5.0);

                ui.horizontal(|ui| {
                    ui.label("Description: ");
                    TextEdit::singleline(&mut self.description).ui(ui);
                });

                ui.add_space(10.0);

                if ui.button(RichText::new("Transfer").strong()).clicked() {
                    should_transfer = true;
                }
            });

        if should_transfer {
            if self.transfer_amount == 0 {
                self.show_dialog("Invalid Input", "You must actually transfer an amount");
            } else if self.description.is_empty() {
                self.show_dialog("Invalid Input", "You must enter a description");
            } else {
                self.transfer(frame);
                self.description.clear();
            }
        }

        if !is_open {
            self.mode = Mode::Summary;
        }
    }
}
