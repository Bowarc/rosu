pub struct __Dummy;

impl super::StateMachine for __Dummy {
    fn update(mut self, ggctx: &mut ggez::Context, sound_bank: &mut crate::assets::sound::SoundBank, delta_time: f64, ) -> super::State {
        unreachable!("You're not supposed to use that variant")
    }

    fn draw(self,_: &mut ggez::Context, _: &mut crate::render::RenderRequest) -> super::State {
        unreachable!("You're not supposed to use that variant")
    }
}
