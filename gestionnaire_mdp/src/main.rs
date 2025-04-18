mod interface_gui;
use interface_gui::MonApp;
mod chiffrement;
fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Mon gestionnaire de mots de passe",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Box::new(MonApp::default())),
    )
}
