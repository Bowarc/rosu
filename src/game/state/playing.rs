use std::io::BufRead;
pub struct Playing {
    time_since_start: f64,
    meta: osu_format::BeatmapMetadata,
    difficulty: osu_format::BeatmapDifficulty,
    next_objects: Vec<osu_format::HitObject>,
    current_objects: Vec<osu_format::HitObject>,
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
        let mut p = osu_format::Parser::new(reader.lines());
        let data = p.parse().unwrap();
        debug!("Creating Playing State");
        debug!("{:#?}", data.general);
        debug!("{:#?}", data.metadata);
        debug!("{:#?}", data.difficulty);
        Self {
            time_since_start: -(data.general.preview_time as f64)* 0.01,
            next_objects: data.hit_objects,
            meta: data.metadata,
            difficulty: data.difficulty,
            current_objects: vec![],
        }
    }
}

impl super::StateMachine for Playing {
    fn update(mut self, _ggctx: &mut ggez::Context, _sound_bank: &mut crate::assets::sound::SoundBank, delta_time: f64, ) -> super::State {
        self.time_since_start += delta_time * 1000.;

        if let Some(nxt_obj) = self.next_objects.first() {
            if (nxt_obj.base().time) as f64 <= self.time_since_start {
                let obj = self.next_objects.remove(0);
                debug!("Adding object: {obj:?}");
                self.current_objects.push(obj);
            }else{
                debug!("Next object in: {}", (nxt_obj.base().time) as f64 - self.time_since_start);
            }
        }

        while self.current_objects.len() > 4 {
            self.current_objects.remove(0);
        }

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
            render_request.add(
                ggez::graphics::Mesh::new_circle(
                    ggctx,
                    ggez::graphics::DrawMode::stroke(1.),
                    crate::maths::Point::new(obj.base().x.into(), obj.base().y.into()) * scale,
                    self.difficulty.circle_size * 5. * ((scale.x + scale.y) / 2.) as f32,
                    0.1,
                    crate::render::Color::WHITE.into(),
                )
                .unwrap(),
                crate::render::DrawParam::default(),
                crate::render::Layer::Game,
            )
        }

        self.into()
    }
}
