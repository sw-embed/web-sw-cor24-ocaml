use gloo::timers::callback::Timeout;
use web_sys::{Element, HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement, KeyboardEvent};
use yew::prelude::*;

pub mod config;
pub mod demos;
pub mod hardware;
pub mod runner;

use demos::{DEMOS, default_demo_index};
use hardware::HardwarePanel;
use runner::Session;

/// Per-tick instruction budget for the run loop. The `Session` runs at
/// most this many cor24 instructions per tick before yielding to the
/// browser. OCaml interpretation is heavier than BASIC, so this is
/// larger than the BASIC project's setting.
const DEFAULT_MAX_INSTRS: u64 = 500_000_000;
const TICK_DELAY_MS: u32 = 0;

fn now_ms() -> f64 {
    js_sys::Date::now()
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum HelpTab {
    UserGuide,
    LanguageReference,
}

pub enum Msg {
    SelectDemo(usize),
    SourceChanged(String),
    Run,
    Tick,
    Stop,
    Reset,
    Clear,
    IncreaseBudget,
    KeyDown(KeyboardEvent),
    InputChanged(String),
    InputSubmit,
    HistoryPrev,
    HistoryNext,
    ToggleS2,
    OpenHelp,
    CloseHelp,
    SetHelpTab(HelpTab),
}

pub struct App {
    selected: usize,
    source: String,
    output: String,
    status: String,
    error: bool,
    session: Option<Session>,
    running: bool,
    max_instrs: u64,
    started_at: f64,
    elapsed_ms: f64,
    budget_exhausted: bool,
    input_line: String,
    awaiting_input: bool,
    // Submitted REPL inputs in chronological order; `history_cursor`
    // indexes from the end (0 = most recent) while the user is
    // walking the history, and is `None` when editing a fresh line.
    input_history: Vec<String>,
    history_cursor: Option<usize>,
    output_len_at_feed: Option<usize>,
    output_ref: NodeRef,
    input_ref: NodeRef,
    s2_on: bool,
    led_on: bool,
    help_open: bool,
    help_tab: HelpTab,
}

impl App {
    fn load_demo(&mut self, idx: usize) {
        if let Some(demo) = DEMOS.get(idx) {
            self.selected = idx;
            self.source = demo.source.to_string();
            self.output.clear();
            self.status = "idle".into();
            self.error = false;
            self.session = None;
            self.running = false;
            self.budget_exhausted = false;
            self.elapsed_ms = 0.0;
            self.input_line.clear();
            self.awaiting_input = false;
            self.input_history.clear();
            self.history_cursor = None;
        }
    }

    fn start_run(&mut self, ctx: &Context<Self>) {
        let interactive = DEMOS
            .get(self.selected)
            .map(|d| d.interactive)
            .unwrap_or(false);
        self.session = Some(if interactive {
            Session::new_interactive(&self.source)
        } else {
            Session::new(&self.source)
        });
        self.input_line.clear();
        self.awaiting_input = false;
        self.input_history.clear();
        self.history_cursor = None;
        self.running = true;
        self.error = false;
        self.budget_exhausted = false;
        self.output.clear();
        self.started_at = now_ms();
        self.elapsed_ms = 0.0;
        self.status = "running...".into();
        self.schedule_tick(ctx);
    }

    fn schedule_tick(&self, ctx: &Context<Self>) {
        let link = ctx.link().clone();
        Timeout::new(TICK_DELAY_MS, move || link.send_message(Msg::Tick)).forget();
    }

    fn finish(&mut self, status: String, error: bool) {
        self.running = false;
        self.status = status;
        self.error = error;
        self.elapsed_ms = now_ms() - self.started_at;
        if let Some(s) = &self.session {
            self.output = s.clean_output();
        }
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let idx = default_demo_index();
        let demo = &DEMOS[idx];
        Self {
            selected: idx,
            source: demo.source.to_string(),
            output: String::new(),
            status: "idle".into(),
            error: false,
            session: None,
            running: false,
            max_instrs: DEFAULT_MAX_INSTRS,
            started_at: 0.0,
            elapsed_ms: 0.0,
            budget_exhausted: false,
            input_line: String::new(),
            awaiting_input: false,
            input_history: Vec::new(),
            history_cursor: None,
            output_len_at_feed: None,
            output_ref: NodeRef::default(),
            input_ref: NodeRef::default(),
            s2_on: false,
            led_on: false,
            help_open: false,
            help_tab: HelpTab::UserGuide,
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        if let Some(el) = self.output_ref.cast::<Element>() {
            el.set_scroll_top(el.scroll_height());
        }
        if self.awaiting_input
            && let Some(el) = self.input_ref.cast::<HtmlInputElement>()
        {
            let _ = el.focus();
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SelectDemo(i) => {
                self.load_demo(i);
                self.max_instrs = DEFAULT_MAX_INSTRS;
                true
            }
            Msg::SourceChanged(v) => {
                self.source = v;
                false
            }
            Msg::Run => {
                self.max_instrs = DEFAULT_MAX_INSTRS;
                self.start_run(ctx);
                true
            }
            Msg::IncreaseBudget => {
                self.max_instrs = self.max_instrs.saturating_mul(4);
                self.start_run(ctx);
                true
            }
            Msg::Stop => {
                if self.running {
                    self.finish("stopped".into(), false);
                }
                true
            }
            Msg::Reset => {
                let idx = self.selected;
                self.running = false;
                self.session = None;
                self.load_demo(idx);
                self.max_instrs = DEFAULT_MAX_INSTRS;
                self.s2_on = false;
                self.led_on = false;
                true
            }
            Msg::ToggleS2 => {
                self.s2_on = !self.s2_on;
                if let Some(s) = self.session.as_mut() {
                    s.set_switch(self.s2_on);
                }
                true
            }
            Msg::Clear => {
                self.output.clear();
                if !self.running {
                    self.status = "idle".into();
                    self.error = false;
                    self.budget_exhausted = false;
                }
                true
            }
            Msg::Tick => {
                if !self.running {
                    return false;
                }
                let Some(session) = self.session.as_mut() else {
                    self.running = false;
                    return true;
                };
                let interactive = DEMOS
                    .get(self.selected)
                    .map(|d| d.interactive)
                    .unwrap_or(false);
                if !interactive {
                    let remaining = self.max_instrs.saturating_sub(session.instructions());
                    if remaining == 0 {
                        self.budget_exhausted = true;
                        let instrs = session.instructions();
                        self.finish(format!("halted (budget) -- {} instrs", instrs), true);
                        return true;
                    }
                }
                // Push the current S2 state into the cor24 switch
                // register before each batch so any `switch ()` reads
                // the demo issues this tick see fresh data.
                session.set_switch(self.s2_on);
                let result = session.tick();
                self.led_on = session.led_on();
                if session.is_awaiting_input() {
                    // is_awaiting_input fires as soon as our rx_queue
                    // empties, which can happen BEFORE the interpreter
                    // has finished evaluating the line we fed it. If
                    // output hasn't grown since the last feed_input,
                    // the interp is still mid-evaluation — keep ticking
                    // instead of stopping and forcing the user to hit
                    // Enter a second time.
                    let still_processing = self
                        .output_len_at_feed
                        .is_some_and(|len| session.clean_output().len() <= len);
                    if still_processing {
                        self.output = session.clean_output();
                        self.elapsed_ms = now_ms() - self.started_at;
                        self.status = format!(
                            "running... {} instrs, {:.0} ms",
                            session.instructions(),
                            self.elapsed_ms
                        );
                        self.schedule_tick(ctx);
                        return true;
                    }
                    self.output_len_at_feed = None;
                    self.awaiting_input = true;
                    self.output = session.clean_output();
                    self.elapsed_ms = now_ms() - self.started_at;
                    self.status = format!(
                        "awaiting input ({} instrs, {:.0} ms)",
                        session.instructions(),
                        self.elapsed_ms
                    );
                    return true;
                }
                if result.done {
                    let instrs = session.instructions();
                    let reason = session.stop_reason();
                    let halted = session.is_halted();
                    self.finish(
                        format!(
                            "{} ({} instrs, {:.0} ms)",
                            reason,
                            instrs,
                            now_ms() - self.started_at
                        ),
                        !halted,
                    );
                } else {
                    self.output = session.clean_output();
                    self.elapsed_ms = now_ms() - self.started_at;
                    self.status = format!(
                        "running... {} instrs, {:.0} ms",
                        session.instructions(),
                        self.elapsed_ms
                    );
                    self.schedule_tick(ctx);
                }
                true
            }
            Msg::InputChanged(v) => {
                self.input_line = v;
                // User is editing a fresh line; leave any history walk.
                self.history_cursor = None;
                false
            }
            Msg::InputSubmit => {
                if !self.awaiting_input {
                    return false;
                }
                let line = std::mem::take(&mut self.input_line);
                // Record non-empty submits; skip consecutive duplicates so
                // hitting Enter twice doesn't bloat the history.
                if !line.is_empty() && self.input_history.last() != Some(&line) {
                    self.input_history.push(line.clone());
                }
                self.history_cursor = None;
                if let Some(session) = self.session.as_mut() {
                    self.output_len_at_feed = Some(session.clean_output().len());
                    session.feed_input(&line);
                }
                self.awaiting_input = false;
                self.status = "running...".into();
                self.schedule_tick(ctx);
                true
            }
            Msg::HistoryPrev => {
                if self.input_history.is_empty() {
                    return false;
                }
                let next = match self.history_cursor {
                    None => 0,
                    Some(i) if i + 1 < self.input_history.len() => i + 1,
                    Some(i) => i,
                };
                self.history_cursor = Some(next);
                let idx = self.input_history.len() - 1 - next;
                self.input_line = self.input_history[idx].clone();
                true
            }
            Msg::HistoryNext => {
                let Some(cursor) = self.history_cursor else {
                    return false;
                };
                if cursor == 0 {
                    self.history_cursor = None;
                    self.input_line.clear();
                } else {
                    let next = cursor - 1;
                    self.history_cursor = Some(next);
                    let idx = self.input_history.len() - 1 - next;
                    self.input_line = self.input_history[idx].clone();
                }
                true
            }
            Msg::KeyDown(e) => {
                if e.key() == "Enter" && (e.ctrl_key() || e.meta_key()) {
                    e.prevent_default();
                    ctx.link().send_message(Msg::Run);
                } else if e.key() == "Escape" {
                    if self.help_open {
                        e.prevent_default();
                        ctx.link().send_message(Msg::CloseHelp);
                    } else if self.running {
                        e.prevent_default();
                        ctx.link().send_message(Msg::Stop);
                    }
                }
                false
            }
            Msg::OpenHelp => {
                self.help_open = true;
                true
            }
            Msg::CloseHelp => {
                self.help_open = false;
                true
            }
            Msg::SetHelpTab(tab) => {
                self.help_tab = tab;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_demo = ctx.link().callback(|e: Event| {
            let target: HtmlSelectElement = e.target_unchecked_into();
            let idx: usize = target.value().parse().unwrap_or(0);
            Msg::SelectDemo(idx)
        });
        let on_src = ctx.link().callback(|e: InputEvent| {
            let target: HtmlTextAreaElement = e.target_unchecked_into();
            Msg::SourceChanged(target.value())
        });
        let on_run = ctx.link().callback(|_| Msg::Run);
        let on_stop = ctx.link().callback(|_| Msg::Stop);
        let on_reset = ctx.link().callback(|_| Msg::Reset);
        let on_clear = ctx.link().callback(|_| Msg::Clear);
        let on_inc = ctx.link().callback(|_| Msg::IncreaseBudget);
        let on_keydown = ctx.link().callback(Msg::KeyDown);
        let on_input_change = ctx.link().callback(|e: InputEvent| {
            let target: HtmlInputElement = e.target_unchecked_into();
            Msg::InputChanged(target.value())
        });
        let on_input_submit = ctx.link().callback(|_| Msg::InputSubmit);
        let on_input_keydown = ctx.link().callback(|e: KeyboardEvent| match e.key().as_str() {
            "Enter" => {
                e.prevent_default();
                Msg::InputSubmit
            }
            "ArrowUp" => {
                e.prevent_default();
                Msg::HistoryPrev
            }
            "ArrowDown" => {
                e.prevent_default();
                Msg::HistoryNext
            }
            _ => Msg::KeyDown(e),
        });

        let status_class = if self.error {
            "status status-error"
        } else {
            "status"
        };
        let run_button = if self.running {
            html! { <button onclick={on_stop}>{ "Stop" }</button> }
        } else {
            html! { <button onclick={on_run}>{ "Run" }</button> }
        };

        let description = DEMOS
            .get(self.selected)
            .map(|d| d.description)
            .unwrap_or("");
        let on_s2 = ctx.link().callback(|_| Msg::ToggleS2);

        html! {
            <>
            <a href="https://github.com/sw-embed/web-sw-cor24-ocaml" class="github-corner"
               aria-label="View source on GitHub" target="_blank">
                <svg width="80" height="80" viewBox="0 0 250 250" aria-hidden="true">
                    <path d="M0,0 L115,115 L130,115 L142,142 L250,250 L250,0 Z" />
                    <path d="M128.3,109.0 C113.8,99.7 119.0,89.6 119.0,89.6 C122.0,82.7 120.5,78.6 \
                        120.5,78.6 C119.2,72.0 123.4,76.3 123.4,76.3 C127.3,80.9 125.5,87.3 125.5,87.3 \
                        C122.9,97.6 130.6,101.9 134.4,103.2" fill="currentColor"
                        style="transform-origin:130px 106px;" class="octo-arm" />
                    <path d="M115.0,115.0 C114.9,115.1 118.7,116.5 119.8,115.4 L133.7,101.6 C136.9,99.2 \
                        139.9,98.4 142.2,98.6 C133.8,88.0 127.5,74.4 143.8,58.0 C148.5,53.4 154.0,51.2 \
                        159.7,51.0 C160.3,49.4 163.2,43.6 171.4,40.1 C171.4,40.1 176.1,42.5 178.8,56.2 \
                        C183.1,58.6 187.2,61.8 190.9,65.4 C194.5,69.0 197.7,73.2 200.1,77.6 C213.8,80.2 \
                        216.3,84.9 216.3,84.9 C212.7,93.1 206.9,96.0 205.4,96.6 C205.1,102.4 203.0,107.8 \
                        198.3,112.5 C181.9,128.9 168.3,122.5 157.7,114.1 C157.9,116.9 156.7,120.9 \
                        152.7,124.9 L141.0,136.5 C139.8,137.7 141.6,141.9 141.8,141.8 Z"
                        fill="currentColor" />
                </svg>
            </a>
            <main class="page" onkeydown={on_keydown.clone()}>
                <header class="chrome">
                    <h1>{ "web-sw-cor24-ocaml" }</h1>
                    <div class="controls">
                        <select onchange={on_demo} disabled={self.running}>
                            { for DEMOS.iter().enumerate().map(|(i, d)| html! {
                                <option value={i.to_string()} selected={i == self.selected}>
                                    { d.name }
                                </option>
                            })}
                        </select>
                        { run_button }
                        <button class="secondary" onclick={on_reset} disabled={self.running}>{ "Reset" }</button>
                        <button class="secondary" onclick={on_clear}>{ "Clear" }</button>
                        <button
                            class="secondary icon-btn"
                            onclick={ctx.link().callback(|_| Msg::OpenHelp)}
                            title="Open help (User Guide / Language Reference)"
                            aria-label="Open help"
                        >{ "?" }</button>
                    </div>
                </header>
                <p class="demo-description">{ description }</p>
                <div class="workspace">
                <section class="panel panel-src">
                    <label>{ "source (.ml)" }</label>
                    <textarea
                        class="src"
                        spellcheck="false"
                        value={self.source.clone()}
                        oninput={on_src}
                        onkeydown={on_keydown.clone()}
                    />
                </section>
                <section class="panel panel-out">
                    <div class={status_class}>
                        { format!("status: {}", self.status) }
                        { if self.budget_exhausted {
                            html! {
                                <>
                                    { " -- " }
                                    <button class="link-btn" onclick={on_inc}>
                                        { "Increase budget 4x" }
                                    </button>
                                </>
                            }
                        } else { html! {} }}
                    </div>
                    <pre class="out" ref={self.output_ref.clone()}>{ &self.output }</pre>
                    { if self.awaiting_input {
                        html! {
                            <>
                                <div class="input-row">
                                    <label>{ "input:" }</label>
                                    <input
                                        ref={self.input_ref.clone()}
                                        type="text"
                                        value={self.input_line.clone()}
                                        oninput={on_input_change}
                                        onkeydown={on_input_keydown}
                                        autofocus=true
                                    />
                                    <button onclick={on_input_submit}>{ "Send" }</button>
                                </div>
                                <p class="input-hint">
                                    { "tip: each input must be a complete expression. \
                                       `let` forms require an `in` clause, e.g. " }
                                    <code>{ "let x = 42 in x" }</code>
                                    { ". Press ↑/↓ to recall previous inputs." }
                                </p>
                            </>
                        }
                    } else { html! {} }}
                </section>
                </div>
            </main>
            <HardwarePanel led_on={self.led_on} s2_on={self.s2_on} on_s2_toggle={on_s2} />
            { self.view_help(ctx) }
            <footer>
                <span>{"MIT License"}</span>
                <span class="footer-sep">{"\u{00b7}"}</span>
                <span>{"\u{00a9} 2026 Michael A Wright"}</span>
                <span class="footer-sep">{"\u{00b7}"}</span>
                <a href="https://makerlisp.com" target="_blank">{"COR24-TB"}</a>
                <span class="footer-sep">{"\u{00b7}"}</span>
                <a href="https://software-wrighter-lab.github.io/" target="_blank">{"Blog"}</a>
                <span class="footer-sep">{"\u{00b7}"}</span>
                <a href="https://discord.com/invite/Ctzk5uHggZ" target="_blank">{"Discord"}</a>
                <span class="footer-sep">{"\u{00b7}"}</span>
                <a href="https://www.youtube.com/@SoftwareWrighter" target="_blank">{"YouTube"}</a>
                <span class="footer-sep">{"\u{00b7}"}</span>
                <a href="https://github.com/sw-embed/web-sw-cor24-ocaml/blob/main/docs/demos.md" target="_blank">{"Demo Documentation"}</a>
                <span class="footer-sep">{"\u{00b7}"}</span>
                <a href="https://github.com/sw-embed/web-sw-cor24-ocaml/blob/main/CHANGES.md" target="_blank">{"Changes"}</a>
                <span class="footer-sep">{"\u{00b7}"}</span>
                <span>{ format!("{} \u{00b7} {} \u{00b7} {}",
                    env!("BUILD_HOST"),
                    env!("BUILD_SHA"),
                    env!("BUILD_TIMESTAMP"),
                ) }</span>
            </footer>
            </>
        }
    }
}

impl App {
    fn view_help(&self, ctx: &Context<Self>) -> Html {
        if !self.help_open {
            return html! {};
        }
        let on_close = ctx.link().callback(|_| Msg::CloseHelp);
        let on_stop_prop = Callback::from(|e: MouseEvent| e.stop_propagation());
        let on_user = ctx
            .link()
            .callback(|_| Msg::SetHelpTab(HelpTab::UserGuide));
        let on_lang = ctx
            .link()
            .callback(|_| Msg::SetHelpTab(HelpTab::LanguageReference));
        let user_class = if self.help_tab == HelpTab::UserGuide {
            "modal-tab active"
        } else {
            "modal-tab"
        };
        let lang_class = if self.help_tab == HelpTab::LanguageReference {
            "modal-tab active"
        } else {
            "modal-tab"
        };
        let body = match self.help_tab {
            HelpTab::UserGuide => user_guide_body(),
            HelpTab::LanguageReference => language_reference_body(),
        };
        html! {
            <div class="modal-overlay" onclick={on_close.clone()}>
                <div class="modal" onclick={on_stop_prop} role="dialog" aria-modal="true">
                    <header class="modal-header">
                        <div class="modal-tabs">
                            <button class={user_class} onclick={on_user}>{ "User Guide" }</button>
                            <button class={lang_class} onclick={on_lang}>{ "Language Reference" }</button>
                        </div>
                        <button class="modal-close" onclick={on_close}
                                aria-label="Close help">{ "×" }</button>
                    </header>
                    <div class="modal-body">{ body }</div>
                </div>
            </div>
        }
    }
}

fn user_guide_body() -> Html {
    html! {
        <>
            <h3>{ "Running a demo" }</h3>
            <ul>
                <li>{ "Pick a program from the demo dropdown. Source loads into the left panel; hit " }
                    <strong>{ "Run" }</strong>
                    { " or " }<kbd>{ "Cmd" }</kbd>{ "/" }<kbd>{ "Ctrl" }</kbd>{ "+" }<kbd>{ "Enter" }</kbd>
                    { " to execute." }</li>
                <li><strong>{ "Esc" }</strong>{ " or the " }<strong>{ "Stop" }</strong>
                    { " button halts a running session." }</li>
                <li><strong>{ "Reset" }</strong>{ " reloads the demo's source. " }
                    <strong>{ "Clear" }</strong>{ " wipes the output panel but keeps the edits." }</li>
                <li>{ "Edit the source freely before hitting Run -- the dropdown source is a starting point." }</li>
            </ul>

            <h3>{ "Status bar and budget" }</h3>
            <ul>
                <li>{ "The status line shows instruction count and elapsed ms as the VM runs." }</li>
                <li>{ "If a program exhausts its instruction budget, an " }
                    <em>{ "Increase budget 4x" }</em>
                    { " button appears -- click to continue." }</li>
            </ul>

            <h3>{ "REPL demo (interactive)" }</h3>
            <ul>
                <li>{ "The " }<code>{ "repl-session" }</code>
                    { " demo pauses after the seed source finishes and waits for your input." }</li>
                <li>{ "Each input must be a single complete expression. " }
                    <code>{ "let x = 42" }</code>
                    { " is a parse error on its own; use " }
                    <code>{ "let x = 42 in x" }</code>{ "." }</li>
                <li>{ "Press " }<kbd>{ "↑" }</kbd>{ " / " }<kbd>{ "↓" }</kbd>
                    { " in the input row to recall previously submitted lines." }</li>
                <li>{ "Press " }<kbd>{ "Enter" }</kbd>
                    { " or click " }<strong>{ "Send" }</strong>{ " to submit." }</li>
            </ul>

            <h3>{ "Hardware panel" }</h3>
            <ul>
                <li>{ "Bottom-right panel: toggle the " }<strong>{ "S2" }</strong>
                    { " switch to drive " }<code>{ "switch ()" }</code>{ "; the " }
                    <strong>{ "D2" }</strong>{ " indicator reflects " }
                    <code>{ "led_on" }</code>{ " / " }<code>{ "led_off" }</code>{ "." }</li>
                <li>{ "Try the " }<code>{ "led-toggle" }</code>
                    { " demo with S2 on vs off." }</li>
            </ul>

            <h3>{ "Footer" }</h3>
            <ul>
                <li>{ "Build host, short SHA, and UTC build timestamp are shown in the footer to help diagnose stale-cache deploys." }</li>
            </ul>
        </>
    }
}

fn language_reference_body() -> Html {
    html! {
        <>
            <p>{ "This is an " }<em>{ "integer-subset OCaml" }</em>
                { " interpreter (sw-cor24-ocaml): no floats, no references, \
                     no exceptions, a small built-in standard library. The REPL \
                     reads one complete expression per line and echoes its value." }</p>

            <h3>{ "Literals" }</h3>
            <ul>
                <li>{ "Integers: " }<code>{ "0 42 -7 1000000" }</code></li>
                <li>{ "Strings: " }<code>{ "\"hello\"" }</code>{ " (concatenate with " }
                    <code>{ "^" }</code>{ ")" }</li>
                <li>{ "Unit: " }<code>{ "()" }</code></li>
                <li>{ "Pairs / tuples: " }<code>{ "(1, 2)" }</code>{ ", " }
                    <code>{ "(a, (b, c))" }</code></li>
                <li>{ "Lists: " }<code>{ "[]" }</code>{ ", " }
                    <code>{ "[1; 2; 3]" }</code>{ ", " }
                    <code>{ "1 :: nil" }</code>{ ", " }
                    <code>{ "h :: t" }</code></li>
                <li>{ "Options: " }<code>{ "None" }</code>{ ", " }
                    <code>{ "Some 42" }</code></li>
                <li>{ "Booleans: " }<code>{ "true" }</code>{ ", " }
                    <code>{ "false" }</code></li>
            </ul>

            <h3>{ "Operators" }</h3>
            <ul>
                <li>{ "Arithmetic: " }<code>{ "+ - * / mod" }</code>
                    { " (unary minus too)" }</li>
                <li>{ "String: " }<code>{ "^" }</code>{ " (concatenation)" }</li>
                <li>{ "Comparison: " }<code>{ "= < > <= >= <>" }</code></li>
                <li>{ "Logical: " }<code>{ "&& ||" }</code>
                    { " (short-circuit); " }<code>{ "not" }</code></li>
                <li>{ "Cons: " }<code>{ "::" }</code>{ " (right-associative)" }</li>
                <li>{ "Sequencing: " }<code>{ "e1 ; e2" }</code>
                    { " (must fit on one REPL line)" }</li>
            </ul>

            <h3>{ "Bindings and functions" }</h3>
            <ul>
                <li><code>{ "let x = expr in body" }</code>
                    { " -- bare top-level " }<code>{ "let x = 42" }</code>
                    { " is " }<em>{ "not" }</em>{ " accepted; use " }
                    <code>{ "let x = 42 in x" }</code>{ "." }</li>
                <li><code>{ "let rec f = fun n -> ..." }</code>
                    { " / " }<code>{ "let rec f n = ..." }</code>
                    { " for recursion." }</li>
                <li>{ "Curried sugar: " }<code>{ "let add x y = x + y in add 3 4" }</code></li>
                <li>{ "Lambdas: " }<code>{ "fun x -> x + 1" }</code>{ ", " }
                    <code>{ "fun x y -> x + y" }</code></li>
                <li><code>{ "function p1 -> e1 | p2 -> e2" }</code>
                    { " -- shorthand for " }
                    <code>{ "fun x -> match x with ..." }</code></li>
                <li>{ "Destructuring args: " }<code>{ "let swap (x, y) = (y, x)" }</code></li>
            </ul>

            <h3>{ "Control flow" }</h3>
            <ul>
                <li><code>{ "if cond then e1 else e2" }</code>
                    { " (else required)" }</li>
                <li><code>{ "match e with p1 -> e1 | p2 -> e2" }</code></li>
                <li>{ "Guards: " }<code>{ "| n when n < 0 -> ..." }</code></li>
                <li>{ "Patterns: literals, " }<code>{ "_" }</code>
                    { " wildcard, tuples " }<code>{ "(a, b)" }</code>
                    { ", lists " }<code>{ "[]" }</code>{ " / " }<code>{ "h :: t" }</code>
                    { " / " }<code>{ "[a; b]" }</code>{ ", " }
                    <code>{ "None" }</code>{ " / " }<code>{ "Some x" }</code>
                    { ", ADT constructors." }</li>
            </ul>

            <h3>{ "Types" }</h3>
            <ul>
                <li><code>{ "type color = Red | Green | Blue" }</code>
                    { " (nullary constructors only)" }</li>
                <li>{ "Built-ins: " }<code>{ "option" }</code>
                    { " (" }<code>{ "None" }</code>{ " | " }<code>{ "Some x" }</code>{ ")" }</li>
            </ul>

            <h3>{ "Module-qualified functions" }</h3>
            <ul>
                <li><code>{ "List.length" }</code>{ ", " }
                    <code>{ "List.rev" }</code>{ ", " }
                    <code>{ "List.hd" }</code>{ ", " }
                    <code>{ "List.tl" }</code>{ ", " }
                    <code>{ "List.is_empty" }</code></li>
                <li><code>{ "List.map" }</code>{ ", " }
                    <code>{ "List.filter" }</code>{ ", " }
                    <code>{ "List.fold_left" }</code>{ ", " }
                    <code>{ "List.iter" }</code></li>
                <li><code>{ "String.length" }</code></li>
            </ul>

            <h3>{ "I/O and hardware" }</h3>
            <ul>
                <li><code>{ "print_int : int -> unit" }</code>
                    { " -- writes decimal via UART (no newline)" }</li>
                <li><code>{ "print_endline : string -> unit" }</code>
                    { " -- writes string + newline" }</li>
                <li><code>{ "putc : int -> unit" }</code>
                    { " -- writes one byte to UART" }</li>
                <li><code>{ "led_on ()" }</code>{ ", " }
                    <code>{ "led_off ()" }</code>{ ", " }
                    <code>{ "set_led : bool -> unit" }</code></li>
                <li><code>{ "switch : unit -> bool" }</code>
                    { " -- reads the S2 panel switch" }</li>
            </ul>

            <h3>{ "Comments" }</h3>
            <ul>
                <li><code>{ "(* this is a comment *)" }</code>
                    { " -- block comments, and they " }<em>{ "nest" }</em>{ "." }</li>
                <li>{ "No single-line comment form. " }<code>{ "#" }</code>
                    { " is not a token at all; real OCaml's toplevel directives \
                       (" }<code>{ "#use" }</code>{ ", " }<code>{ "#quit" }</code>
                    { ") are not implemented and will produce PARSE ERROR." }</li>
            </ul>

            <h3>{ "REPL quirks" }</h3>
            <ul>
                <li>{ "Each line is parsed independently -- a trailing " }
                    <code>{ ";" }</code>
                    { " on a continuation line is a parse error." }</li>
                <li>{ "Every line's value is echoed. Use unit-returning \
                     expressions (" }<code>{ "print_int" }</code>
                    { ", " }<code>{ "print_endline" }</code>
                    { ") if you want raw output and no echo formatting." }</li>
            </ul>
        </>
    }
}
