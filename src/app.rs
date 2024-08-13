use crate::browser::open_webview;
use crate::ui::build_ui;
use eframe::egui;

pub struct AdvancedBrowserApp {
    pub url: String,
    pub history: Vec<String>,
    pub current_page: usize,
}

impl Default for AdvancedBrowserApp {
    fn default() -> Self {
        let home_page = "https://www.example.com".to_string();
        Self {
            url: home_page.clone(),
            history: vec![home_page],
            current_page: 0,
        }
    }
}

impl eframe::App for AdvancedBrowserApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        build_ui(ctx, self);
    }
}

impl AdvancedBrowserApp {
    pub fn navigate_to(&mut self, new_url: String) {
        if self.current_page < self.history.len() - 1 {
            self.history.truncate(self.current_page + 1);
        }
        self.history.push(new_url.clone());
        self.current_page += 1;
        self.url = new_url;
        open_webview(&self.url);
    }

    pub fn navigate_back(&mut self) {
        if self.current_page > 0 {
            self.current_page -= 1;
            self.url = self.history[self.current_page].clone();
            open_webview(&self.url);
        }
    }

    pub fn navigate_forward(&mut self) {
        if self.current_page < self.history.len() - 1 {
            self.current_page += 1;
            self.url = self.history[self.current_page].clone();
            open_webview(&self.url);
        }
    }
}
