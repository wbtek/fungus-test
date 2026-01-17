use eframe::egui;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct App { val: f32 }
impl Default for App { fn default() -> Self { Self { val: 0.0 } } }

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Fungus Terminal");
            ui.add(egui::Slider::new(&mut self.val, 0.0..=100.0));
        });
    }
}

// This is your C-style main()
#[wasm_bindgen(start)]
pub async fn start_app() -> Result<(), JsValue> {
    eframe::WebRunner::new().start(
        "main_canvas", 
        eframe::WebOptions::default(), 
        Box::new(|_| Box::new(App::default()))
    ).await
}
