use eframe::egui;
use crate::app::AdvancedBrowserApp;

pub fn build_ui(ctx: &egui::Context, app: &mut AdvancedBrowserApp) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Advanced WebKit Browser");

        ui.horizontal(|ui| {
            if ui.button("Back").clicked() {
                app.navigate_back();
            }

            if ui.button("Forward").clicked() {
                app.navigate_forward();
            }

            ui.text_edit_singleline(&mut app.url);

            if ui.button("Go").clicked() {
                app.navigate_to(app.url.clone());
            }

            if ui.button("Bookmark").clicked() {
                // Placeholder for bookmark functionality
                println!("Bookmarking: {}", app.url);
            }
        });

        ui.add(egui::Spinner::new());
    });
}
