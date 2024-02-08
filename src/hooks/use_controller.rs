use std::collections::HashMap;

use gilrs::{ev::state::GamepadState, Button, ConnectedGamepadsIterator, Gamepad, GamepadId, Gilrs};
use dioxus::prelude::*;

pub struct ControllerState {
  buttons_pressed: Signal<GamepadState>,

}


#[derive(Clone, Copy)]
pub struct ControllerManager {
  pub gilrs: Signal<Gilrs>,
  // gamepads: Signal<HashMap<GamepadId, ControllerState>>,
}

impl ControllerManager {
  // pub fn gamepads(self) -> ConnectedGamepadsIterator<'static> {
  //   self.gilrs.read().gamepads()
  // }

  // pub fn gamepad(self, gamepad: GamepadId) -> Gamepad<'static> {
  //   self.gilrs.read().gamepad(gamepad)
  // }

}

pub fn use_controller() -> ControllerManager {
  let mut gilrs = use_signal(|| Gilrs::new().unwrap());
  // let mut gamepads = use_signal(HashMap::new);

  spawn(async move {
    loop {
      while let Some(event) = gilrs.write().next_event() {

      }

      // for (id, gamepad) in gilrs.read().gamepads() {
      //   gamepad.
      // }
    }
  });

  ControllerManager { gilrs, }
}