use gdnative::prelude::*;

mod client_terminal;

fn init(handle:InitHandle){
    handle.add_class::<client_terminal::ClientTerminal>();
}
godot_init!(init);
