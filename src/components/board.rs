use yew::{prelude::*, html::Scope};

use crate::minesweeper::{board::Board, cell::Cell};

use crate::components::button::Button;

// use log::info;
// use wasm_bindgen::JsValue;

pub struct BoardComponent {
    link: Scope<Self>,
    board: Board,
    onsignal: Callback<Cell>,
    flagsignal: Callback<Cell>
}

pub enum Msg {
    Discover{ cell: Cell },
    Flag{ cell: Cell }
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub board: Board,
    pub onsignal: Callback<Cell>,
    pub flagsignal: Callback<Cell>
}

impl Component for BoardComponent {
    type Message = Msg;

    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            link: ctx.link().clone(),
            board: ctx.props().board.clone(),
            onsignal: ctx.props().onsignal.clone(),
            flagsignal: ctx.props().flagsignal.clone()
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let b = ctx.props().board.get_board().clone();

        let height = self.board.get_height();
        let width = self.board.get_width();

        html! {
            <div class="board">
                {b.into_iter().map( |row| {
                    html! {
                        <>
                        {row.into_iter().map(|c| {
                            html! {
                                <Button 
                                    onsignal={self.link.callback(move |_| Msg::Discover{cell: c})} 
                                    flagsignal={self.link.callback(move |_| Msg::Flag{cell: c})} 
                                    cell={c}
                                    height={height}
                                    width={width}/>
                            } 
                        }).collect::<Html>()}
                        <br/>
                        </>
                    }
                }).collect::<Html>()}
            </div>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Discover {cell} => {
                self.onsignal.emit(cell);
            }
            Msg::Flag {cell} => {
                self.flagsignal.emit(cell);
            }
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        self.board = ctx.props().board.clone();
        self.onsignal = ctx.props().onsignal.clone();
        self.flagsignal = ctx.props().flagsignal.clone();
        true
    }

}