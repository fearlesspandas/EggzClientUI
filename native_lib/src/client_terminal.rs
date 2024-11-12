use gdnative::prelude::*;
use gdnative::api::*;
use serde_json::{Result as JResult, Value};
use serde::{Deserialize,Serialize};
use tokio::sync::mpsc;
use std::{fmt,str::FromStr};
use crate::terminal_commands::{Command,CommandType,InputCommand,SocketModeArgs,SocketModeAllArgs,StartDataStreamArgs,ArgsConstructor};
use crate::terminal_actions::{ActionType,Action};
use crate::socket_mode::SocketMode;
use crate::traits::{FromArgs,GetAll,Autocomplete,CreateSignal};
use crate::data_display::{DataDisplay,DataType};
use crate::data_graphs::{BarGraph};
type Sender<T> = mpsc::UnboundedSender<T>;
type Receiver<T> = mpsc::UnboundedReceiver<T>;

#[derive(NativeClass)]
#[inherit(CanvasLayer)]
#[register_with(Self::register_signals)]
pub struct ClientTerminal{
    bg_rect: Ref<ColorRect>,
    input:Ref<TextEdit>,
    auto_complete:Vec<String>,
    suggestions: Ref<Label>,
    output:Ref<RichTextLabel>,
    cmd_tx:Sender<Command>,
    cmd_rx:Receiver<Command>,
    action_tx:Sender<Action>,
    action_rx:Receiver<Action>,
    history:Vec<String>,
    hist_idx: i64,
    data_display: Instance<DataDisplay>,
    graph_display: Instance<BarGraph>,
    data_collection_timer: Ref<Timer>,
    data_collection_types: Vec<DataType>,
}

#[methods]
impl ClientTerminal{

    fn register_signals(builder:&ClassBuilder<Self>){
        CommandType::register(&builder);
        ActionType::register(&builder);
    }

    fn new(_base:&CanvasLayer) -> Self{
        let (tx,rx) = mpsc::unbounded_channel::<Command>();
        let (atx,arx) = mpsc::unbounded_channel::<Action>();
        let data_display = DataDisplay::make().into_shared();
        let graph_display = BarGraph::make().into_shared();
        ClientTerminal{
            bg_rect: ColorRect::new().into_shared(),
            input : TextEdit::new().into_shared(),
            auto_complete: Vec::new(),
            suggestions: Label::new().into_shared(),
            output : RichTextLabel::new().into_shared(),
            cmd_tx: tx,
            cmd_rx: rx,
            action_tx:atx,
            action_rx:arx,
            history: Vec::new(),
            hist_idx: -1,
            data_display: data_display,
            graph_display: graph_display,
            data_collection_timer: Timer::new().into_shared(),
            data_collection_types: Vec::new(),
        }
    }

    #[method]
    fn _ready(&self,#[base] owner:&CanvasLayer){
        let rect  = unsafe{self.bg_rect.assume_safe()};
        let input = unsafe{self.input.assume_safe()};
        let output = unsafe{self.output.assume_safe()};
        let suggestions = unsafe{self.suggestions.assume_safe()};
        let display = unsafe{self.data_display.assume_safe()};
        let graph = unsafe{self.graph_display.assume_safe()};
        let data_collection_timer = unsafe{self.data_collection_timer.assume_safe()};
        rect.set_anchors_preset(Control::PRESET_WIDE,true);
        rect.set_frame_color(Color{r:0.0,g:0.0,b:0.0,a:0.5});
        output.set_scroll_active(true);
        owner.add_child(rect,true);
        owner.add_child(input,true);
        owner.add_child(output,true);
        owner.add_child(suggestions,true);
        owner.add_child(display,true);
        owner.add_child(graph,true);
        data_collection_timer.set_wait_time(1.0);
        owner.add_child(data_collection_timer,true);
    }
    
