use std::{collections::HashMap, time::Duration};

use gilrs::{ev::state::GamepadState, Button, ConnectedGamepadsIterator, Gamepad, GamepadId, Gilrs};
use dioxus::prelude::*;
use async_std::stream::{interval, StreamExt};

pub struct ControllerState {
  buttons_pressed: Signal<GamepadState>,

}


#[derive(Clone, Copy)]
pub struct ControllerManager {
  pub gilrs: Signal<Gilrs>,
  pub current_gamepad: Signal<Option<GamepadId>>
  // gamepads: Signal<HashMap<GamepadId, ControllerState>>,
}

impl ControllerManager {
  // pub fn gamepads(self) -> ConnectedGamepadsIterator<'static> {
  //   self.gilrs.read().gamepads()
  // }

  // pub fn gamepad(self, gamepad: GamepadId) -> Gamepad<'static> {
  //   self.gilrs.read().gamepad(gamepad)
  // }

  pub fn get_gamepad<'a>(&self) -> Option<GamepadState> {
    if let Some(id) = self.current_gamepad.read().as_ref() {
      return Some(self.gilrs.read().gamepad(*id).state().clone());
    }
    None
  }

}

pub fn use_controller(delay: Option<u64>) -> ControllerManager {
  let mut gilrs = use_signal(|| Gilrs::new().unwrap());
  let mut current_gamepad = use_signal(|| None);
  // let mut gamepads = use_signal(HashMap::new);
  let mut timer = delay.map(|f| interval(Duration::from_millis(f)));

  spawn(async move {
    loop {
      while let Some(event) = gilrs.write().next_event() {
        if current_gamepad.read().is_none() {
          *current_gamepad.write() = Some(event.id);
        }
      }

      if let Some(timer) = timer.as_mut() {
        timer.next().await;
      }
    }
  });

  ControllerManager { gilrs, current_gamepad }
}