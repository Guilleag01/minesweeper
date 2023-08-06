use serde::{Deserialize, Serialize};
// use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
// use web_sys::EventTarget;
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
    Discover{cell: Cell}
}

pub struct App {
    link: Scope<Self>,
    game: Game
}


// #[function_component(App)]
// pub fn app() -> Html {
//     html! {

//     }
// }

impl Component for App {
    type Message = Msg;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let mut game = Game::new(25, 40, 50);
        game.start_board();

        info!("\n{}", game.get_board().to_string());

        Self { 
            link: ctx.link().clone(),
            game
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let b = self.game.get_board().clone();

        html!{
            <main class="container">
                <BoardComponent onsignal={self.link.callback(|cell| Msg::Discover{cell})} board={b}/>
            </main>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Discover {cell} => {
                // info!("Pos (from App): {}", format!("{:?}", cell.get_pos()));
                self.game.show(cell.get_pos());

                // info!("celhid  (from App): {}", format!("{:?}", self.game.get_cell(cell.get_pos()).is_hidden()));
                info!("\n{}", self.game.get_board().to_string())
            }
        }
        true
    }
}