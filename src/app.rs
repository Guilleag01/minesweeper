use serde::{Deserialize, Serialize};
// use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use web_sys::AddEventListenerOptions;
// use wasm_bindgen_futures::spawn_local;
use yew::{prelude::*, html::Scope};

use log::info;
use wasm_bindgen::JsValue;

use minesweeper_ui::components::{button::Button, board::BoardComponent};

use minesweeper_ui::minesweeper::{cell::Cell, board::Board, Game};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

pub enum Msg {
    Discover{ cell: Cell },
    Flag{ cell: Cell }
}

pub struct App {
    link: Scope<Self>,
    game: Game
}

impl Component for App {
    type Message = Msg;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let height = 100;
        let width = 41;
        let num_mines =(height * width / 10) as usize;

        let mut game = Game::new(height, width, num_mines);
        game.start_board();

        Self { 
            link: ctx.link().clone(),
            game
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let b = self.game.get_board().clone();
        html!{
            <main class="container">
                // Disable context menu
                // <button class="restart-button">Restart</button>
                <script>{"document.addEventListener('contextmenu', event => event.preventDefault());"}</script>
                <BoardComponent onsignal={self.link.callback(|cell| Msg::Discover{cell})} flagsignal={self.link.callback(|cell| Msg::Flag{cell})} board={b}/>
            </main>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Discover {cell} => {
                self.game.show(cell.get_pos());
            },
            Msg::Flag {cell} => {
                self.game.set_flag(cell.get_pos(), !self.game.get_cell(cell.get_pos()).is_flagged());
            }
        }
        true
    }
}