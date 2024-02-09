/*
    Used to target an abstract position
*/
#[derive(Clone, Copy, Debug)]
pub enum Anchor {
    CenterCenter,
    Topleft,
    TopCenter,
    TopRight,
    RightCenter,
    BotRight,
    BotCenter,
    BotLeft,
    LeftCenter,
}

impl Anchor {
    /// Computes and returns the center point of the element
    pub fn compute(
        &self,
        drawable_size: crate::maths::Point,
        element_size: crate::maths::Point,
    ) -> crate::maths::Point {
        match self {
            Anchor::CenterCenter => drawable_size * 0.5,
            Anchor::Topleft => element_size * 0.5,
            Anchor::TopCenter => {
                crate::maths::Point::new(drawable_size.x * 0.5, element_size.y * 0.5)
            }
            Anchor::TopRight => crate::maths::Point::new(
                drawable_size.x - element_size.x * 0.5,
                element_size.y * 0.5,
            ),
            Anchor::RightCenter => crate::maths::Point::new(
                drawable_size.x - element_size.x * 0.5,
                drawable_size.y * 0.5,
            ),
            Anchor::BotRight => drawable_size - element_size * 0.5,
            Anchor::BotCenter => crate::maths::Point::new(
                drawable_size.x * 0.5,
                drawable_size.y - element_size.y * 0.5,
            ),
            Anchor::BotLeft => crate::maths::Point::new(
                element_size.x * 0.5,
                drawable_size.y - element_size.y * 0.5,
            ),
            Anchor::LeftCenter => {
                crate::maths::Point::new(element_size.x * 0.5, drawable_size.y * 0.5)
            }
        }

        // match self {
        // Anchor::CenterCenter => {
        // crate::maths::Point::new(drawable_size.x * 0.5, drawable_size.y * 0.5)
        // - element_size * 0.5
        // }
        // Anchor::Topleft => crate::maths::Point::ZERO,
        // Anchor::TopCenter => {
        // crate::maths::Point::new(drawable_size.x * 0.5 - element_size.x * 0.5, 0.)
        // }
        // Anchor::TopRight => crate::maths::Point::new(drawable_size.x - element_size.x, 0.),
        // Anchor::RightCenter => crate::maths::Point::new(
        // drawable_size.x - element_size.x,
        // drawable_size.y * 0.5 - element_size.y * 0.5,
        // ),
        // Anchor::BotRight => crate::maths::Point::new(
        // drawable_size.x - element_size.x,
        // drawable_size.y - element_size.y,
        // ),
        // Anchor::BotCenter => crate::maths::Point::new(
        // drawable_size.x * 0.5 - element_size.x * 0.5,
        // drawable_size.y - element_size.y,
        // ),
        // Anchor::BotLeft => crate::maths::Point::new(0., drawable_size.y - element_size.y),
        // Anchor::LeftCenter => {
        // crate::maths::Point::new(0., drawable_size.y * 0.5 - element_size.y * 0.5)
        // }
        // }
    }
    /// Returns the center point
    pub fn as_value(&self, size: impl Into<super::Vector>) -> super::Vector {
        use super::value::MagicValue;
        let size = size.into();
        match self {
            Anchor::CenterCenter => {
                super::Vector::new(MagicValue::ScreenSizeW * 0.5, MagicValue::ScreenSizeH * 0.5)
            }
            Anchor::Topleft => size * 0.5,
            Anchor::TopCenter => super::Vector::new(MagicValue::ScreenSizeW * 0.5, size.y() * 0.5),
            Anchor::TopRight => {
                super::Vector::new(MagicValue::ScreenSizeW - size.x() * 0.5, size.y() * 0.5)
            }
            Anchor::RightCenter => super::Vector::new(
                MagicValue::ScreenSizeW - size.x() * 0.5,
                MagicValue::ScreenSizeH * 0.5,
            ),
            Anchor::BotRight => {
                super::Vector::new(MagicValue::ScreenSizeW, MagicValue::ScreenSizeH) - size * 0.5
            }
            Anchor::BotCenter => super::Vector::new(
                MagicValue::ScreenSizeW * 0.5,
                MagicValue::ScreenSizeH - size.y() * 0.5,
            ),
            Anchor::BotLeft => {
                super::Vector::new(size.x() * 0.5, MagicValue::ScreenSizeH - size.y() * 0.5)
            }
            Anchor::LeftCenter => super::Vector::new(size.x() * 0.5, MagicValue::ScreenSizeW * 0.5),
        }

        // match self{
        //     Anchor::CenterCenter => (MagicValue::ScreenSizeW*0.5, MagicValue::ScreenSizeH * 0.5),
        //     Anchor::Topleft => (0f64.into(), 0f64.into()),
        //     Anchor::TopCenter => (MagicValue::ScreenSizeW*0.5, 0f64.into()),
        //     Anchor::TopRight => (MagicValue::ScreenSizeW.into(), 0f64.into()),
        //     Anchor::RightCenter => (MagicValue::ScreenSizeW.into(), MagicValue::ScreenSizeH * 0.5),
        //     Anchor::BotRight => (MagicValue::ScreenSizeW.into(), MagicValue::ScreenSizeH.into()),
        //     Anchor::BotCenter =>(MagicValue::ScreenSizeW * 0.5, MagicValue::ScreenSizeH.into()),
        //     Anchor::BotLeft =>(0f64.into(), MagicValue::ScreenSizeH.into()),
        //     Anchor::LeftCenter => (0f64.into(), MagicValue::ScreenSizeH * 0.5),
        // }
    }
}