    #[method]
    fn _input(&mut self,#[base] owner: &CanvasLayer,event: Ref<InputEvent>){
        let input = unsafe{self.input.assume_safe()};
        let output = unsafe{self.output.assume_safe()};
        if let Ok(event) = event.try_cast::<InputEventKey>(){
            let event = unsafe{ event.assume_safe()};
            let tree = unsafe{owner.get_tree().unwrap().assume_safe()};
            if event.is_action_pressed("terminal_autocomplete_accept",true,true){
                self.action_tx.send(Action::AutoCompleteAccept);
                tree.set_input_as_handled();
            }
            if event.is_action_released("terminal_toggle",false){
                owner.set_visible(!owner.is_visible());
                self.action_tx.send(Action::SetActive(owner.is_visible()));
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
                let res = Self::get_command(&input_text.to_string());
                self.output_append(input_text.to_string().into());
                self.output_append(format!("{res:?}\n").into());
                res.map(|cmd| self.cmd_tx.send(cmd));
                self.hist_idx = -1;
                self.input_update_from_idx();
                let ov_scroh = unsafe{output.get_v_scroll().unwrap().assume_safe()};
                ov_scroh.set_value(ov_scroh.max());
                tree.set_input_as_handled();
            }
        }
    }
    
    #[method]
    fn _process(&mut self,#[base] owner:TRef<CanvasLayer>,delta:f64){
        let rect = unsafe{ self.bg_rect.assume_safe()};
        let input = unsafe{self.input.assume_safe()};
        let output = unsafe{self.output.assume_safe()};
        let suggestions = unsafe{self.suggestions.assume_safe()};
        let data_display = unsafe{self.data_display.assume_safe()};
        let graph_display = unsafe{self.graph_display.assume_safe()};
        /////derive sizes/////////////
        let r_size = rect.size();
        let input_size = Vector2{x:r_size.x/2.0,y:50.0};
        let input_loc = Vector2{x : 0.0 , y : r_size.y - input_size.y};
        let output_size = Vector2{x:r_size.x/2.0,y:r_size.y/2.0};
        let output_loc = Vector2{x : 0.0 , y : 0.0};
        let data_display_size = output_size;
        let data_display_loc = Vector2{x:r_size.x/2.0,y:0.0};
        let graph_display_size = Vector2{x:r_size.x/2.0 , y: r_size.y/2.0};
        let graph_display_loc = Vector2{x:r_size.x - graph_display_size.x, y: r_size.y - graph_display_size.y};
        ////set sizes and positions///
        input.set_size(input_size,true);
        input.set_position(input_loc,true);
        output.set_size(output_size,true);
        output.set_position(output_loc,true);
        let suggestion_size = Vector2{x:input_size.x, y : self.auto_complete.len() as f32 * 20.0};
        let suggestion_loc = Vector2{x:0.0,y:input_loc.y - suggestion_size.y};
        suggestions.set_size(suggestion_size,false);
        suggestions.set_position(suggestion_loc,false);
        data_display.map(|dat,dat_own| {
            dat_own.set_size(data_display_size,false);
            dat_own.set_position(data_display_loc,false);
        });
        graph_display.map(|graph,control| {
            control.set_size(graph_display_size,false);
            control.set_position(graph_display_loc,false);
        });
        ////Main Render Loop////////// 
        self.handle_received_commands(owner);
        self.handle_received_actions(owner); 
        self.update_auto_complete();
    }

    #[method]
    fn add_incoming_data(&self,tag:String,data:String){
        let data_display = unsafe{self.data_display.assume_safe()};
        data_display
            .map_mut(|display,control| display.add_data(tag,data))
            .map_err(|e| godot_print!("{}",format!("Could not process incoming data due to {e:?}")));
    }
    #[method]
    fn add_graph_data(&self,tag:String,data:f32){
        let graph = unsafe{self.graph_display.assume_safe()};
        graph
            .map_mut(|graph_disp,_| graph_disp.add_data(tag,data))
            .map_err(|e| godot_print!("{}",format!("Could not process incoming grpah data due to {e:?}")));

    }

