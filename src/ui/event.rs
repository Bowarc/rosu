#[derive(Copy, Clone, Debug)]
pub enum Event {
    MousePress {
        button: ggez::input::mouse::MouseButton,
        position: crate::maths::Point,
    },
    MouseRelease {
        button: ggez::input::mouse::MouseButton,
        position: crate::maths::Point,
    },
    MouseMotion {
        position: crate::maths::Point,
        delta: crate::maths::Vec2,
    },
    MouseWheel {
        delta: crate::maths::Point,
    },
    KeyDown {
        key: ggez::input::keyboard::KeyInput,
        repeated: bool,
    },
    KeyUp {
        key: ggez::input::keyboard::KeyInput,
    },
    TextInput {
        character: char,
    },
}
