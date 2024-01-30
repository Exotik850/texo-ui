use core::time::Duration;

use async_std::stream::StreamExt;
use dioxus::prelude::*;
use tween::{Tween, TweenValue, Tweener};

pub struct TweenManager<Value: 'static, Time: 'static, T: 'static> {
    value: Signal<Value>,
    tween: Signal<Option<Tweener<Value, Time, T>>>,
    animating: Signal<bool>,
}

const DELTA: u64 = 3;

impl<Value: TweenValue, T: Tween<Value>> TweenManager<Value, u64, T> {
    pub fn start(&mut self, start: Value, end: Value, time: u64, tween: T) {
        if *self.animating.read() {
            return;
        }

        self.animating.set(false);

        let tweener = Tweener::new(start, end, time, tween);

        *self.tween.write() = Some(tweener);

        let tween = self.tween;
        let mut value = self.value;
        let mut animating = self.animating;

        spawn(async move {
            let mut ticker = async_std::stream::interval(Duration::from_millis(DELTA));
            loop {
                if let Some(mut tweener) = tween.as_mut() {
                    if tweener.is_finished() {
                        break;
                    }
                    value.set(tweener.move_by(DELTA));
                } else {
                    break;
                };
                ticker.next().await;
            }

            *tween.write() = None;
            animating.set(false);
        });
    }

    pub fn set_value(&mut self, val: Value) {
        self.value.set(val)
    }

    pub fn is_animating(&self) -> bool {
        *self.animating.read()
    }

    pub fn value(&self) -> Value {
        *self.value.read()
    }
}

pub fn use_tween<Value, Time, T>(init_val: Value) -> TweenManager<Value, Time, T> {
    let value = use_signal(|| init_val);
    let animating = use_signal(|| false);
    let tween = use_signal(|| None);

    TweenManager {
        value,
        animating,
        tween,
    }
}
