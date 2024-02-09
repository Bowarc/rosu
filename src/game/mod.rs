mod state;
use state::State;

pub struct Game {
    state: state::State,
}
impl Game {
    pub fn new(sound_bank: &mut crate::assets::sound::SoundBank) -> Self {
        Self {
            state: State::Playing(state::playing::Playing::new(sound_bank)),
        }
    }
    pub fn try_get_ui_mgr_mut(&mut self) -> Option<&mut crate::ui::UiManager> {
        state::StateMachine::try_get_ui_mgr_mut(&mut self.state)
    }
    pub fn update(&mut self, ggctx: &mut ggez::Context, sound_bank: &mut crate::assets::sound::SoundBank, delta_time: f64) {
        self.state = state::StateMachine::update(
            std::mem::replace(&mut self.state, State::dummy()),
            ggctx,
            sound_bank,
            delta_time,
        );
        self.verify_state()
    }
    pub fn draw(&mut self, ggctx: &mut ggez::Context,render_request: &mut crate::render::RenderRequest) {
        self.state = state::StateMachine::draw(
            std::mem::replace(&mut self.state, State::dummy()),
            ggctx, render_request,
        );
        self.verify_state()
    }
    pub fn verify_state(&self) {
        if let State::__Dummy(_) = self.state {
            panic!("Dummy state detected, you might have forgot to switch it back");
        }
    }
}
