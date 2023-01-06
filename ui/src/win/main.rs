use imgui::Ui;
use emulation::common::emulator::EmulMgr;
use crate::GuiCtx;
use imgui::MenuItem;
use super::game::GameWindow;

pub struct MainWindow<'a> {
    rn: GameWindow<'a>,
}

impl<'a> MainWindow<'a> {

    pub fn new() -> Self {
        Self {
            rn: GameWindow::new(),
        }
    }
    
    pub fn show(&mut self, emul: &EmulMgr, ui: &Ui, gui_ctx: &mut GuiCtx) {
        self.rn.show_window(emul, ui, gui_ctx);
        self.main_menu(ui, gui_ctx);
    }

    fn main_menu(&mut self, ui: &Ui, gui_ctx: &GuiCtx)  {
        if let Some(menu_bar) = ui.begin_main_menu_bar() {
            if let Some(menu) = ui.begin_menu("File") {
                ui.menu_item("Open");
                ui.separator();
                ui.menu_item("Exit");
                menu.end();
            }
            if let Some(menu) = ui.begin_menu("View") {
                
            }
            menu_bar.end();
        }
    }
    
}