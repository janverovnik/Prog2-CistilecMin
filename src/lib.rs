// mod display;
// mod generator_polja;
// mod strukture;
// mod gui;

use sauron::{node, wasm_bindgen, Application, Cmd, Node, Program};

struct App;

impl Application for App {
    type MSG = ();

    fn view(&self) -> Node<()> {
        node! {
            <p>
                "hello"
            </p>
        }
    }

    fn update(&mut self, _msg: ()) -> Cmd<()> {
        Cmd::none()
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    Program::mount_to_body(App);
}