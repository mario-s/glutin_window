#![deny(missing_docs)]

//! A Glutin window back-end for the Piston game engine.

extern crate glutin;
extern crate gl;
extern crate window;
extern crate shader_version;
extern crate input;
#[macro_use]
extern crate quack;

// External crates.
use input::{
    keyboard,
    MouseButton,
    Button,
    Input,
    Motion,
};
use window::{
    WindowSettings,
    ShouldClose, Size, PollEvent, SwapBuffers,
    CaptureCursor, DrawSize, Title,
    ExitOnEsc
};
use shader_version::opengl::OpenGL;

/// Contains stuff for game window.
pub struct GlutinWindow {
    /// The window.
    pub window: glutin::Window,
    // Used to compute relative mouse movement.
    last_mouse_pos: Option<(f64, f64)>,
    // The back-end does not remember the title.
    title: String,
    exit_on_esc: bool,
}

impl GlutinWindow {
    /// Creates a new game window for GLFW.
    pub fn new(opengl: OpenGL, settings: WindowSettings) -> GlutinWindow {
        let (major, minor) = opengl.get_major_minor();
        let mut builder = glutin::WindowBuilder::new()
            .with_dimensions(settings.size[0], settings.size[1])
            .with_gl_version((major as u32, minor as u32))
            .with_title(settings.title.clone())
            .with_multisampling(settings.samples as u16);
        if settings.fullscreen {
            builder = builder.with_fullscreen(glutin::get_primary_monitor());
        }
        let window = builder
            .build().unwrap();
        unsafe { window.make_current(); }

        // Load the OpenGL function pointers
        gl::load_with(|s| window.get_proc_address(s));

        GlutinWindow {
            window: window,
            last_mouse_pos: None,
            title: settings.title,
            exit_on_esc: settings.exit_on_esc,
        }
    }

    fn poll_event(&mut self) -> Option<Input> {
        use glutin::Event as E;
        use input::Input;

        if let Some((x, y)) = self.last_mouse_pos {
            return Some(Input::Move(input::Motion::MouseRelative(x, y)));
        }

        match self.window.poll_events().next() {
            None => None,
            Some(E::Resized(w, h)) =>
                Some(Input::Resize(w, h)),
            Some(E::ReceivedCharacter(ch)) =>
                Some(Input::Text(ch.to_string())),
            Some(E::Focused(focused)) =>
                Some(Input::Focus(focused)),
            Some(E::KeyboardInput(glutin::ElementState::Pressed,
                _, Some(key))) =>
                Some(Input::Press(Button::Keyboard(map_key(key)))),
            Some(E::KeyboardInput(glutin::ElementState::Released,
                _, Some(key))) =>
                Some(Input::Release(Button::Keyboard(map_key(key)))),
            Some(E::MouseMoved((x, y))) => {
                self.last_mouse_pos = Some((x as f64, y as f64));
                Some(Input::Move(
                    Motion::MouseCursor(x as f64, y as f64)))
            }
            Some(E::MouseWheel(y)) =>
                Some(Input::Move(
                    Motion::MouseScroll(0.0, y as f64))),
            Some(E::MouseInput(glutin::ElementState::Pressed, button)) =>
                Some(Input::Press(Button::Mouse(map_mouse(button)))),
            Some(E::MouseInput(glutin::ElementState::Released, button)) =>
                Some(Input::Release(Button::Mouse(map_mouse(button)))),
            _ => None,
        }
    }

    /*
    fn capture_cursor(&mut self, enabled: bool) {

    }
    */
}

quack! {
obj: GlutinWindow[]
get:
    fn () -> Size [] {
        if let Some((w, h)) = obj.window.get_outer_size() {
            Size([w, h])
        } else {
            Size([0, 0])
        }
    }
    fn () -> ShouldClose [] { ShouldClose(obj.window.should_close()) }
    fn () -> DrawSize [] {
        if let Some((w, h)) = obj.window.get_inner_size() {
            DrawSize([w, h])
        } else {
            DrawSize([0, 0])
        }
    }
    fn () -> Title [] { Title(obj.title.clone()) }
    fn () -> ExitOnEsc [] { ExitOnEsc(obj.exit_on_esc) }
set:
    // fn (val: CaptureCursor) [] {}
    // fn (val: ShouldClose) [] {}
    fn (val: Title) [] {
        obj.title = val.0;
        obj.window.set_title(&obj.title[]);
    }
    fn (val: ExitOnEsc) [] { obj.exit_on_esc = val.0; }
action:
    fn (__: PollEvent) -> Option<Input> [] { obj.poll_event() }
    fn (__: SwapBuffers) -> () [] { obj.window.swap_buffers(); }
}

