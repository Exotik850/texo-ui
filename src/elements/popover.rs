use dioxus::{
    html::geometry::euclid::{Rect, Size2D},
    prelude::*,
};

use crate::TexoTrigger;


#[derive(Clone, Copy, PartialEq)]
pub enum PopoverPosition {
    TopStart,
    TopCenter,
    TopEnd,
    LeftStart,
    LeftCenter,
    LeftEnd,
    RightStart,
    RightCenter,
    RightEnd,
    BottomStart,
    BottomCenter,
    BottomEnd,
}

// Yes it is ugly, but it is a direct port from the original JS code
// I will refactor it later
// Original: https://github.com/vaheqelyan/svelte-popover/blob/ea1171755c4ffdc96dba190b0166316a999cff03/src/Content.svelte#L49
fn calculate_style(
    Rect {
        origin: target_origin,
        size: target_size,
    }: Rect<f64, f64>,
    Rect {
        origin: content_origin,
        size: content_size,
    }: Rect<f64, f64>,
    Size2D {
        width: window_width,
        height: window_height,
        ..
    }: Size2D<f64, f64>,
    postion: Option<PopoverPosition>,
) -> String {
    let ccl = content_origin.x - content_size.width;
    let cl = ccl.min(0.0);

    let ccr = content_origin.x + content_size.width + target_size.width;
    let cr = if ccr > window_width {
        window_width - ccr
    } else {
        0.0
    };

    let cct = content_origin.y - content_size.height;
    let ct = cct.min(0.0);

    let ccb = content_size.height + target_origin.y + target_size.height;
    let cb = if ccb > window_height {
        window_height - ccb
    } else {
        0.0
    };

    let caxcl = content_origin.x - target_size.width / 2.0 - content_size.width / 2.0;
    let caxcr = content_origin.x + target_size.width / 2.0 + content_size.width * 1.5;

    let coxcl = if caxcl < 0.0 { caxcl } else { 0.0 };
    let coxcr = if caxcr > window_width {
        window_width - caxcr
    } else {
        0.0
    };

    let cayct = content_origin.y + target_size.height / 2.0 - content_size.height * 0.5;
    let coyct = cayct.min(0.0);

    let caycb = content_origin.y + target_size.height / 2.0 - content_size.height * 1.5;
    let coycb = if caycb > window_height { caycb } else { 0.0 };

    let cats = content_origin.x + content_size.height;
    let cots = if cats > window_width {
        window_width - cats
    } else {
        0.0
    };

    let cate = content_origin.x - (content_size.width - target_size.width);
    let cote = cate.min(0.0);

    let calet = content_origin.y - (content_size.height - target_size.height);
    let colet = calet.min(0.0);

    let coret = colet;
    let calsb = content_origin.y + content_size.height;
    let colsb = if calsb > window_height {
        window_height - calsb
    } else {
        0.0
    };

    let corsb = colsb;

    let cobsr = cots;
    let cobel = cote;

    let xcs = target_size.height / 2.0 - content_size.height / 2.0;
    let rle = -(content_size.height - target_size.height);
    let tbe = -(content_size.width - target_size.width);
    let tbc = target_size.width / 2.0 - content_size.width / 2.0;

    let lls = -content_size.width;
    let tts = -content_size.height;
    let rls = target_size.width;
    let bts = target_size.height;

    let position = postion.unwrap_or_else(|| {
        let cover_vals = [
            cots + ct,
            ct + coxcl + coxcr,
            ct + cote,
            cl + colsb,
            cl + coyct + coycb,
            cl + colet,
            cr + corsb,
            cr + coyct + coycb,
            cr + coret,
            cb + cobsr,
            cb + coxcl + coxcr,
            cb + cobel,
        ];
        let max = cover_vals.iter().fold(0.0, |acc: f64, &x| acc.max(x));
        let index = cover_vals.iter().position(|&r| r == max).unwrap();
        use PopoverPosition as PP;
        match index {
            0 => PP::TopStart,
            1 => PP::TopCenter,
            2 => PP::TopEnd,
            3 => PP::LeftStart,
            4 => PP::LeftCenter,
            5 => PP::LeftEnd,
            6 => PP::RightStart,
            7 => PP::RightCenter,
            8 => PP::RightEnd,
            9 => PP::BottomStart,
            10 => PP::BottomCenter,
            11 => PP::BottomEnd,
            _ => unreachable!(),
        }
    });

    match position {
        PopoverPosition::TopStart => format!("top:{tts}px"),
        PopoverPosition::TopCenter => format!("top:{tts}px;left:{tbc}px"),
        PopoverPosition::TopEnd => format!("top:{tts}px;left:{tbe}px"),
        PopoverPosition::LeftStart => format!("left:{lls}px"),
        PopoverPosition::LeftCenter => format!("left:{lls}px;top:{xcs}px"),
        PopoverPosition::LeftEnd => format!("left:{lls}px;top:{rle}px"),
        PopoverPosition::RightStart => format!("right:{rls}px"),
        PopoverPosition::RightCenter => format!("right:{rls}px;top:{xcs}px"),
        PopoverPosition::RightEnd => format!("right:{rls}px;top:{rle}px"),
        PopoverPosition::BottomStart => format!("bottom:{bts}px"),
        PopoverPosition::BottomCenter => format!("bottom:{bts}px;left:{tbc}px"),
        PopoverPosition::BottomEnd => format!("bottom:{bts}px;left:{tbe}px"),
    }
}

