// use std::fmt::format;

use yew::{prelude::*, html::Scope};
use crate::minesweeper::cell::Cell;

// use log::info;
// use wasm_bindgen::JsValue;

pub struct Button {
    link: Scope<Self>,
    cell: Cell,
    onsignal: Callback<Cell>,
    flagsignal: Callback<Cell>,
    width: usize,
    height: usize,
}

pub enum Msg {
    Clicked,
    RightClicked
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub cell: Cell,
    pub width: usize,
    pub height: usize,
    pub onsignal: Callback<Cell>,
    pub flagsignal: Callback<Cell>
}

impl Component for Button {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            link: ctx.link().clone(),
            cell: ctx.props().cell.to_owned(),
            onsignal: ctx.props().onsignal.clone(),
            flagsignal: ctx.props().flagsignal.clone(),
            width: ctx.props().width.to_owned(),
            height: ctx.props().height.to_owned()

        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {

        let style_width = format!("width: calc(max(min({}vw - ({}vw), {}vh - ({}vh) - ({}px)), 25px)); ", (100 / self.width) as f32, (40 / self.width) as f32, (100 / self.height) as f32, (20 / self.height) as f32, (110 / self.height) as f32);
        let style_font = format!("font-size: calc(0.5 * max(min({}vw - ({}vw), {}vh - ({}vh) - ({}px)), 20px)); ", (100 / self.width) as f32, (40 / self.width) as f32, (100 / self.height) as f32, (20 / self.height) as f32, (110 / self.height) as f32);
        // 100% = 20hv + (2 * n + 1) + n * x
        let mut style = if !self.cell.is_hidden() {format!("background-color: #2f2f2f; color: #ffffff; transition: background-color 0.5s, color 0.5s; transition-delay: {}s, {}s; ", self.cell.get_delay(), self.cell.get_delay())} else {"".to_string()};
        style.push_str(&style_width);
        style.push_str(&style_font);

        html!{
            <button 
                class={if self.cell.is_hidden() {"button-hidden"} else {"button-shown"}}
                onclick={self.link.callback(|_| Msg::Clicked)}
                oncontextmenu={self.link.callback(|_| {Msg::RightClicked})}
                style={style}>
                    if self.cell.is_flagged() {
                        <svg xmlns="http://www.w3.org/2000/svg" class="button-icon icon icon-tabler icon-tabler-pennant-filled" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
                            <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
                            <path d="M10 2a1 1 0 0 1 .993 .883l.007 .117v.35l8.406 3.736c.752 .335 .79 1.365 .113 1.77l-.113 .058l-8.406 3.735v7.351h1a1 1 0 0 1 .117 1.993l-.117 .007h-4a1 1 0 0 1 -.117 -1.993l.117 -.007h1v-17a1 1 0 0 1 1 -1z" stroke-width="0" fill="#ffffff"></path>
                        </svg>
                    } else {
                        if self.cell.is_mine() && !self.cell.is_hidden(){
                            <svg xmlns="http://www.w3.org/2000/svg" class="button-icon icon icon-tabler icon-tabler-bomb-filled" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
                                <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
                                <path d="M14.499 3.996a2.2 2.2 0 0 1 1.556 .645l3.302 3.301a2.2 2.2 0 0 1 0 3.113l-.567 .567l.043 .192a8.5 8.5 0 0 1 -3.732 8.83l-.23 .144a8.5 8.5 0 1 1 -2.687 -15.623l.192 .042l.567 -.566a2.2 2.2 0 0 1 1.362 -.636zm-4.499 5.004a4 4 0 0 0 -4 4a1 1 0 0 0 2 0a2 2 0 0 1 2 -2a1 1 0 0 0 0 -2z" stroke-width="0" fill="#ffffff"></path>
                                <path d="M21 2a1 1 0 0 1 .117 1.993l-.117 .007h-1c0 .83 -.302 1.629 -.846 2.25l-.154 .163l-1.293 1.293a1 1 0 0 1 -1.497 -1.32l.083 -.094l1.293 -1.292c.232 -.232 .375 -.537 .407 -.86l.007 -.14a2 2 0 0 1 1.85 -1.995l.15 -.005h1z" stroke-width="0" fill="#ffffff"></path>
                            </svg>
                        } else {
                            {&self.cell.to_string()}
                        }
                    }
            </button>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Clicked => {
                self.onsignal.emit(self.cell);
            }
            Msg::RightClicked => {
                self.flagsignal.emit(self.cell);
            },
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        self.cell = ctx.props().cell.to_owned();
        self.onsignal = ctx.props().onsignal.clone();
        self.flagsignal = ctx.props().flagsignal.clone();
        self.width = ctx.props().width.to_owned();
        self.height = ctx.props().height.to_owned();
        true
    }
}