// use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
// use wasm_bindgen_futures::spawn_local;
use yew::{prelude::*, html::Scope};

// use log::info;
// use wasm_bindgen::JsValue;

use minesweeper_ui::components::board::BoardComponent;

use minesweeper_ui::minesweeper::{cell::Cell, Game};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

pub enum Msg {
    Discover{ cell: Cell },
    Flag{ cell: Cell },
    Reset
}

pub struct App {
    link: Scope<Self>,
    game: Game,
    height: usize,
    width: usize,
    num_mines: usize
}

impl Component for App {
    type Message = Msg;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let height = 30;
        let width = 30;
        let num_mines =(height * width / 10) as usize;

        let mut game = Game::new(height, width, 5);
        game.start_board();

        Self { 
            link: ctx.link().clone(),
            game,
            height,
            width,
            num_mines
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let b = self.game.get_board().clone();
        html!{
            <main class="container">
                // Disable context menu
                <script>
                    {"document.addEventListener('contextmenu', event => event.preventDefault());"}
                </script>
                
                <div class="upper-menu">
                    <button class="button-reset" 
                        onclick={self.link.callback(|_| Msg::Reset)}>{"Reset"}</button>
                </div>

                <div class="game">
                </div>
                <BoardComponent 
                    onsignal={self.link.callback(|cell| Msg::Discover{cell})} 
                    flagsignal={self.link.callback(|cell| Msg::Flag{cell})} 
                    board={b}/>
            </main>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Discover {cell} => {
                self.game.show(cell.get_pos());
            },
            Msg::Flag {cell} => {
                self.game.set_flag(cell.get_pos(), !self.game.get_cell(cell.get_pos()).is_flagged());
            }
            Msg::Reset => {
                self.game = Game::new(self.height, self.width, self.num_mines);
                self.game.start_board();
            }
        }
        true
    }
}