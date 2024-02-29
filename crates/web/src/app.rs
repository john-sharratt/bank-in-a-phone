use crate::{render::Mode, state::local_app::LocalApp};

impl LocalApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    pub fn save_state(&mut self, frame: &mut eframe::Frame) {
        if let Some(storage) = frame.storage_mut() {
            eframe::set_value(storage, eframe::APP_KEY, self);
        }
    }

    fn powered_by_egui_and_eframe(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.label("Powered by ");
            ui.hyperlink_to("egui", "https://github.com/emilk/egui");
            ui.label(" and ");
            ui.hyperlink_to(
                "eframe",
                "https://github.com/emilk/egui/tree/master/crates/eframe",
            );
            ui.label(" with ");
            ui.add(egui::github_link_file!(
                "https://github.com/john-sharratt/bank-in-a-phone/blob/master/",
                "Source code."
            ));
        });
    }
}

impl eframe::App for LocalApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let is_web = cfg!(target_arch = "wasm32");
        if !is_web {
            egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
                // The top panel is often a good place for a menu bar:

                egui::menu::bar(ui, |ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                });
            });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Bank in a Phone");

            match self.mode {
                Mode::Create => self.render_create_account(ui, frame),
                Mode::Summary => self.render_bank_summary(ui, frame),
                Mode::MoveMoney => {
                    self.render_move_money(ui, frame)
                },
                Mode::SendMoney => {
                    self.render_send_money(ui, frame)
                },
            }
            self.render_dialog(ui);

            ui.separator();

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    egui::widgets::global_dark_light_mode_buttons(ui);

                    ui.add_space(10.0);

                    if ui.button("Reset").clicked() {
                        *self = Self::default();
                        self.save_state(frame);
                    }
                });
                self.powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}
