pub mod dummy;
pub mod game_end;
pub mod game_start;
pub mod main_menu;
pub mod playing;
pub mod song_select;

use dummy::__Dummy;
use game_end::GameEnd;
use game_start::GameStart;
use main_menu::MainMenu;
use playing::Playing;
use song_select::SongSelect;

#[enum_dispatch::enum_dispatch]
pub trait StateMachine: Sized {
    fn update(self, ggctx: &mut ggez::Context, sound_bank: &mut crate::assets::sound::SoundBank, delta_time: f64, ) -> State;
    fn draw(self, _: &mut ggez::Context, _: &mut crate::render::RenderRequest) -> State;

    fn try_get_ui_mgr_mut(&mut self) -> Option<&mut crate::ui::UiManager> {
        None
    }
}

#[enum_dispatch::enum_dispatch(StateMachine)]
#[derive(enum_variant_name::VariantName)]
pub enum State {
    __Dummy,
    MainMenu,
    SongSelect,
    GameStart,
    Playing,
    GameEnd,
}

impl Default for State {
    fn default() -> Self {
        MainMenu::new().into()
    }
}

impl State {
    pub fn dummy() -> Self {
        __Dummy.into()
    }
}
