use imgui::Ui;
use crate::GuiCtx;
use imgui::MenuItem;

pub struct MainWindow {
    //rn: GameWindow<'a>,
}

impl MainWindow {

    pub fn new() -> Self {
        Self {
            //rn: GameWindow::new(),
        }
    }
    
    pub fn show(&mut self, gui_ctx: &GuiCtx) {
        //self.rn.show_window(&self.ctx, gui_ctx);
        self.main_menu(gui_ctx);
    }

    fn main_menu(&mut self, gui_ctx: &GuiCtx)  {
        let ui = gui_ctx.ui();
        if let Some(menu_bar) = ui.begin_main_menu_bar() {
            if let Some(menu) = ui.begin_menu("File") {
                MenuItem::new("Open").build(ui);
                ui.separator();
                MenuItem::new("Exit").build(ui);
                menu.end();
            }
            if let Some(menu) = ui.begin_menu("View") {
                
            }
            menu_bar.end();
        }
    }
    
}