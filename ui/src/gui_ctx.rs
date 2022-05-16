use json_gettext::JSONGetText;
use glium::backend::Facade;
use imgui::{ Ui, Textures };
use imgui_glium_renderer::Texture;

pub enum GuiMode {
    GAME,
    DEBUG,
}

pub struct UiState {
    pub open_file: bool,
    pub gui_mode: GuiMode,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            open_file: false,
            gui_mode: GuiMode::GAME,
        }
    }
}

pub struct GuiCtx<'a> {
    ui: &'a Ui<'a>, 
    textures: &'a mut Textures<Texture>, 
    facade: &'a dyn Facade,
    local: &'a JSONGetText<'a>,
    state: &'a mut UiState,
    work_size: [f32; 2],
    work_pos: [f32; 2],
}

impl<'a> GuiCtx<'a> {
    
    pub fn new(
        ui: &'a Ui<'a>, 
        textures: &'a mut Textures<Texture>, 
        facade: &'a dyn Facade,
        local: &'a JSONGetText<'a>,
        state: &'a mut UiState, 
        work_size: [f32; 2], 
        work_pos: [f32; 2]
    ) -> Self {
        Self {
            ui, textures, facade, local, state, work_size, work_pos
        }
    }

    pub fn ui(&'a self) -> &'a Ui {
        self.ui
    }

    pub fn textures(&'a mut self) -> &'a mut Textures<Texture> {
        self.textures
    }

    pub fn facade(&'a self) -> &'a dyn Facade {
        self.facade
    }

    pub fn localize(&'a self, text: &str) -> String {
        let txt = self.local.get_text(text);
        match txt {
            Some(t) => String::from(t.as_str().unwrap()),
            _ => String::from(text)
        }
    }

    pub fn state(&'a mut self) -> &'a mut UiState {
        self.state
    }

    pub fn work_pos(&self) -> &[f32; 2] {
        &self.work_pos
    }

    pub fn work_size(&self) -> &[f32; 2] {
        &self.work_size
    }
}