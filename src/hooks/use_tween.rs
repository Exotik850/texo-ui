use core::time::Duration;

use async_std::stream::StreamExt;
use dioxus::prelude::*;
use tween::{Tween, TweenValue, Tweener};

#[derive(Clone, Copy)]
pub struct TweenManager<Value: 'static, Time: 'static, T: 'static> {
    value: Signal<Value>,
    tween: Signal<Option<Tweener<Value, Time, T>>>,
    animating: Signal<bool>,
}

const DELTA: u64 = 3;

impl<Value: TweenValue, T: Tween<Value>> TweenManager<Value, u64, T> {
    /// Starts the tween animation.
    ///
    /// # Arguments
    ///
    /// * `start` - The starting value of the tween animation.
    /// * `end` - The ending value of the tween animation.
    /// * `time` - The duration of the tween animation in milliseconds.
    /// * `tween` - The type of tween to use for the animation.
    pub fn start(&mut self, start: Value, end: Value, time: u64, tween: T) {
        let TweenManager {
            mut value,
            mut tween,
            mut animating,
        } = self;
        if *animating.read() {
            return;
        }

        animating.set(true);

        let tweener = Tweener::new(start, end, time, tween);

        *tween.write() = Some(tweener);

        spawn(async move {
            let mut ticker = async_std::stream::interval(Duration::from_millis(DELTA));
            loop {
                let Some(mut tweener) = tween.as_mut() else {
                    break;
                };
                if tweener.is_finished() {
                    break;
                }
                value.set(tweener.move_by(DELTA));
                ticker.tick().await;
            }

            *tween.write() = None;
            animating.set(false);
        });
    }

    /// Sets the current value of the tween animation.
    ///
    /// # Arguments
    ///
    /// * `val` - The new value to set.
    pub fn set_value(&mut self, val: Value) {
        self.value.set(val)
    }

    /// Checks if the tween animation is currently animating.
    ///
    /// Returns `true` if the animation is in progress, `false` otherwise.
    pub fn is_animating(&self) -> bool {
        *self.animating.read()
    }

    /// Returns the current value of the tween animation.
    pub fn value(&self) -> Value {
        *self.value.read()
    }
}

/// Creates a new `TweenManager` instance for managing tween animations.
///
/// # Arguments
///
/// * `f` - A closure that returns the initial value of the tween animation.
///
/// # Example
///
/// ```
/// use texo_ui::hooks::use_tween;
///
/// let manager = use_tween(|| 0.0);
/// ```
pub fn use_tween<Value, Time, T>(f: impl FnOnce() -> Value) -> TweenManager<Value, Time, T> {
    let value = use_signal(f);
    let animating = use_signal(|| false);
    let tween = use_signal(|| None);

    TweenManager {
        value,
        animating,
        tween,
    }
}
