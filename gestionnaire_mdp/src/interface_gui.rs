use eframe::egui;

pub struct MonApp;

impl Default for MonApp {
    fn default() -> Self {
        MonApp
    }
}

impl eframe::App for MonApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Bienvenue dans l'interface graphique !");
        });
    }
}
