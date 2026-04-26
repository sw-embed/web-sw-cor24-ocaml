//! Module-file editor component for multi-file demos (Phase 2 of
//! `docs/multiple-file-demos-plan.md`).
//!
//! Each auxiliary `.ml` file shipped alongside a demo's main source
//! gets its own collapsible cell. Phase 2 supports per-file editing
//! and collapse only -- no add/remove/upload (those land in Phase 3).
//! Modeled on `web-sw-cor24-plsw`'s `MacroEditor`, trimmed.

use wasm_bindgen::JsCast;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

/// One auxiliary file the user can edit. Constructed in App state
/// from a demo's `auxiliary_files` on demo selection / reset.
#[derive(Clone, PartialEq)]
pub struct ModuleFile {
    pub name: String,
    pub source: String,
    pub collapsed: bool,
}

#[derive(Properties, PartialEq)]
pub struct ModuleEditorProps {
    pub files: Vec<ModuleFile>,
    pub on_change: Callback<(usize, String)>,
    pub on_toggle_collapse: Callback<usize>,
    #[prop_or_default]
    pub disabled: bool,
}

#[function_component(ModuleEditor)]
pub fn module_editor(props: &ModuleEditorProps) -> Html {
    if props.files.is_empty() {
        return html! {};
    }
    html! {
        <section class="panel aux-files-panel">
            <label>{ "module files (.ml)" }</label>
            <div class="aux-files">
                { for props.files.iter().enumerate().map(|(idx, file)| {
                    render_aux_file(idx, file, &props.on_change, &props.on_toggle_collapse, props.disabled)
                })}
            </div>
        </section>
    }
}

fn render_aux_file(
    idx: usize,
    file: &ModuleFile,
    on_change: &Callback<(usize, String)>,
    on_toggle_collapse: &Callback<usize>,
    disabled: bool,
) -> Html {
    let collapse_icon = if file.collapsed { "▶" } else { "▼" };

    let oninput = {
        let on_change = on_change.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(target) = e.target()
                && let Some(ta) = target.dyn_ref::<HtmlTextAreaElement>()
            {
                on_change.emit((idx, ta.value()));
            }
        })
    };

    let on_toggle = {
        let on_toggle_collapse = on_toggle_collapse.clone();
        Callback::from(move |_: MouseEvent| on_toggle_collapse.emit(idx))
    };

    html! {
        <div class="aux-file">
            <div class="aux-file-header">
                <button class="aux-collapse-btn" onclick={on_toggle}
                    aria-label="Toggle collapse">
                    { collapse_icon }
                </button>
                <span class="aux-file-name">{ &file.name }</span>
            </div>
            if !file.collapsed {
                <textarea
                    class="src aux-file-textarea"
                    spellcheck="false"
                    autocomplete="off"
                    {disabled}
                    value={file.source.clone()}
                    {oninput}
                />
            }
        </div>
    }
}