/// Maps Glutin's key to Piston's key.
pub fn map_key(keycode: glutin::VirtualKeyCode) -> keyboard::Key {
    use input::keyboard::Key;
    use glutin::VirtualKeyCode as K;

    match keycode {
        // K::Num0 => Key::D0,
        // K::Num1 => Key::D1,
        // K::Num2 => Key::D2,
        // K::Num3 => Key::D3,
        // K::Num4 => Key::D4,
        // K::Num5 => Key::D5,
        // K::Num6 => Key::D6,
        // K::Num7 => Key::D7,
        // K::Num8 => Key::D8,
        // K::Num9 => Key::D9,
        K::A => Key::A,
        K::B => Key::B,
        K::C => Key::C,
        K::D => Key::D,
        K::E => Key::E,
        K::F => Key::F,
        K::G => Key::G,
        K::H => Key::H,
        K::I => Key::I,
        K::J => Key::J,
        K::K => Key::K,
        K::L => Key::L,
        K::M => Key::M,
        K::N => Key::N,
        K::O => Key::O,
        K::P => Key::P,
        K::Q => Key::Q,
        K::R => Key::R,
        K::S => Key::S,
        K::T => Key::T,
        K::U => Key::U,
        K::V => Key::V,
        K::W => Key::W,
        K::X => Key::X,
        K::Y => Key::Y,
        K::Z => Key::Z,
        K::Apostrophe => Key::Unknown,
        K::Backslash => Key::Backslash,
        // K::Backspace => Key::Backspace,
        // K::CapsLock => Key::CapsLock,
        K::Delete => Key::Delete,
        K::Comma => Key::Comma,
        K::Down => Key::Down,
        K::End => Key::End,
        // K::Enter => Key::Return,
        // K::Equal => Key::Equals,
        K::Escape => Key::Escape,
        K::F1 => Key::F1,
        K::F2 => Key::F2,
        K::F3 => Key::F3,
        K::F4 => Key::F4,
        K::F5 => Key::F5,
        K::F6 => Key::F6,
        K::F7 => Key::F7,
        K::F8 => Key::F8,
        K::F9 => Key::F9,
        K::F10 => Key::F10,
        K::F11 => Key::F11,
        K::F12 => Key::F12,
        K::F13 => Key::F13,
        K::F14 => Key::F14,
        K::F15 => Key::F15,
        // K::F16 => Key::F16,
        // K::F17 => Key::F17,
        // K::F18 => Key::F18,
        // K::F19 => Key::F19,
        // K::F20 => Key::F20,
        // K::F21 => Key::F21,
        // K::F22 => Key::F22,
        // K::F23 => Key::F23,
        // K::F24 => Key::F24,
        // Possibly next code.
        // K::F25 => Key::Unknown,
        // K::Kp0 => Key::NumPad0,
        // K::Kp1 => Key::NumPad1,
        // K::Kp2 => Key::NumPad2,
        // K::Kp3 => Key::NumPad3,
        // K::Kp4 => Key::NumPad4,
        // K::Kp5 => Key::NumPad5,
        // K::Kp6 => Key::NumPad6,
        // K::Kp7 => Key::NumPad7,
        // K::Kp8 => Key::NumPad8,
        // K::Kp9 => Key::NumPad9,
        // K::KpDecimal => Key::NumPadDecimal,
        // K::KpDivide => Key::NumPadDivide,
        // K::KpMultiply => Key::NumPadMultiply,
        // K::KpSubtract => Key::NumPadMinus,
        // K::KpAdd => Key::NumPadPlus,
        // K::KpEnter => Key::NumPadEnter,
        // K::KpEqual => Key::NumPadEquals,
        // K::LeftShift => Key::LShift,
        // K::LeftControl => Key::LCtrl,
        // K::LeftAlt => Key::LAlt,
        // K::LeftSuper => Key::LGui,
        // K::RightShift => Key::RShift,
        // K::RightControl => Key::RCtrl,
        // K::RightAlt => Key::RAlt,
        // K::RightSuper => Key::RGui,
        // Map to backslash?
        // K::GraveAccent => Key::Unknown,
        K::Home => Key::Home,
        K::Insert => Key::Insert,
        K::Left => Key::Left,
        // K::LeftBracket => Key::LeftBracket,
        // K::Menu => Key::Menu,
        K::Minus => Key::Minus,
        // K::NumLock => Key::NumLockClear,
        K::PageDown => Key::PageDown,
        K::PageUp => Key::PageUp,
        K::Pause => Key::Pause,
        K::Period => Key::Period,
        // K::PrintScreen => Key::PrintScreen,
        K::Right => Key::Right,
        // K::RightBracket => Key::RightBracket,
        // K::ScrollLock => Key::ScrollLock,
        K::Semicolon => Key::Semicolon,
        K::Slash => Key::Slash,
        K::Space => Key::Space,
        K::Tab => Key::Tab,
        K::Up => Key::Up,
        // K::World1 => Key::Unknown,
        // K::World2 => Key::Unknown,
        _ => Key::Unknown,
    }
}

/// Maps Glutin's mouse button to Piston's mouse button.
pub fn map_mouse(mouse_button: glutin::MouseButton) -> MouseButton {
    use glutin::MouseButton as M;

    match mouse_button {
        M::Left => MouseButton::Left,
        M::Right => MouseButton::Right,
        M::Middle => MouseButton::Middle,
        M::Other(0) => MouseButton::X1,
        M::Other(1) => MouseButton::X2,
        M::Other(2) => MouseButton::Button6,
        M::Other(3) => MouseButton::Button7,
        M::Other(4) => MouseButton::Button8,
        _ => MouseButton::Unknown
    }
}