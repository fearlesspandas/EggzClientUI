use gdnative::prelude::*;

pub mod terminal_commands;
pub mod socket_mode;
pub mod traits;
pub mod terminal_actions;
pub mod data_display;
pub mod data_graphs;
pub mod client_entity;
pub mod entity_flight_tail;
pub mod data_snapshots;
pub mod shop_menu;
mod client_terminal;


fn init(handle:InitHandle){
    handle.add_class::<client_terminal::ClientTerminal>();
    handle.add_class::<data_display::DataDisplay>();
    handle.add_class::<data_graphs::BarGraph>();
    handle.add_class::<data_graphs::BarGraphColumn>();
    handle.add_class::<data_graphs::AggregateStats>();
    handle.add_class::<data_graphs::HoverStats>();
    handle.add_class::<entity_flight_tail::BeltOrbital>();
    handle.add_class::<entity_flight_tail::OrbitNode>();
    handle.add_class::<data_snapshots::DataSnapshots>();
    handle.add_class::<shop_menu::ShopItem>();
    handle.add_class::<shop_menu::ShopMenu>();
}
godot_init!(init);
