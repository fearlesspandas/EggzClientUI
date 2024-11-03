use gdnative::prelude::*;
use gdnative::api::*;
use serde_json::{Result as JResult, Value};
use serde::{Deserialize,Serialize};
use std::sync::mpsc;

type Sender<T> = mpsc::Sender<T>;
type Receiver<T> = mpsc::Receiver<T>;

#[derive(Deserialize,Serialize,Debug)]
enum SocketMode{
    Native,
    GodotClient,
}
#[derive(Debug,Deserialize)]
enum Command{
    SetEntitySocketMode(String,SocketMode),
}
#[derive(Deserialize,Serialize)]
enum CommandType{
    set_entity_socket_mode,
}
#[derive(Deserialize,Serialize)]
pub struct InputCommand{
    typ:CommandType,
    args: Value
}
#[derive(Deserialize,Serialize)]
pub struct SocketModeArgs{
    id:String,
    mode:SocketMode,
}
#[derive(NativeClass)]
#[inherit(CanvasLayer)]
pub struct ClientTerminal{
    bg_rect: Ref<ColorRect>,
    input:Ref<TextEdit>,
    label:Ref<Label>,
    cmd_tx:Sender<Command>,
    cmd_rx:Receiver<Command>,
}

#[methods]
impl ClientTerminal{
    fn new(_base:&CanvasLayer) -> Self{
        let (tx,rx) = mpsc::channel();
        ClientTerminal{
            bg_rect: ColorRect::new().into_shared(),
            input : TextEdit::new().into_shared(),
            label : Label::new().into_shared(),
            cmd_tx: tx,
            cmd_rx: rx,
        }
    }

    #[method]
    fn _ready(&self,#[base] owner:&CanvasLayer){
        let rect  = unsafe{self.bg_rect.assume_safe()};
        let input = unsafe{self.input.assume_safe()};
        let label = unsafe{self.label.assume_safe()};
        rect.set_anchors_preset(Control::PRESET_WIDE,true);
        rect.set_frame_color(Color{r:0.0,g:0.0,b:0.0,a:0.5});
        owner.add_child(rect,true);
        owner.add_child(input,true);
        owner.add_child(label,true);
    }

    fn get_command(mut raw: String) -> Result<Command,&'static str>{
        raw.push_str("}");
        let mut r:String = "{".into();
        r.push_str(raw.as_str());
        let raw = r;
        let Some((typ,args)) = raw.split_once(" ") else {todo!()};
        match serde_json::from_str::<CommandType>(typ){
            Ok(CommandType::set_entity_socket_mode) => {
                serde_json::from_value::<SocketModeArgs>(args.into())
                    .map(|args| Command::SetEntitySocketMode(args.id,args.mode) )
                    .map(|cmd| todo!())
                    .map_err(|e| "could not resolve args for set_entity_socket_mode")
            }
            _ => {Err("Whoops")}
        }
    }

    fn label_append_text(&self,text:GodotString){
        let label = unsafe{self.label.assume_safe()};
        label.set_text(label.text().clone() + text.clone());
    }
    #[method]
    fn _input(&self,#[base] owner: &CanvasLayer,event: Ref<InputEvent>){
        let input = unsafe{self.input.assume_safe()};
        let label = unsafe{self.label.assume_safe()};
        if let Ok(event) = event.try_cast::<InputEventKey>(){
            let event = unsafe{ event.assume_safe()};
            if event.is_action_released("terminal_toggle",false){
                owner.set_visible(!owner.is_visible());
                if owner.is_visible(){input.grab_focus();}
            }
            if event.is_action_released("terminal_accept",true){
                let t_label = label.text();
                let input_text = input.text();
                let res = Self::get_command(input_text.to_string());
                self.label_append_text(input_text);
                self.label_append_text(format!("{res:?}").into());
                res.map(|cmd| self.cmd_tx.send(cmd));
                input.set_text("");
            }
        }
    }

    #[method]
    fn _process(&self,#[base] owner:&CanvasLayer,delta:f64){
        let rect = unsafe{ self.bg_rect.assume_safe()};
        let input = unsafe{self.input.assume_safe()};
        let label = unsafe{self.label.assume_safe()};
        let r_size = rect.size();
        let input_size = Vector2{x:r_size.x/2.0,y:r_size.y/2.0};
        let input_loc = Vector2{x : 0.0 , y : r_size.y/2.0};
        let label_size = Vector2{x:r_size.x/2.0,y:r_size.y/2.0};
        let label_loc = Vector2{x : 0.0 , y : 0.0};
        input.set_size(input_size,true);
        input.set_position(input_loc,true);
        label.set_size(label_size,true);
        label.set_position(label_loc,true);

        //self.cmd_rx.recv()
        //    .map(|cmd| match cmd{
        //        Command::SetEntitySocketMode(id,mode) => {self.label_append_text("set entity socket mode".into())}
        //    });
    }
}
