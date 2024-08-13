mod app;
mod browser;
mod ui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Advanced WebKit Browser",
        options,
        Box::new(|_cc| Box::new(app::AdvancedBrowserApp::default())),
    )
}