#[component]
pub fn Popover(
    children: Element,
    content: Element,
    #[props(default = 1000)] z_index: u16,
    open: Signal<bool>,
    #[props(default)] trigger: TexoTrigger,
    #[props(default)] position: Option<PopoverPosition>,
) -> Element {
    let mut target_el: Signal<Option<std::rc::Rc<MountedData>>> = use_signal(|| None);
    let mut content_el: Signal<Option<std::rc::Rc<MountedData>>> = use_signal(|| None);
    let mut body_el: Signal<Option<std::rc::Rc<MountedData>>> = use_signal(|| None);

    let mut target_el_rect = use_signal(|| None);
    let mut content_el_rect = use_signal(|| None);
    let mut body_el_rect = use_signal(|| None);

    use_future(move || async move {
            let t = target_el.read();
            if let Some(t) = t.as_ref() {
                let rect = t.get_client_rect().await.unwrap();
                target_el_rect.set(Some(rect));
            }
            let t = content_el.read();
            if let Some(t) = t.as_ref() {
                let rect = t.get_client_rect().await.unwrap();
                content_el_rect.set(Some(rect));
            }
            let t = body_el.read();
            if let Some(t) = t.as_ref() {
                let rect = t.get_client_rect().await.unwrap();
                body_el_rect.set(Some(rect));
            }
        });

    let style = if target_el_rect.read().is_none() || content_el_rect.read().is_none() || body_el_rect.read().is_none() {
        String::new()
    } else {
      calculate_style(target_el_rect.read().unwrap(), content_el_rect.read().unwrap(), body_el_rect.read().unwrap().size, position)
    };
    // let style = "";

    rsx!(
        div {
            position: "relative",
            onmounted: move |el| body_el.set(Some(el.data())),
            div {
                onmounted: move |el| target_el.set(Some(el.data())),
                style: if open() { "z-index:{z_index}" },
                onmouseenter: move |_| {
                    if trigger == TexoTrigger::Hover {
                        open.set(true)
                    }
                },
                onmouseleave: move |_| {
                    if trigger == TexoTrigger::Hover {
                        open.set(true)
                    }
                },
                onclick: move |e| {
                    if trigger == TexoTrigger::Click {
                        e.stop_propagation();
                        open.set(true)
                    }
                },
                ontouchend: move |e| {
                    if trigger == TexoTrigger::Click {
                        e.stop_propagation();
                        open.set(true)
                    }
                },

                {children}
            }

            if open() {

                div {
                    onmounted: move |e| content_el.set(Some(e.data())),
                    style: "z-index {z_index + 10}; {style}",
                    {content}
                }
            }
        }
    )
}

