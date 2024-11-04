use gdnative::prelude::*;
use gdnative::api::*;
use serde_json::{Result as JResult, Value};
use serde::{Deserialize,Serialize};
use tokio::sync::mpsc;
use std::fmt;

type Sender<T> = mpsc::UnboundedSender<T>;
type Receiver<T> = mpsc::UnboundedReceiver<T>;

#[derive(Deserialize,Serialize,Debug)]
enum SocketMode{
    Native,
    GodotClient,
}
impl fmt::Display for SocketMode{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self{
            SocketMode::Native => { write!(f,"Native") }
            SocketMode::GodotClient => { write!(f,"GodotClient") }
        }
    }
}
#[derive(Debug,Deserialize)]
enum Command{
    SetEntitySocketMode(String,SocketMode),
}
#[derive(Deserialize,Serialize,Debug)]
enum CommandType{
    set_entity_socket_mode,
}
impl fmt::Display for CommandType{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self{
            CommandType::set_entity_socket_mode => {
                write!(f,"set_entity_socket_mode")
            }
        }
    }
}
trait FromArgs{
    fn new(args:&Value) -> Result<Self,&'static str> where Self:Sized;
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
impl FromArgs for SocketModeArgs{
    fn new(args:&Value) -> Result<Self,&'static str> where Self:Sized{
        match args{
            Value::Array(values) => {
                if values.len() < 2{
                    return Err("too few arguments for SocketModeArgs")
                }
                let mut fmt_args = serde_json::Map::new();
                let id = &values[0];
                let mode = &values[1];
                fmt_args.insert("id".to_string(),id.clone());
                fmt_args.insert("mode".to_string(),mode.clone());
                serde_json::from_value::<SocketModeArgs>(Value::Object(fmt_args))
                    .map_err(|e| "Error while parsing args for SocketModeArgs")
            }
            _ => {Err("unexpected value type for socket mode args; expected Value::Array")}
        }
    }
}
#[derive(NativeClass)]
#[inherit(CanvasLayer)]
#[register_with(Self::register_signals)]
pub struct ClientTerminal{
    bg_rect: Ref<ColorRect>,
    input:Ref<TextEdit>,
    output:Ref<RichTextLabel>,
    cmd_tx:Sender<Command>,
    cmd_rx:Receiver<Command>,
    history:Vec<String>,
    hist_idx: i64,
}

#[methods]
impl ClientTerminal{
    fn register_signals(builder:&ClassBuilder<Self>){
        builder
            .signal(&CommandType::set_entity_socket_mode.to_string())
            .with_param("id"  ,VariantType::GodotString)
            .with_param("mode",VariantType::GodotString)
            .done();
    }

    fn new(_base:&CanvasLayer) -> Self{
        let (tx,rx) = mpsc::unbounded_channel::<Command>();
        ClientTerminal{
            bg_rect: ColorRect::new().into_shared(),
            input : TextEdit::new().into_shared(),
            output : RichTextLabel::new().into_shared(),
            cmd_tx: tx,
            cmd_rx: rx,
            history: Vec::new(),
            hist_idx: -1,
        }
    }

    #[method]
    fn _ready(&self,#[base] owner:&CanvasLayer){
        let rect  = unsafe{self.bg_rect.assume_safe()};
        let input = unsafe{self.input.assume_safe()};
        let output = unsafe{self.output.assume_safe()};
        rect.set_anchors_preset(Control::PRESET_WIDE,true);
        rect.set_frame_color(Color{r:0.0,g:0.0,b:0.0,a:0.5});
        output.set_scroll_active(true);
        owner.add_child(rect,true);
        owner.add_child(input,true);
        owner.add_child(output,true);
    }
    
