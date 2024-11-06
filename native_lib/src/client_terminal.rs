use gdnative::prelude::*;
use gdnative::api::*;
use serde_json::{Result as JResult, Value};
use serde::{Deserialize,Serialize};
use tokio::sync::mpsc;
use std::{fmt,str::FromStr};
type Sender<T> = mpsc::UnboundedSender<T>;
type Receiver<T> = mpsc::UnboundedReceiver<T>;

#[derive(Deserialize,Serialize,Debug)]
enum SocketMode{
    Native,
    NativeProcess,
    GodotClient,
}
fn get_all_socket_modes() -> Vec<SocketMode>{
    let mut v = Vec::new();
    v.push(SocketMode::Native);
    v.push(SocketMode::NativeProcess);
    v.push(SocketMode::GodotClient);
    v
}
impl fmt::Display for SocketMode{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self{
            SocketMode::Native => { write!(f,"Native") }
            SocketMode::NativeProcess => { write!(f,"NativeProcess") }
            SocketMode::GodotClient => { write!(f,"GodotClient") }
        }
    }
}
#[derive(Debug,Deserialize)]
enum Command{
    SetEntitySocketMode(String,SocketMode),
    SetAllEntitySocketMode(SocketMode),
}
trait Autocomplete{
    fn auto_complete(&self) -> fn(Vec<&str>) -> Vec<String>;
}
#[derive(Deserialize,Serialize,Debug)]
enum CommandType{
    set_entity_socket_mode,
    set_all_entity_socket_mode,
}
fn get_all_command_types() -> Vec<CommandType>{
    let mut v = Vec::new();
    v.push( CommandType::set_entity_socket_mode);
    v.push( CommandType::set_all_entity_socket_mode);
    v
}
impl Autocomplete for CommandType{
    fn auto_complete(&self) -> fn(Vec<&str>) -> Vec<String> {
        match self{
            CommandType::set_entity_socket_mode => SocketModeArgs::autocomplete_args,
            CommandType::set_all_entity_socket_mode => SocketModeAllArgs::autocomplete_args,
            _ => todo!()
        }
    }
}
impl fmt::Display for CommandType{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self{
            CommandType::set_entity_socket_mode => {
                write!(f,"set_entity_socket_mode")
            }
            CommandType::set_all_entity_socket_mode => {
                write!(f,"set_all_entity_socket_mode")
            }
        }
    }
}
impl FromStr for CommandType {
    type Err = String;
    fn from_str(input:&str) -> Result<CommandType,Self::Err>{
        match input{
            "set_entity_socket_mode" => Ok(CommandType::set_entity_socket_mode),
            "set_all_entity_socket_mode" => Ok(CommandType::set_all_entity_socket_mode),
            _ => Err(format!("No result found for command type {input:?}"))
        } 
    }
}
trait FromArgs{
    fn new(args:&Value) -> Result<Self,&'static str> where Self:Sized;
    fn autocomplete_args(args:Vec<&str>) -> Vec<String>;
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
    fn autocomplete_args(args:Vec<&str>) -> Vec<String> {
        match args.len(){
            0 | 1 => {
                let mut v = Vec::new();
                v.push("id:String".to_string());
                v
            }
            2 => {
                let modes = get_all_socket_modes();
                let pattern = &args[1];
                modes
                    .into_iter()
                    .map(|mode| mode.to_string())
                    .filter(|mode| mode.contains(pattern))
                    .collect()
            }
            _ => {Vec::new()}
        }
    }
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
#[derive(Deserialize,Serialize)]
pub struct SocketModeAllArgs{
    mode:SocketMode,
}
impl FromArgs for SocketModeAllArgs{
    fn autocomplete_args(args:Vec<&str>) -> Vec<String>{Vec::new()}
    fn new(args:&Value) -> Result<Self,&'static str> where Self:Sized{
        match args{
            Value::Array(values) => {
                if values.len() < 1{
                    return Err("too few arguments for SocketModeAllArgs")
                }
                let mut fmt_args = serde_json::Map::new();
                let mode = &values[0];
                fmt_args.insert("mode".to_string(),mode.clone());
                serde_json::from_value::<SocketModeAllArgs>(Value::Object(fmt_args))
                    .map_err(|e| "Error while parsing args for SocketModeAllArgs")
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
    suggestions: Ref<Label>,
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
        builder
            .signal(&CommandType::set_all_entity_socket_mode.to_string())
            .with_param("mode",VariantType::GodotString)
            .done();
    }

    fn new(_base:&CanvasLayer) -> Self{
        let (tx,rx) = mpsc::unbounded_channel::<Command>();
        ClientTerminal{
            bg_rect: ColorRect::new().into_shared(),
            input : TextEdit::new().into_shared(),
            suggestions: Label::new().into_shared(),
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
        let suggestions = unsafe{self.suggestions.assume_safe()};
        rect.set_anchors_preset(Control::PRESET_WIDE,true);
        rect.set_frame_color(Color{r:0.0,g:0.0,b:0.0,a:0.5});
        output.set_scroll_active(true);
        owner.add_child(rect,true);
        owner.add_child(input,true);
        owner.add_child(output,true);
        owner.add_child(suggestions,true);
    }
    
    fn get_suggestions_from_input(text_input:String) -> Vec<String>{
        let split_input = text_input.split_ascii_whitespace().collect::<Vec<&str>>();
        let len = split_input.len();
        let mut v = Vec::new();
        if len <= 0{
            return v
        }
        if len >= 1 {
            get_all_command_types()
                .into_iter()
                .map(|x| x.to_string())
                .filter(|x| x.contains(text_input.as_str()))
                .map(|x| v.push(x));
        }
        match split_input.len(){
            0 => {Vec::new()},
            1 => {
                get_all_command_types()
                    .into_iter()
                    .map(|x| x.to_string())
                    .filter(|x| x.contains(text_input.as_str()))
                    .collect()
            }
            _ => {
                CommandType::from_str(split_input[0])
                    .map(|ct| ct.auto_complete()(split_input[1..].to_vec()))
                    .unwrap_or(Vec::new())
            }
            //_ => {godot_print!("{}",format!("Unhandled input length {len:?}"));Vec::new()} 
        }
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
        let suggestions = unsafe{self.suggestions.assume_safe()};
        let r_size = rect.size();
        let input_size = Vector2{x:r_size.x/2.0,y:r_size.y/2.0};
        let input_loc = Vector2{x : 0.0 , y : r_size.y/2.0};
        let output_size = Vector2{x:r_size.x/2.0,y:r_size.y/4.0};
        let output_loc = Vector2{x : 0.0 , y : 0.0};
        input.set_size(input_size,true);
        input.set_position(input_loc,true);
        output.set_size(output_size,true);
        output.set_position(output_loc,true);
        //set suggestions
        //
        let suggestions_in = Self::get_suggestions_from_input(input.text().to_string());
        let mut suggestion_text = "".to_string();
        for s in &suggestions_in{
            suggestion_text.push_str(s.as_str());
            suggestion_text.push_str("\n");
        }
        let suggestion_size = Vector2{x:input_size.x, y : suggestions_in.len() as f32 * 20.0};
        let suggestion_loc = Vector2{x:0.0,y:input_loc.y - suggestion_size.y};
        suggestions.set_size(suggestion_size,false);
        suggestions.set_position(suggestion_loc,false);
        suggestions.set_text(suggestion_text);
        


        //Command Handler loop
        match self.cmd_rx.try_recv() {
                Ok(Command::SetEntitySocketMode(id,mode)) => {
                    owner.emit_signal(
                        CommandType::set_entity_socket_mode.to_string(),
                        &[Variant::new(id), Variant::new(mode.to_string())]
                    );
                    self.output_append("signaled set entity socket mode".into());
                }
                Ok(Command::SetAllEntitySocketMode(mode)) => {
                    owner.emit_signal(
                        CommandType::set_all_entity_socket_mode.to_string(),
                        &[Variant::new(mode.to_string())]
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
        arr.push(Variant::new(CommandType::set_all_entity_socket_mode.to_string()));
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
                CommandType::set_all_entity_socket_mode => {
                    SocketModeAllArgs::new(&cmd.args)
                        .map(|smargs| Command::SetAllEntitySocketMode(smargs.mode))
                }
            })
        
    }
}
