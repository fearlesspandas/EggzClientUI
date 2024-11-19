use gdnative::prelude::*;

pub mod terminal_commands;
pub mod socket_mode;
pub mod traits;
pub mod terminal_actions;
pub mod data_display;
pub mod data_graphs;
pub mod client_entity;
mod client_terminal;


fn init(handle:InitHandle){
    handle.add_class::<client_terminal::ClientTerminal>();
    handle.add_class::<data_display::DataDisplay>();
    handle.add_class::<data_graphs::BarGraph>();
    handle.add_class::<data_graphs::BarGraphColumn>();
    handle.add_class::<data_graphs::HoverStats>();
}
godot_init!(init);
