mod sound_id;
mod sound_source;


use ggez::audio as ggaudio;

use sound_source::SoundSource;
pub use sound_id::SoundId;

pub struct SoundBank {
    sounds: std::collections::HashMap<SoundId, ggez::audio::SoundData>,
    now_playing: Vec<SoundSource>,
    requests: Vec<SoundId>,

}

impl SoundBank {
    pub fn new(_loader_handle: &mut super::loader::Handle) -> Self {
        Self {
            sounds: std::collections::HashMap::new(),
            now_playing: Vec::new(),
            requests: Vec::new()
        }
    }
    /// Check the loader handle to receive data for sounds that are in loading
    fn receive_loaded_data(&mut self, loader_handle: &mut super::loader::Handle) {
        if let Some(data) = loader_handle.retrieve_data(super::loader::TargetId::Sound) {
            match data.request {
                super::loader::Request::Sound(id) => {
                    debug!("Sound bank received data for {id:?}");

                    let sound_data = ggaudio::SoundData::from_bytes(&data.bytes);
                    if let Some(_old_data) = self.sounds.insert(id, sound_data) {
                        warn!("Saviing sound {id:?} in bank replaced some already loaded data, has it been requested twice ?")
                    }
                }
                super::loader::Request::Sprite(id) => {
                    error!("Sound bank received sprite data with id: {id:?}");
                }
                super::loader::Request::Font(_) => todo!(),
            }
        }
    }
    /// Request a sound to be played asap
    pub fn request_play(&mut self, id: SoundId) {
        self.requests.push(id)
    }
    /// Play a sound without specific position in space
    fn play(
        &mut self,
        audio_ctx: &impl ggez::context::Has<ggaudio::AudioContext>,
        loader_handle: &mut super::loader::Handle,
        id: SoundId,
    ) -> ggez::GameResult<bool> {
        use ggaudio::SoundSource as _;

        let Some(data) = self.sounds.get(&id).cloned() else{
            loader_handle.request(super::loader::Request::Sound(id));

            // i don't want to check what is the content of the error received by Self::play_requested so i'll return Ok(())
            // here so i don't spam the console with an 'normal' error

            return Ok(false)
            // return Err(ggez::GameError::AudioError(format!("Could not get the sound data for id {id:?}")));
        };

        let mut builded = match ggaudio::Source::from_data(audio_ctx, data) {
            Ok(builded) => builded,
            Err(_e) => {
                return Err(ggez::GameError::AudioError(format!(
                    "Could not build sound {id:?} with the saved data."
                )))
            }
        };

        builded.set_volume(0.01);

        builded.play(audio_ctx)?;

        debug!("Playing {id:?} static");

        self.now_playing.push(SoundSource(builded, id));

        Ok(true)
    }
    /// Play requested sounds
    fn play_requested(
        &mut self,
        audio_ctx: &impl ggez::context::Has<ggaudio::AudioContext>,
        loader_handle: &mut super::loader::Handle,
    ) {
        let mut new_requests = Vec::new();

        while let Some(id) = self.requests.pop() {
            let result = self.play(audio_ctx, loader_handle, id);

            match result {
                Ok(played) => {
                    if !played {
                        new_requests.push(id)
                    }else{
                        error!("Succesfully started playing: {id:?}");
                    }
                }
                Err(e) => {
                    error!("{e}");
                    new_requests.push(id);
                }
            }
        }
        self.requests = new_requests;
    }

}

impl super::Bank<SoundId, ggez::audio::Source> for SoundBank {
    fn update(&mut self, ctx: &mut ggez::Context, loader_handle: &mut super::loader::Handle) {
        self.receive_loaded_data(loader_handle);
        self.play_requested(ctx, loader_handle)
    }

    fn try_get_mut(
        &mut self,
        _: &SoundId,
        _: &mut super::loader::Handle,
    ) -> std::option::Option<&mut ggez::audio::Source> {
        todo!()
    }
}
