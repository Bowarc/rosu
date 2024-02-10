use std::io::BufRead;
pub struct Playing {
    time_since_start: f64,
    meta: crate::osu_format::BeatmapMetadata,
    difficulty: crate::osu_format::BeatmapDifficulty,
    next_objects: Vec<crate::game::object::Object>,
    current_objects: Vec<crate::game::object::Object>,
}

impl Playing {
    pub fn new(sound_bank: &mut crate::assets::sound::SoundBank) -> Self {
        sound_bank.request_play(crate::assets::sound::SoundId::HighScore);
        let f = std::fs::File::open(
            "resources/Panda_Eyes_&_Teminite_-_Highscore_(Fort)_[Game_Over].osu",
        )
        .unwrap();
        let reader = std::io::BufReader::new(f);
        // println!("{:?}", reader.lines().next());

        // panic!("");
        let mut p = crate::osu_format::Parser::new(reader.lines());
        let data = p.parse().unwrap();
        debug!("Creating Playing State");
        debug!("{:#?}", data.general);
        debug!("{:#?}", data.metadata);
        debug!("{:#?}", data.difficulty);
        debug!("{:?}", data.hit_objects.get(74).unwrap());
        Self {
            time_since_start: -(data.general.preview_time as f64) * 0.01,
            next_objects: data
                .hit_objects
                .iter()
                .map(|ho| crate::game::object::Object::new(ho.clone(), data.difficulty.circle_size))
                .collect::<Vec<crate::game::object::Object>>(),
            meta: data.metadata,
            difficulty: data.difficulty,
            current_objects: vec![],
        }
    }
}

impl super::StateMachine for Playing {
    fn update(
        mut self,
        _ggctx: &mut ggez::Context,
        _sound_bank: &mut crate::assets::sound::SoundBank,
        dt: f64,
    ) -> super::State {
        self.time_since_start += dt * 1000.;

        if let Some(nxt_obj) = self.next_objects.first() {
            if (nxt_obj.base().time) as f64 <= self.time_since_start {
                let obj = self.next_objects.remove(0);
                debug!("Adding object: {obj:?}");
                self.current_objects.push(obj);
            } else {
                debug!(
                    "Next object in: {}",
                    (nxt_obj.base().time) as f64 - self.time_since_start
                );
            }
        }

        for obj in  self.current_objects.iter_mut(){
            obj.update(dt);
        }

        self.current_objects.retain(|obj| !obj.is_dead());

        self.into()
    }

    fn draw(
        self,
        ggctx: &mut ggez::Context,
        render_request: &mut crate::render::RenderRequest,
    ) -> super::State {
        let screen_size: crate::maths::Point = ggctx.gfx.drawable_size().into();
        let osu_default_screen_size = crate::maths::Point::new(512., 384.);

        let scale = screen_size / osu_default_screen_size;
        // debug!("{scale:?}");

        for obj in &self.current_objects {
            obj.draw(ggctx, render_request, scale)
        }

        self.into()
    }
}
