use yew::{prelude::*, html::Scope};
use crate::minesweeper::cell::Cell;

use log::info;
use wasm_bindgen::JsValue;

pub struct Button {
    link: Scope<Self>,
    cell: Cell,
    onsignal: Callback<Cell>,
}

pub enum Msg {
    Clicked,
    RightClicked
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub cell: Cell,
    pub onsignal: Callback<Cell>,
}

impl Component for Button {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            link: ctx.link().clone(),
            cell: ctx.props().cell.clone(),
            onsignal: ctx.props().onsignal.clone()
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut style = String::new();
        if !self.cell.is_hidden() {
            style = format!("background-color: #2f2f2f; color: #ffffff; transition: background-color 0.5s, color 0.5s; transition-delay: {}s, {}s;", self.cell.get_delay(), self.cell.get_delay());
        }
        html!{

            <button 
                class={if self.cell.is_hidden() {"button-hidden"} else {"button-shown"}}
                onclick={self.link.callback(|_| Msg::Clicked)}
                // oncontextmenu={self.link.callback(|_| Msg::Clicked)}
                oncontextmenu={self.link.callback(|e: MouseEvent| {drop(e); Msg::RightClicked})}
                // style={(if !self.cell.is_hidden() {format!("background-color: #2f2f2f; transition: background-color 0.5s; transition-delay: {}s;", self.cell.get_delay())} else {"".to_string()}).as_str()}
                style={style}>
                { &self.cell.to_string() }
            </button>

        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Clicked => {
                // info!("hid (from cell): {}", format!("{:?}", self.cell.is_hidden()));
                self.onsignal.emit(self.cell);
            }
            Msg::RightClicked => {
                self.onsignal.emit(self.cell);
            },
        }
        false
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        self.cell = ctx.props().cell.clone();
        self.onsignal = ctx.props().onsignal.clone();
        true
    }
}