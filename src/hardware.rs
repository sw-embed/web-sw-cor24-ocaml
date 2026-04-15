//! Floating hardware panel: S2 switch (clickable) + D2 LED indicator.
//!
//! Same shape as `../web-sw-cor24-apl/src/hardware.rs` (TX/RX byte
//! displays dropped -- not useful for OCaml's REPL UX). Uses the
//! `hw-*` CSS classes in `src/ui.css`.

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HardwarePanelProps {
    pub led_on: bool,
    pub s2_on: bool,
    pub on_s2_toggle: Callback<()>,
}

#[function_component(HardwarePanel)]
pub fn hardware_panel(props: &HardwarePanelProps) -> Html {
    let on_s2 = {
        let cb = props.on_s2_toggle.clone();
        Callback::from(move |_: MouseEvent| cb.emit(()))
    };
    // Buttons inside a panel that overlays the workspace would steal
    // focus from the source textarea on click; suppress the default
    // focus-on-mousedown so the editor's caret stays where it was.
    let prevent_focus = Callback::from(|e: MouseEvent| e.prevent_default());

    let led_class = if props.led_on {
        "hw-led hw-led-on"
    } else {
        "hw-led hw-led-off"
    };

    let switch_class = if props.s2_on {
        "hw-switch hw-switch-on"
    } else {
        "hw-switch hw-switch-off"
    };

    html! {
        <div class="hw-panel" aria-label="COR24 hardware panel">
            <div class="hw-title">{ "Hardware" }</div>
            <div class="hw-row">
                <span class="hw-label">{ "S2" }</span>
                <span class={switch_class} onclick={on_s2} onmousedown={prevent_focus}
                      role="button" tabindex="0">
                    { if props.s2_on { "ON" } else { "OFF" } }
                </span>
            </div>
            <div class="hw-row">
                <span class="hw-label">{ "D2" }</span>
                <span class={led_class}></span>
            </div>
        </div>
    }
}
