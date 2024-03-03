use egui::{Align2, Color32, Margin, RichText, Rounding, Stroke, Vec2, Widget};
use immutable_bank_model::{account::AccountType, pretty::pretty_print_cents};

use crate::{state::local_app::FocusOn, LocalApp};

use super::Mode;

impl LocalApp {
    fn render_account(&mut self, ui: &mut egui::Ui, index: usize) {
        egui::Frame::default()
            .rounding(Rounding::default().at_least(3.0))
            .inner_margin(Margin::same(8.0))
            .outer_margin(Margin::same(3.0))
            .stroke(Stroke::new(1.0, Color32::DARK_GRAY))
            .show(ui, |ui| {
                let account = self.bank().and_then(|s| s.accounts.get(index));
                let account = match account {
                    Some(account) => account,
                    None => return,
                };

                let account_name = format!("{}", &account.type_);
                egui::Label::new(RichText::new(&account_name).strong())
                    .selectable(false)
                    .ui(ui);

                ui.horizontal(|ui| {
                    ui.add_space(200.0);
                });
                ui.horizontal(|ui| {
                    egui::Label::new("Balance: ").selectable(false).ui(ui);

                    let amount_str = pretty_print_cents(account.balance_cents);
                    egui::Label::new(RichText::new(&amount_str).strong())
                        .selectable(false)
                        .ui(ui);
                });

                let account_type = account.type_.clone();
                ui.horizontal(|ui| {
                    if account_type.can_move_money() {
                        if ui.button("Move Money").clicked() {
                            self.transfer_amount = 0;
                            self.from_account = account_type;
                            self.to_account = match account_type {
                                AccountType::Wallet => AccountType::Savings,
                                _ => AccountType::Wallet,
                            };
                            self.to_bank = Default::default();
                            self.description.clear();
                            self.mode = Mode::MoveMoney;
                            self.focus_on.replace(FocusOn::Amount);
                        }
                    }

                    if account_type.can_send_money() {
                        if ui.button("Send Money").clicked() {
                            self.transfer_amount = 0;
                            self.from_account = AccountType::Wallet;
                            self.to_account = AccountType::Wallet;
                            self.to_bank = Default::default();
                            self.description.clear();
                            self.mode = Mode::SendMoney;
                            self.focus_on.replace(FocusOn::Amount);
                        }
                    }
                });
            });
    }

    pub fn render_bank_summary(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let bank_id = self
            .session
            .as_ref()
            .map(|s| s.bank_id.clone())
            .clone()
            .unwrap_or_else(|| "My Bank".to_string().into());
        egui::Window::new(bank_id.as_str())
            .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
            .default_size(Vec2::new(200.0, 600.0))
            .resizable(false)
            .collapsible(false)
            .show(ui.ctx(), |ui| {
                let num_accounts = self.bank().map(|b| b.accounts.len()).unwrap_or_default();
                for index in 0..num_accounts {
                    self.render_account(ui, index);
                }

                egui::Frame::default()
                    .rounding(Rounding::default().at_least(3.0))
                    .inner_margin(Margin::same(8.0))
                    .outer_margin(Margin::same(3.0))
                    .stroke(Stroke::new(1.0, Color32::DARK_GRAY))
                    .show(ui, |ui| {
                        egui::Label::new(RichText::new("Transaction History").strong())
                            .selectable(false)
                            .ui(ui);

                        ui.horizontal(|ui| {
                            ui.add_space(200.0);
                        });
                        egui::ScrollArea::vertical()
                            .enable_scrolling(true)
                            .drag_to_scroll(true)
                            .vscroll(true)
                            .max_height(200.0)
                            .show(ui, |ui| {
                                let me = self.bank().map(|s| s.owner.clone()).unwrap_or_default();
                                for transaction in
                                    self.ledger.transactions_for(me.as_str()).iter().rev()
                                {
                                    egui::Frame::default()
                                        .rounding(Rounding::default().at_least(3.0))
                                        .inner_margin(Margin::same(8.0))
                                        .outer_margin(Margin::same(3.0))
                                        .stroke(Stroke::new(1.0, Color32::DARK_GRAY))
                                        .show(ui, |ui| {
                                            if !transaction.description.is_empty() {
                                                egui::Label::new(&transaction.description)
                                                    .selectable(false)
                                                    .wrap(true)
                                                    .ui(ui);
                                                ui.add_space(5.0);
                                            }

                                            ui.horizontal(|ui| {
                                                egui::Label::new(
                                                    RichText::new("Amount: ").strong(),
                                                )
                                                .selectable(false)
                                                .ui(ui);
                                                let direction = format!(
                                                    "{}",
                                                    pretty_print_cents(transaction.amount_cents)
                                                );
                                                egui::Label::new(direction)
                                                    .selectable(false)
                                                    .ui(ui);
                                            });

                                            ui.horizontal(|ui| {
                                                egui::Label::new(RichText::new("From: ").strong())
                                                    .selectable(false)
                                                    .ui(ui);
                                                let direction = if transaction.from.bank == bank_id
                                                {
                                                    format!("({})", transaction.from.account)
                                                } else {
                                                    format!("[{}]", transaction.from.bank)
                                                };
                                                egui::Label::new(direction)
                                                    .selectable(false)
                                                    .ui(ui);
                                            });

                                            ui.horizontal(|ui| {
                                                egui::Label::new(RichText::new("To: ").strong())
                                                    .selectable(false)
                                                    .ui(ui);
                                                let direction = if transaction.to.bank == bank_id {
                                                    format!("({})", transaction.to.account)
                                                } else {
                                                    format!("[{}]", transaction.to.bank)
                                                };
                                                egui::Label::new(direction)
                                                    .selectable(false)
                                                    .ui(ui);
                                            });
                                        });
                                }
                            });
                    });
            });
    }
}