    //incomplete
    #[method]
    fn get_all_signals() -> VariantArray<Unique> {
        let arr = VariantArray::new();
        arr.push(Variant::new(CommandType::set_entity_socket_mode.to_string()));
        arr.push(Variant::new(CommandType::set_all_entity_socket_mode.to_string()));
        arr.push(Variant::new(CommandType::start_data_stream.to_string()));
        arr
    }

    fn handle_received_commands(&mut self, owner:TRef<CanvasLayer>){
        match self.cmd_rx.try_recv() {
                Ok(Command::ClearData) => {
                    let bar_graph = unsafe{self.graph_display.assume_safe()};
                    bar_graph.map_mut(|obj,_| obj.queue_clear());
                }
                Ok(Command::StartDataStream(data_type)) => {
                    let data_type_str = &data_type.to_string();
                    self.data_collection_types.push(data_type);
                    self.output_append(format!("added data_type {data_type_str:?} to stream").into());
                    let data_collection_timer = unsafe{self.data_collection_timer.assume_safe()};
                    if data_collection_timer.is_stopped(){
                        data_collection_timer.connect("timeout",owner,"send_data_requests",VariantArray::new_shared(),0);
                        data_collection_timer.start(-1.0);
                        self.output_append("started data stream".into());
                    }
                }
                Ok(Command::StopDataStream(data_type)) => {
                    let data_type_str = &data_type.to_string();
                    self.data_collection_types.retain(|val| val != &data_type);
                    self.output_append("removed data_type {data_type_str:?} from stream".into());
                    let data_size = self.data_collection_types.len();
                    self.output_append("data_types size:{data_size:?}".into());
                    if data_size == 0{
                        let data_collection_timer = unsafe{self.data_collection_timer.assume_safe()};
                        data_collection_timer.disconnect("timeout",owner,"send_data_requests");
                        data_collection_timer.stop();
                        self.output_append("size 0 : stopped data stream".into());
                    }

                }
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
                    self.output_append("signaled set all entity socket mode".into());
                }
                Err(_) => {}
            }

    }

    fn handle_received_actions(&mut self, owner:TRef<CanvasLayer>){
        let input = unsafe{self.input.assume_safe()};
        match self.action_rx.try_recv(){
            Ok(Action::SetActive(value)) => {
                owner.emit_signal(ActionType::set_active.to_string(),&[Variant::new(value)]);
            }
            Ok(Action::AutoCompleteAccept) => {
                let current_input = input.text().to_string();
                let split_args = current_input.split_ascii_whitespace().collect::<Vec<&str>>();
                let last = split_args.last().unwrap();
                input.set_text(
                    current_input.clone() + 
                    &(self.auto_complete.get(0).unwrap_or(&"".to_string()).replace(&last.to_string(),""))
                );
                input.cursor_set_line(0,false,false,0);
                input.cursor_set_column(input.text().len() as i64,false);
            }
            Ok(Action::RequestData(data_type)) => {}
            Err(_) => {}
        }
    }

    #[method]
    fn send_data_requests(&self,#[base] owner:TRef<CanvasLayer>){
        for req_type in &self.data_collection_types{
            owner.emit_signal(
                ActionType::request_data.to_string(),
                &[Variant::new(req_type.to_string())]
          );
        }
    }

    //Input Handling
    fn update_auto_complete(&mut self){
        let input = unsafe{self.input.assume_safe()};
        let suggestions = unsafe{self.suggestions.assume_safe()};
        let mut suggestion_text = "".to_string();
        for s in &self.auto_complete{
            suggestion_text.push_str(s.as_str());
            suggestion_text.push_str("\n");
        }
        suggestions.set_text(&suggestion_text);
        self.auto_complete = Self::get_suggestions_from_input(input.text().to_string());
    }

    fn get_suggestions_from_input(text_input:String) -> Vec<String>{
        let split_input = text_input.split_ascii_whitespace().collect::<Vec<&str>>();
        let len = split_input.len();
        match split_input.len(){
            0 => {Vec::new()},
            1 => {
                CommandType::get_all()
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
            .and_then(|cmd| cmd.from_args(()))
    }
}
