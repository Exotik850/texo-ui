use std::{future::Future, time::Duration};

use dioxus::prelude::*;

#[derive(Clone, Copy)]
pub struct TimeoutManager<T: 'static> {
    run: Signal<EventHandler<Signal<T>>>,
    value: Signal<T>,
}

impl<T: Clone> TimeoutManager<T> {
    pub fn value(&self) -> T {
        self.value.read().clone()
    }
}

impl<T> TimeoutManager<T> {
    pub fn start(&self, duration: u64) {
        let TimeoutManager { run, value } = *self;
        spawn(async move {
            async_std::task::sleep(Duration::from_millis(duration)).await;
            run.read().call(value);
        });
    }
}

pub fn timeout<F>(mut f: F, delay: u64)
where
    F: FnMut() -> () + 'static,
{
    spawn(async move {
        async_std::task::sleep(Duration::from_millis(delay)).await;
        f();
    });
}

// redacted for now
async fn timeout_async<F>(mut f: F, delay: u64)
where
    F: Future<Output = ()> + 'static,
{
    spawn(async move {
        async_std::task::sleep(Duration::from_millis(delay)).await;
        f.await;
    });
}

pub fn use_timeout<F, Ft, T>(f: F, init_val: Ft) -> TimeoutManager<T>
where
    F: FnMut(Signal<T>) -> () + 'static,
    Ft: FnOnce() -> T,
{
    let value = use_signal(init_val);
    let run = use_signal(|| EventHandler::new(f));

    TimeoutManager { run, value }
}
