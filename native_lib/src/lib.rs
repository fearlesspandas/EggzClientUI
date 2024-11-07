use gdnative::prelude::*;

pub mod terminal_commands;
pub mod socket_mode;
pub mod traits;
mod client_terminal;

fn init(handle:InitHandle){
    handle.add_class::<client_terminal::ClientTerminal>();
}
godot_init!(init);
