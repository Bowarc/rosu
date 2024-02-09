use ggez::audio as ggaudio;

pub struct SoundSource(pub ggaudio::Source, pub super::SoundId);


impl SoundSource {
    // pub fn get_static_source_mut(&mut self) -> &mut ggaudio::Source {
    //     match &mut self {
    //         SoundSource::Static(source, _) => source,
    //         _ => panic!("Tried to get a raw static source out of a Spatial SoundSource"),
    //     }
    // }
    // pub fn get_dyamic_source_mut(&mut self) -> &mut ggaudio::SpatialSource {
    //     match &mut self {
    //         SoundSource::Spatial(source, _, _) => source,
    //         _ => panic!("Tried to get a raw spatial source out of a Static SoundSource"),
    //     }
    // }

    pub fn get_sound_id(&self) -> super::SoundId {
        self.1 
    }
}

#[allow(unused)]
impl ggaudio::SoundSource for SoundSource {
    fn play(
        &mut self,
        audio: &impl ggez::context::Has<ggez::audio::AudioContext>,
    ) -> ggez::GameResult {
        use ggaudio::SoundSource as _;
        self.0.play(audio)
    }

    fn play_later(&self) -> ggez::GameResult {
        use ggaudio::SoundSource as _;
        self.0.play_later()
    }
    fn play_detached(
        &mut self,
        audio: &impl ggez::context::Has<ggez::audio::AudioContext>,
    ) -> ggez::GameResult {
        use ggaudio::SoundSource as _;
        self.0.play_detached(audio)
    }
    fn set_repeat(&mut self, repeat: bool) {
        use ggaudio::SoundSource as _;
        self.0.set_repeat(repeat)
    }
    fn set_fade_in(&mut self, dur: std::time::Duration) {
        use ggaudio::SoundSource as _;
        self.0.set_fade_in(dur)
    }
    fn set_start(&mut self, dur: std::time::Duration) {
        use ggaudio::SoundSource as _;
        self.0.set_start(dur)
    }
    fn set_pitch(&mut self, ratio: f32) {
        use ggaudio::SoundSource as _;
        self.0.set_pitch(ratio)
    }
    fn repeat(&self) -> bool {
        use ggaudio::SoundSource as _;
        self.0.repeat()
    }
    fn pause(&self) {
        use ggaudio::SoundSource as _;
        self.0.pause()
    }
    fn resume(&self) {
        use ggaudio::SoundSource as _;
        self.0.resume()
    }
    fn stop(&mut self, audio: &impl ggez::context::Has<ggaudio::AudioContext>) -> ggez::GameResult {
        self.0.stop(audio)
    }

    fn stopped(&self) -> bool {
        use ggaudio::SoundSource as _;
        self.0.stopped()
    }
    fn volume(&self) -> f32 {
        use ggaudio::SoundSource as _;
        self.0.volume()
    }
    fn set_volume(&mut self, value: f32) {
        use ggaudio::SoundSource as _;
        self.0.set_volume(value)    
    }
    fn paused(&self) -> bool {
        use ggaudio::SoundSource as _;
        self.0.paused()
    }
    fn playing(&self) -> bool {
        use ggaudio::SoundSource as _;
        self.0.playing()
    }
    fn elapsed(&self) -> std::time::Duration {
        use ggaudio::SoundSource as _;
        self.0.elapsed()
    }
    fn set_query_interval(&mut self, t: std::time::Duration) {
        use ggaudio::SoundSource as _;
        self.0.set_query_interval(t)
    }


}
