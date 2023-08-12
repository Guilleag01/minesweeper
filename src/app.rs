// use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use web_sys::HtmlInputElement;

// use wasm_bindgen_futures::spawn_local;
use yew::{html::Scope, prelude::*};

// use log::info;
use wasm_bindgen::JsValue;

use regex::Regex;

use minesweeper_ui::components::board::BoardComponent;

use minesweeper_ui::minesweeper::{cell::Cell, Game};

// use timer::Timer;

// use chrono::Duration;

// use std::marker::Sync;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

pub enum Msg {
    Discover { cell: Cell },
    Flag { cell: Cell },
    Reset,
    ToggleSettings,
    UpdateHeight,
    UpdateWidth,
    UpdateMines,
    // AddTime { time: usize },
}

pub struct App {
    link: Scope<Self>,
    game: Game,
    height: usize,
    width: usize,
    num_mines: usize,
    show_settings: bool,
    // timer: Timer,
}

impl Component for App {
    type Message = Msg;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let game = Game::new(10, 10, 5);
        // game.start_board();

        // let t = Timer::new();
        // t.schedule_repeating(Duration::new(1, 0), || {
        //     Callback(||)
        // });

        Self {
            link: ctx.link().clone(),
            game,
            height: 10,
            width: 10,
            num_mines: 5,
            show_settings: false,
            // timer: Timer::new(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let b = self.game.get_board().clone();
        let style = format!(
            "height: {}px; transition: height 1s;",
            if !self.show_settings { 0 } else { 98 }
        )
        .to_string();

        html! {
            <main class="container">
                // Disable context menu
                <script>
                    {"document.addEventListener('contextmenu', event => event.preventDefault());"}
                </script>

                <div class="upper-menu">
                    <div class="menu-buttons">
                        <button class="button-reset"
                            onclick={self.link.callback(|_| Msg::Reset)}>
                            {"Reset"}
                        </button>
                        <div class="time">
                            {self.game.get_time()}
                        </div>
                        <button
                            id="open-settings"
                            class="open-settings"
                            onclick={self.link.callback(|_| Msg::ToggleSettings)}>
                            <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-settings-filled" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="#ffffff" fill="none" stroke-linecap="round" stroke-linejoin="round">
                                <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
                                <path d="M14.647 4.081a.724 .724 0 0 0 1.08 .448c2.439 -1.485 5.23 1.305 3.745 3.744a.724 .724 0 0 0 .447 1.08c2.775 .673 2.775 4.62 0 5.294a.724 .724 0 0 0 -.448 1.08c1.485 2.439 -1.305 5.23 -3.744 3.745a.724 .724 0 0 0 -1.08 .447c-.673 2.775 -4.62 2.775 -5.294 0a.724 .724 0 0 0 -1.08 -.448c-2.439 1.485 -5.23 -1.305 -3.745 -3.744a.724 .724 0 0 0 -.447 -1.08c-2.775 -.673 -2.775 -4.62 0 -5.294a.724 .724 0 0 0 .448 -1.08c-1.485 -2.439 1.305 -5.23 3.744 -3.745a.722 .722 0 0 0 1.08 -.447c.673 -2.775 4.62 -2.775 5.294 0zm-2.647 4.919a3 3 0 1 0 0 6a3 3 0 0 0 0 -6z" stroke-width="0" fill="#ffffff"></path>
                            </svg>
                        </button>
                    </div>

                    <div style={style} class="settings">
                        <div class="custom-settings">
                            <div class="setting">
                                {"Height  "}
                                <input class="text-input" id="height-input" type="text"
                                oninput={self.link.callback(|_| Msg::UpdateHeight)}/>
                            </div>
                            <div class="setting">
                                {"Width  "}
                                <input class="text-input" id="width-input" type="text"
                                oninput={self.link.callback(|_| Msg::UpdateWidth)}/>
                            </div>
                            <div class="setting">
                                {"Mines  "}
                                <input class="text-input" id="mines-input" type="text"
                                oninput={self.link.callback(|_| Msg::UpdateMines)}/>
                            </div>
                        </div>
                        <div class="preset-settings">
                            <button class="preset-setting">{"Easy (WIP)"}</button>
                            <button class="preset-setting">{"Normal (WIP)"}</button>
                            <button class="preset-setting">{"Hard (WIP)"}</button>
                        </div>
                    </div>
                </div>

                <div class="game">
                    <BoardComponent
                        onsignal={self.link.callback(|cell| Msg::Discover{cell})}
                        flagsignal={self.link.callback(|cell| Msg::Flag{cell})}
                        board={b}/>
                </div>

                <div class="gamestate">
                    if self.game.get_gamestate() == 1 {
                        <h1>{"You have lost"}</h1>
                    } else if self.game.get_gamestate() == 2 {
                        <h1>{"You win!"}</h1>
                    }
                </div>
            </main>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        let d = web_sys::window().unwrap().document().unwrap();

        let get_mines = || {
            let text = d
                .get_element_by_id("mines-input")
                .unwrap()
                .dyn_into::<HtmlInputElement>()
                .unwrap()
                .value();

            let re = Regex::new(
                "^((0+)|(1|[2-9]|[1-9][0-9]|[1-9][0-9]{2}|[1-9][0-9]{3}|[1-9][0-9]{4}|[1-8][0-9]{5}|9[0-8][0-9]{4}|99[0-7][0-9]{3}|998[0--1][0-9]{2}|9980[0--1][0-9]|99800[0-0]|998001))$",
            )
            .unwrap();
            let mut value: usize = self.num_mines;
            if re.is_match(&text) {
                value = text.parse().unwrap();
            }
            value
        };

        match msg {
            Msg::Discover { cell } => {
                if self.game.get_gamestate() == 0 {
                    if !self.game.get_fist_interaction() {
                        self.game.start_board(cell.get_pos());
                        self.game.set_fist_interaction(true);
                    }
                    self.game.show(cell.get_pos());
                    self.game.update_state();
                }
            }
            Msg::Flag { cell } => {
                self.game.set_flag(
                    cell.get_pos(),
                    !self.game.get_cell(cell.get_pos()).is_flagged(),
                );
            }
            Msg::Reset => {
                self.game = Game::new(self.height, self.width, self.num_mines);
                self.game.set_time(0);
            }
            Msg::ToggleSettings => {
                self.show_settings = !self.show_settings;
            }
            Msg::UpdateHeight => {
                let text = d
                    .get_element_by_id("height-input")
                    .unwrap()
                    .dyn_into::<HtmlInputElement>()
                    .unwrap()
                    .value();

                let re =
                    Regex::new("^(0*)(1|[2-9]|[1-9][0-9]|[1-8][0-9]{2}|9[0-8][0-9]|99[0-8]|999)$")
                        .unwrap();
                if re.is_match(&text) {
                    self.height = text.parse().unwrap();
                    let mines = get_mines();
                    self.num_mines = mines.min(self.height * self.width);
                    self.game = Game::new(self.height, self.width, self.num_mines);
                    self.game.set_time(0);
                }
            }
            Msg::UpdateWidth => {
                let text = d
                    .get_element_by_id("width-input")
                    .unwrap()
                    .dyn_into::<HtmlInputElement>()
                    .unwrap()
                    .value();

                let re =
                    Regex::new("^(0*)(1|[2-9]|[1-9][0-9]|[1-8][0-9]{2}|9[0-8][0-9]|99[0-8]|999)$")
                        .unwrap();
                if re.is_match(&text) {
                    self.width = text.parse().unwrap();
                    let mines = get_mines();
                    self.num_mines = mines.min(self.height * self.width);
                    self.game = Game::new(self.height, self.width, self.num_mines);
                    self.game.set_time(0);
                }
            }
            Msg::UpdateMines => {
                let mines = get_mines();
                if mines <= self.height * self.width {
                    self.num_mines = mines;
                    self.game = Game::new(self.height, self.width, self.num_mines);
                    self.game.set_time(0);
                }
            } // Msg::AddTime { time } => {
              //     self.game.add_time(time);
              // }
        }
        true
    }
}
