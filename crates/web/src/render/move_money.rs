use egui::{Align2, Key, RichText, Vec2, Widget};
use immutable_bank_model::account::AccountType;

use crate::state::local_app::{FocusOn, LocalApp};

use super::Mode;

impl LocalApp {
    pub fn render_move_money(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        let mut is_open = true;
        let mut should_transfer = false;
        egui::Window::new("Move Money")
            .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
            .default_size(Vec2::new(200.0, 200.0))
            .resizable(false)
            .collapsible(false)
            .open(&mut is_open)
            .show(ui.ctx(), |ui| {
                ui.add_enabled_ui(self.pending.is_none(), |ui| {
                    let mut enter_pressed =
                        ui.ctx().input_mut(|input| input.key_pressed(Key::Enter));

                    let mut focus_to = false;
                    let mut focus_transfer = false;

                    ui.horizontal(|ui| {
                        let max = self
                            .bank()
                            .and_then(|b| b.accounts.iter().find(|a| a.type_ == self.from_account))
                            .map(|a| a.balance_cents)
                            .unwrap_or_default() as f64;
                        let max = max / 100.0;
                        egui::Label::new(RichText::new("Amount: ").strong())
                            .selectable(false)
                            .ui(ui);

                        let mut transfer_amount = self.transfer_amount as f64 / 100.0;

                        let res = ui.add(
                            egui::Slider::new(&mut transfer_amount, 0.0f64..=max)
                                .max_decimals(2)
                                .min_decimals(2),
                        );

                        if self.focus_on == Some(FocusOn::Amount) {
                            self.focus_on.take();
                            res.request_focus();
                        }

                        self.transfer_amount = (transfer_amount * 100.0) as u64;
                    });

                    ui.horizontal(|ui| {
                        egui::Label::new(RichText::new("From: ").strong())
                            .selectable(false)
                            .ui(ui);
                        let res = ui.label(format!("{}", self.from_account));

                        if enter_pressed || res.lost_focus() {
                            enter_pressed = false;
                            focus_to = true;
                        }
                    });

                    ui.horizontal(|ui| {
                        egui::Label::new(RichText::new("To: ").strong())
                            .selectable(false)
                            .ui(ui);

                        let res = egui::ComboBox::from_id_source("move_money_to_account")
                            .selected_text(format!("{}", self.to_account))
                            .show_ui(ui, |ui| {
                                ui.style_mut().wrap = Some(false);
                                ui.set_min_width(60.0);

                                let account_type = match self.from_account {
                                    AccountType::Wallet => AccountType::Savings,
                                    _ => AccountType::Wallet,
                                };
                                ui.selectable_value(
                                    &mut self.to_account,
                                    account_type,
                                    format!("{}", account_type),
                                );
                            });

                        if enter_pressed || res.response.lost_focus() {
                            enter_pressed = false;
                            focus_transfer = true;
                        }
                    });

                    ui.add_space(10.0);

                    ui.horizontal(|ui| {
                        let res = ui.button(RichText::new("Transfer").strong());

                        if focus_transfer {
                            res.request_focus();
                        }

                        if res.clicked() {
                            should_transfer = true;
                        }

                        if ui.button(RichText::new("Cancel").strong()).clicked() {
                            self.mode = Mode::Summary;
                        }
                    });
                });
            });

        if should_transfer {
            if self.transfer_amount == 0 {
                self.show_dialog(ui, "Invalid Input", "You must actually transfer an amount");
            } else {
                self.transfer(ui, frame);
                self.description.clear();
            }
        }

        if !is_open {
            self.mode = Mode::Summary;
        }
    }
}
