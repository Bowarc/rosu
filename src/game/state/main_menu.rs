
pub struct MainMenu {}

impl MainMenu {
    pub fn new() -> Self {
        debug!("Creating MainMenu State");
        Self {}
    }
}

impl super::StateMachine for MainMenu {
    fn update(mut self, ggctx: &mut ggez::Context, sound_bank: &mut crate::assets::sound::SoundBank, delta_time: f64, ) -> super::State {
        self.into()
    }

    fn draw(self, _: &mut ggez::Context,_: &mut crate::render::RenderRequest) -> super::State {
        self.into()
        
    }
}
