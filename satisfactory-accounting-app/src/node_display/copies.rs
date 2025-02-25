// Copyright 2022 Zachary Stewart
//
//   Licensed under the Apache License, Version 2.0 (the "License");
//   you may not use this file except in compliance with the License.
//   You may obtain a copy of the License at
//
//       http://www.apache.org/licenses/LICENSE-2.0
use log::warn;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::node_display::get_value_from_input_event;

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    /// Last set value for the number of virtual copies.
    pub copies: u32,
    /// Callback to change the actual value.
    pub update_copies: Callback<u32>,
}

pub enum Msg {
    /// Message during editing to update the edited text.
    UpdateInput { input: String },
    /// Message while not editing to start editing.
    StartEdit { input: u32 },
    /// Message to finish editing.
    FinishEdit,
    /// Cancel editing without changing the value.
    Cancel,
}

/// Display and editing for number of coipes.
#[derive(Default)]
pub struct VirtualCopies {
    /// Pending edit text if number of copies is being changed.
    edit_text: Option<String>,
    /// Whether we did focus since last committing an edit.
    did_focus: bool,
    /// Input to focus on editing.
    input: NodeRef,
}

impl Component for VirtualCopies {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Default::default()
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateInput { input } => {
                self.edit_text = Some(input);
                true
            }
            Msg::StartEdit { input } => {
                self.edit_text = Some(input.to_string());
                self.did_focus = false;
                true
            }
            Msg::FinishEdit => {
                if let Some(edit_text) = self.edit_text.take() {
                    if let Ok(value) = edit_text.parse::<u32>() {
                        ctx.props().update_copies.emit(value);
                    }
                    true
                } else {
                    warn!("FinishEdit while not editing");
                    false
                }
            }
            Msg::Cancel => {
                self.edit_text = None;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        if let Some(edit_text) = &self.edit_text {
            let oninput = link.callback(|input| Msg::UpdateInput {
                input: get_value_from_input_event(input),
            });
            let onkeyup = link.batch_callback(|e: KeyboardEvent| match &*e.key() {
                "Esc" | "Escape" => Some(Msg::Cancel),
                _ => None,
            });
            let onblur = link.callback(|_| Msg::FinishEdit);
            let onsubmit = link.callback(|e: FocusEvent| {
                e.prevent_default();
                Msg::FinishEdit
            });
            html! {
                <form class="VirtualCopies" title="Multiplier" {onsubmit}>
                    <input class="current-virt-copies" type="text" value={edit_text.clone()}
                        {oninput} {onblur} {onkeyup} ref={self.input.clone()} />
                    <span>{"×"}</span>
                </form>
            }
        } else {
            let value = ctx.props().copies;
            let onclick = link.callback(move |_| Msg::StartEdit { input: value });
            html! {
                <div class="VirtualCopies" title="Multiplier" {onclick}>
                    <span class="current-virt-copies">{value.to_string()}</span>
                    <span>{"×"}</span>
                </div>
            }
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        if !self.did_focus {
            if let Some(input) = self.input.cast::<HtmlInputElement>() {
                if let Err(e) = input.focus() {
                    warn!("Failed to focus input: {:?}", e);
                }
                input.select();
                self.did_focus = true;
            }
        }
    }
}
