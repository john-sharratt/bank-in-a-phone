use egui::{Color32, Margin, Rounding, Stroke, Ui};
use std::sync::mpsc::TryRecvError;

use crate::LocalApp;

impl LocalApp {
    pub fn render_pending(&mut self, ui: &mut Ui, frame: &mut eframe::Frame) {
        egui::Frame::default()
            .rounding(Rounding::default().at_least(3.0))
            .inner_margin(Margin::same(8.0))
            .outer_margin(Margin::same(3.0))
            .stroke(Stroke::new(1.0, Color32::DARK_GRAY))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Processing... Please wait.");
                    ui.spinner();
                });
            });

        // Check if the pending operation has finished, if it has then we need to change the mode
        if let Some(pending) = self.pending.as_mut() {
            let res = match pending.try_recv() {
                Ok(res) => res,
                Err(TryRecvError::Disconnected) => Err(anyhow::anyhow!("Request aborted")),
                Err(TryRecvError::Empty) => {
                    return;
                }
            };
            self.pending.take();

            // If there's an error then show the dialog, otherwise we
            // move to the next mode
            match res {
                Ok(data) => {
                    if let Some(callback) = self.pending_callback.take() {
                        if let Err(err) = callback(data, self, frame) {
                            self.show_error(ui, "Request failed", err);
                        }
                    }
                }
                Err(err) => self.show_error(ui, "Request failed", err),
            }
        }
    }
}
