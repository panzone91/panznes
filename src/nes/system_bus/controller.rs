use crate::Nes;
use crate::nes::NesControllerButton;

impl<'a> Nes<'a> {

    pub fn set_controller_status(&mut self, button: NesControllerButton, is_pressed:bool) {
        self.controller_first_port[button as usize] = is_pressed;
    }

}