    fn input_update_from_idx(&self){
        let input = unsafe{self.input.assume_safe()};
        if self.hist_idx < 0{
            input.set_text("");
            input.cursor_set_line(0,false,false,0);
            input.cursor_set_column(0,true);
            return;
        }
        let history = &self.history;
        history
            .get((self.history.len() as i64 - 1 - self.hist_idx) as usize)
            .map_or_else(
                || input.set_text(""),
                |cmd| input.set_text(cmd)
            );
        input.cursor_set_line(0,false,false,0);
        input.cursor_set_column(input.text().len() as i64,false);

    }
    #[method]
    fn _input(&mut self,#[base] owner: &CanvasLayer,event: Ref<InputEvent>){
        let input = unsafe{self.input.assume_safe()};
        let output = unsafe{self.output.assume_safe()};
        if let Ok(event) = event.try_cast::<InputEventKey>(){
            let event = unsafe{ event.assume_safe()};
            let tree = unsafe{owner.get_tree().unwrap().assume_safe()};
            if event.is_action_released("terminal_toggle",false){
                owner.set_visible(!owner.is_visible());
                if owner.is_visible(){input.grab_focus();}
                tree.set_input_as_handled();
            }
            if event.is_action_pressed("terminal_hist_inc",false,true){
                self.hist_idx = std::cmp::min(self.hist_idx + 1, (self.history.len() - 1) as i64);
                self.input_update_from_idx();
                tree.set_input_as_handled();
            }
            if event.is_action_pressed("terminal_hist_dec",false,true){
                self.hist_idx = std::cmp::max(self.hist_idx -1,-1);
                self.input_update_from_idx();
                tree.set_input_as_handled();
            }
            if event.is_action_released("terminal_accept",true){
                let input_text = input.text().to_string().replace("\n","");
                match self.history.last() {
                    Some(cmd) => {
                            if !(cmd.to_string() == input_text){
                                self.history.push(input_text.clone());
                            }
                    }
                    None => {
                        self.history.push(input_text.clone());
                    }
                }
                    
                //self.history.push(input_text.clone());
                let res = Self::get_command(&input_text.to_string());
                self.output_append(input_text.to_string().into());
                self.output_append(format!("{res:?}\n").into());
                res.map(|cmd| self.cmd_tx.send(cmd));
                self.hist_idx = -1;
                self.input_update_from_idx();
                let ov_scroh = unsafe{output.get_v_scroll().unwrap().assume_safe()};
                //ov_scroh.set_as_ratio(1.0);
                ov_scroh.set_value(ov_scroh.max());
                tree.set_input_as_handled();
            }
        }
    }

    #[method]
    fn _process(&mut self,#[base] owner:&CanvasLayer,delta:f64){
        let rect = unsafe{ self.bg_rect.assume_safe()};
        let input = unsafe{self.input.assume_safe()};
        let output = unsafe{self.output.assume_safe()};
        let r_size = rect.size();
        let input_size = Vector2{x:r_size.x/2.0,y:r_size.y/2.0};
        let input_loc = Vector2{x : 0.0 , y : r_size.y/2.0};
        let label_size = Vector2{x:r_size.x/2.0,y:r_size.y/2.0};
        let label_loc = Vector2{x : 0.0 , y : 0.0};
        input.set_size(input_size,true);
        input.set_position(input_loc,true);
        output.set_size(label_size,true);
        output.set_position(label_loc,true);

        //Command Handler loop
        match self.cmd_rx.try_recv() {
                Ok(Command::SetEntitySocketMode(id,mode)) => {
                    owner.emit_signal(
                        CommandType::set_entity_socket_mode.to_string(),
                        &[Variant::new(id), Variant::new(mode.to_string())]
                    );
                    self.output_append("signaled set entity socket mode".into());
                }
                _ => {}
            }
    }

    #[method]
    fn get_all_signals() -> VariantArray<Unique> {
        let arr = VariantArray::new();
        arr.push(Variant::new(CommandType::set_entity_socket_mode.to_string()));
        arr
    }

    fn output_append(&self,text:GodotString){
        let output = unsafe{self.output.assume_safe()};
        output.set_text(output.text().clone() + text.clone() + "\n".into());
    }

    fn get_command(mut raw: &str) -> Result<Command,&'static str>{
        let Some((typ,args)) = raw.split_once(" ") else {todo!()};
        let typ_value = Value::String(typ.to_string());
        let args_value = Value::Array(
                args
                .split_ascii_whitespace()
                .collect::<Vec<&str>>()
                .iter()
                .map(|v| Value::String(v.to_string()))
                .collect()
            );
        let mut cmd_map = serde_json::Map::new();
        cmd_map.insert("typ".to_string(),typ_value);
        cmd_map.insert("args".to_string(), args_value);
        let cmd_obj = Value::Object(cmd_map);
        serde_json::from_value::<InputCommand>(cmd_obj)
            .map_err(|e| "Error while parsing InputCommand")
            .and_then(|cmd| match cmd.typ{
                CommandType::set_entity_socket_mode => {
                    SocketModeArgs::new(&cmd.args)
                        .map(|smargs| Command::SetEntitySocketMode(smargs.id,smargs.mode))
                }
            })
        
    }
}
