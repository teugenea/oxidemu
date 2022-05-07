mod main_win;
mod render;

pub fn create_main_winow() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Show an image with eframe/egui",
        options,
        Box::new(|_cc| Box::new(main_win::OxidemuApp::default()))
    );
}
