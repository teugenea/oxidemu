mod main_win;

pub fn create_main_winow() {
    let app = main_win::OxidemuApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
