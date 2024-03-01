use egui::{Align2, Color32, Margin, RichText, Rounding, Stroke, Vec2, Widget};
use immutable_bank_model::{account::AccountType, ledger_type::LedgerType};

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
                let account = self.bank_mut().and_then(|s| s.accounts.get_mut(index));
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
                            self.from_account = AccountType::Wallet;
                            self.to_account = AccountType::Savings;
                            self.to_user = Default::default();
                            self.description.clear();
                            self.mode = Mode::MoveMoney;
                            self.pending.take();
                            self.focus_on.replace(FocusOn::Amount);
                        }
                    }

                    if account_type.can_send_money() {
                        if ui.button("Send Money").clicked() {
                            self.from_account = AccountType::Wallet;
                            self.to_account = AccountType::Wallet;
                            self.to_user = Default::default();
                            self.description.clear();
                            self.mode = Mode::SendMoney;
                            self.pending.take();
                            self.focus_on.replace(FocusOn::Amount);
                        }
                    }
                });
            });
    }

    pub fn render_bank_summary(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let name = self
            .session
            .clone()
            .unwrap_or_else(|| "My Bank".to_string());
        egui::Window::new(name)
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
                                    self.ledger.entries.iter().filter_map(|f| match &f.entry {
                                        LedgerType::Transfer {
                                            local_bank,
                                            transaction,
                                        } if local_bank.as_str() == me.as_str() => {
                                            Some(transaction)
                                        }
                                        _ => None,
                                    })
                                {
                                    egui::Frame::default()
                                        .rounding(Rounding::default().at_least(3.0))
                                        .inner_margin(Margin::same(8.0))
                                        .outer_margin(Margin::same(3.0))
                                        .stroke(Stroke::new(1.0, Color32::DARK_GRAY))
                                        .show(ui, |ui| {
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
                                                let direction = format!("{}", transaction.from);
                                                egui::Label::new(direction)
                                                    .selectable(false)
                                                    .ui(ui);
                                            });

                                            ui.horizontal(|ui| {
                                                egui::Label::new(RichText::new("To: ").strong())
                                                    .selectable(false)
                                                    .ui(ui);
                                                let direction = format!("{}", transaction.to);
                                                egui::Label::new(direction)
                                                    .selectable(false)
                                                    .ui(ui);
                                            });

                                            ui.add_space(5.0);
                                            egui::Label::new(&transaction.description)
                                                .selectable(false)
                                                .wrap(true)
                                                .ui(ui);
                                        });
                                }
                            });
                    });
            });
    }
}

pub fn pretty_print_cents(total_cents: u64) -> String {
    let dollars = total_cents / 100;
    let cents = total_cents % 100;
    format!("${}.{:02}", pretty_print_u64(dollars), cents)
}

fn pretty_print_u64(i: u64) -> String {
    let mut s = String::new();
    let i_str = i.to_string();
    let a = i_str.chars().rev().enumerate();
    for (idx, val) in a {
        if idx != 0 && idx % 3 == 0 {
            s.insert(0, ',');
        }
        s.insert(0, val);
    }
    s
}
