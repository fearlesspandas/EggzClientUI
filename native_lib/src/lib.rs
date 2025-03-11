use gdnative::prelude::*;

pub mod terminal_commands;
pub mod socket_mode;
pub mod traits;
pub mod ui_traits;
pub mod terminal_actions;
pub mod data_display;
pub mod data_graphs;
pub mod client_entity;
pub mod entity_flight_tail;
pub mod data_snapshots;
pub mod shop_menu;
pub mod slizzard;
pub mod field;
pub mod field_server;
pub mod field_abilities;
pub mod field_ability_mesh;
pub mod field_ability_colliders;
pub mod field_ability_actions;
pub mod collision_layer;
pub mod item_menu;
pub mod button_tiles;
pub mod damage_indicator;
pub mod server_console;
mod client_terminal;

#[allow(non_snake_case)]
#[allow(non_camel_case_types)]

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
    handle.add_class::<shop_menu::MenuButton>();
    handle.add_class::<shop_menu::ShopMenu>();
    handle.add_class::<slizzard::BodyPiece>();
    handle.add_class::<slizzard::Slizzard>();
    handle.add_class::<field::Field>();
    handle.add_class::<field::FieldZone>();
    handle.add_class::<field::FieldOp>();
    handle.add_class::<field::FieldOps>();
    handle.add_class::<field::FieldOp3D>();
    handle.add_class::<field::FieldOps3D>();
    handle.add_class::<field_ability_mesh::FieldAbilityMesh>();
    handle.add_class::<field_server::FieldServer>();
    handle.add_class::<field_server::FieldZoneServer>();
    handle.add_class::<item_menu::InventoryMenu>();
    handle.add_class::<item_menu::Pocket>();
    handle.add_class::<item_menu::InventorySlot>();
    handle.add_class::<item_menu::InventoryOperations>();
    handle.add_class::<item_menu::OperationButton>();
    handle.add_class::<item_menu::SlotAmount>();
    handle.add_class::<button_tiles::Tile>();
    handle.add_class::<damage_indicator::DamageIndicator>();
    handle.add_class::<damage_indicator::DamageDisplay>();
    handle.add_class::<server_console::ServerConsole>();
    handle.add_class::<server_console::PlayerLocation>();
    handle.add_class::<server_console::PlayerChart>();
    //handle.add_class::<item_menu::ControlBox<item_menu::ItemSlotCommand>>();
}
godot_init!(init);
