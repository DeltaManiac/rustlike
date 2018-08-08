use core::events::event::KeyboardEvent;
use core::events::event::MouseEvent;
use core::point_2d::Point2d;

pub struct MouseState {
    pub left_button: bool,
    pub middle_button: bool,
    pub right_button: bool,
    pub extra_button_1: bool,
    pub extra_button_2: bool,
    pub extra_button_3: bool,
    pub extra_button_4: bool,
    pub position: Point2d,
}

pub struct KeyboardState {
    pub keys: [bool; 256],
    pub left_control: bool,
    pub left_shift: bool,
    pub left_alt: bool,
    pub right_control: bool,
    pub right_shift: bool,
    pub right_alt: bool,
}

impl MouseState {
    pub fn new() -> MouseState {
        MouseState {
            left_button: false,
            middle_button: false,
            right_button: false,
            extra_button_1: false,
            extra_button_2: false,
            extra_button_3: false,
            extra_button_4: false,
            position: Point2d::empty(),
        }
    }

    pub fn update_from_event(&mut self, mouse: MouseEvent) {
        self.left_button = mouse.left_button;
        self.middle_button = mouse.middle_button;
        self.right_button = mouse.right_button;
        self.extra_button_1 = mouse.extra_button_1;
        self.extra_button_2 = mouse.extra_button_2;
        self.extra_button_3 = mouse.extra_button_3;
        self.extra_button_4 = mouse.extra_button_4;
        self.position = mouse.position;
    }
}

impl KeyboardState {
    pub fn new() -> KeyboardState {
        KeyboardState {
            keys: [false; 256],
            left_control: false,
            left_shift: false,
            left_alt: false,
            right_control: false,
            right_shift: false,
            right_alt: false,
        }
    }

    pub fn update_from_event(&mut self, keyboard: KeyboardEvent) {
        self.left_control = keyboard.left_control;
        self.left_shift = keyboard.left_shift;
        self.left_alt = keyboard.left_alt;
        self.right_control = keyboard.right_control;
        self.right_shift = keyboard.right_shift;
        self.right_alt = keyboard.right_alt;
    }
}
