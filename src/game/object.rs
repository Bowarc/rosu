use crate::render;
#[derive(Debug)]
pub struct Object {
    inner: crate::osu_format::HitObject,
    lifetime: time::DTDelay,
    size: f32,
}

impl Object {
    pub fn new(inner: crate::osu_format::HitObject, size: f32) -> Self {
        Self {
            inner,
            lifetime: time::DTDelay::new(0.5),
            size,
        }
    }
    pub fn base(&self) -> &crate::osu_format::HitObjectBase {
        self.inner.base()
    }
    pub fn is_dead(&self) -> bool {
        self.lifetime.ended()
    }

    pub fn update(&mut self, dt: f64) {
        self.lifetime.update(dt);
    }

    pub fn draw(
        &self,
        ggctx: &mut ggez::Context,
        render_request: &mut crate::render::RenderRequest,
        window_scale: crate::maths::Point,
    ) {
        // basic node
        render_request.add(
            ggez::graphics::Mesh::new_circle(
                ggctx,
                ggez::graphics::DrawMode::stroke(1.),
                crate::maths::Point::new(self.base().x.into(), self.base().y.into()) * window_scale,
                self.size * 5. * ((window_scale.x + window_scale.y) / 2.) as f32,
                0.1,
                crate::render::Color::WHITE.into(),
            )
            .unwrap(),
            crate::render::DrawParam::default(),
            crate::render::Layer::Game,
        );
        match &self.inner {
            crate::osu_format::HitObject::Circle { base } => {}
            crate::osu_format::HitObject::Slider {
                base: _,
                curve_points,
                slider_type: _,
                repeat: _,
                edge_hitsound: _,
                edge_addition: _,
            } => {
                for (i, curve_point) in curve_points.iter().enumerate() {
                    let last_pt = if i == 0 {
                        (self.base().x, self.base().y)
                    } else {
                        *curve_points.get(i - 1).unwrap()
                    };

                    render_request.add(
                        ggez::graphics::Mesh::new_line(
                            ggctx,
                            &[
                                crate::maths::Point::new(
                                    curve_point.0 as f64,
                                    curve_point.1 as f64,
                                ) * window_scale,
                                crate::maths::Point::new(last_pt.0 as f64, last_pt.1 as f64)
                                    * window_scale,
                            ],
                            10.,
                            render::Color::WHITE.into(),
                        )
                        .unwrap(),
                        crate::render::DrawParam::default(),
                        crate::render::Layer::Game,
                    );
                    render_request.add(
                        ggez::graphics::Mesh::new_circle(
                            ggctx,
                            ggez::graphics::DrawMode::stroke(1.),
                            crate::maths::Point::new(curve_point.0 as f64, curve_point.1 as f64)
                                * window_scale,
                            self.size * 3. * ((window_scale.x + window_scale.y) / 2.) as f32,
                            0.1,
                            crate::render::Color::WHITE.into(),
                        )
                        .unwrap(),
                        crate::render::DrawParam::default(),
                        crate::render::Layer::Game,
                    );
                }
            }
            crate::osu_format::HitObject::Spinner { base, end_time } => todo!(),
            _ => {
                unimplemented!("")
            }
        }
    }
}
