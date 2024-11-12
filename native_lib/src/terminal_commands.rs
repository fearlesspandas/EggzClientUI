use crate::traits::{CreateSignal,GetAll,Autocomplete,FromArgs};
use crate::socket_mode::SocketMode;
use crate::client_terminal::ClientTerminal;
use crate::data_display::DataType;
use tokio::sync::mpsc;
use std::{fmt,str::FromStr};
use serde_json::{Result as JResult, Value};
use serde::{Deserialize,Serialize};
use gdnative::prelude::*;
use gdnative::api::*;

///////////////////////////////
///COMMANDS////////////////////
#[derive(Debug,Deserialize)]
pub enum Command{
    SetEntitySocketMode(String,SocketMode),
    SetAllEntitySocketMode(SocketMode),
    StartDataStream(DataType),
    StopDataStream(DataType),
    ClearData,
}
#[derive(Deserialize,Serialize,Debug)]
pub enum CommandType{
    set_entity_socket_mode,
    set_all_entity_socket_mode,
    start_data_stream,
    stop_data_stream,
    clear_data,
}
impl CreateSignal<ClientTerminal> for CommandType{
    fn register(builder:&ClassBuilder<ClientTerminal>){
        builder
            .signal(&CommandType::set_entity_socket_mode.to_string())
            .with_param("id"  ,VariantType::GodotString)
            .with_param("mode",VariantType::GodotString)
            .done();
        builder
            .signal(&CommandType::set_all_entity_socket_mode.to_string())
            .with_param("mode",VariantType::GodotString)
            .done();
        builder
            .signal(&CommandType::start_data_stream.to_string())
            .with_param("type",VariantType::GodotString)
            .done();
        builder
            .signal(&CommandType::stop_data_stream.to_string())
            .with_param("type",VariantType::GodotString)
            .done();
    }
}
impl GetAll for CommandType{
    fn get_all() -> Vec<Self> where Self:Sized{
        let mut v = Vec::new();
        v.push( CommandType::set_entity_socket_mode);
        v.push( CommandType::set_all_entity_socket_mode);
        v.push( CommandType::start_data_stream);
        v.push( CommandType::stop_data_stream);
        v.push( CommandType::clear_data);
        v
    }
}
impl Autocomplete for CommandType{
    fn auto_complete(&self) -> fn(Vec<&str>) -> Vec<String> {
        match self{
            CommandType::set_entity_socket_mode => SocketModeArgs::autocomplete_args,
            CommandType::set_all_entity_socket_mode => SocketModeAllArgs::autocomplete_args,
            CommandType::start_data_stream => StartDataStreamArgs::autocomplete_args,
            CommandType::stop_data_stream => StartDataStreamArgs::autocomplete_args,
            CommandType::clear_data => ClearDataArgs::autocomplete_args,
        }
    }
}
pub trait ArgsConstructor<T,U,E>{
    fn from_args(&self,args:U) -> Result<T,E>;
}
impl ArgsConstructor<Command,&Value,&'static str> for CommandType{
    fn from_args(&self,args:&Value) -> Result<Command,&'static str>{
        match self{
            CommandType::set_entity_socket_mode => 
                SocketModeArgs::new(args)
                .map(|parsed| Command::SetEntitySocketMode(parsed.id,parsed.mode)),
            CommandType::set_all_entity_socket_mode => 
                SocketModeAllArgs::new(args)
                .map(|parsed| Command::SetAllEntitySocketMode(parsed.mode)),
            CommandType::start_data_stream => 
                StartDataStreamArgs::new(args)
                .map(|parsed| Command::StartDataStream(parsed.data_type)),
            CommandType::stop_data_stream => 
                StartDataStreamArgs::new(args)
                .map(|parsed| Command::StopDataStream(parsed.data_type)),
            CommandType::clear_data => 
                ClearDataArgs::new(args)
                .map(|parsed| Command::ClearData),
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
            CommandType::start_data_stream => {
                write!(f,"start_data_stream")
            }
            CommandType::stop_data_stream => {
                write!(f,"stop_data_stream")
            }
            CommandType::clear_data => {
                write!(f,"clear_data")
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
            "start_data_stream" => Ok(CommandType::start_data_stream),
            "stop_data_stream" => Ok(CommandType::stop_data_stream),
            "clear_data" => Ok(CommandType::clear_data),
            _ => Err(format!("No result found for command type {input:?}"))
        } 
    }
}
///////////////////////////////
///INPUT COMMANDS//////////////
#[derive(Deserialize,Serialize)]
pub struct InputCommand{
    pub typ:CommandType,
    pub args: Value
}
impl ArgsConstructor<Command,(),&'static str> for InputCommand{
    fn from_args(&self,args:()) -> Result<Command,&'static str>{
        self.typ.from_args(&self.args)
    }
}
#[derive(Deserialize,Serialize)]
pub struct SocketModeArgs{
    pub id:String,
    pub mode:SocketMode,
}
impl FromArgs<Value> for SocketModeArgs{
    fn autocomplete_args(args:Vec<&str>) -> Vec<String> {
        match args.len(){
            0 | 1 => {
                let mut v = Vec::new();
                v.push("id:String".to_string());
                v
            }
            2 => {
                let modes = SocketMode::get_all();
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
    pub mode:SocketMode,
}
impl FromArgs<Value> for SocketModeAllArgs{
    fn autocomplete_args(args:Vec<&str>) -> Vec<String>{
        match args.len(){
            0 => {
                SocketMode::get_all()
                    .into_iter()
                    .map(|mode| mode.to_string())
                    .collect()
            }
            1 => {
                let pattern = &args[0];
                SocketMode::get_all()
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

#[derive(Deserialize,Serialize)]
pub struct StartDataStreamArgs{
    pub data_type:DataType
}
impl FromArgs<Value> for StartDataStreamArgs{
    fn autocomplete_args(args:Vec<&str>) -> Vec<String>{
        match args.len(){
            0 => {
                DataType::get_all()
                    .into_iter()
                    .map(|data_type| data_type.to_string())
                    .collect()
            }
            1 => {
                let pattern = &args[0];
                DataType::get_all()
                    .into_iter()
                    .map(|data_type| data_type.to_string())
                    .filter(|data_type| data_type.contains(pattern))
                    .collect()
            }
            _ => {Vec::new()}
        }
    }
    fn new(args:&Value) -> Result<Self,&'static str> where Self:Sized{
        match args{
            Value::Array(values) => {
                if values.len() < 1{
                    return Err("too few arguments for SocketModeAllArgs")
                }
                let mut fmt_args = serde_json::Map::new();
                let data_type = &values[0];
                fmt_args.insert("data_type".to_string(),data_type.clone());
                serde_json::from_value::<StartDataStreamArgs>(Value::Object(fmt_args))
                    .map_err(|e| "could not map StartDataStreamArgs")
            }
            _ => {Err("unexpected value type for StartDataStreamArgs; expected Value::Array")}
        }
    }
}
#[derive(Deserialize,Serialize)]
pub struct ClearDataArgs;
impl FromArgs<Value> for ClearDataArgs{
    fn autocomplete_args(args:Vec<&str>) -> Vec<String> {Vec::new()}
    fn new(args:&Value) -> Result<Self,&'static str> where Self:Sized{
        Ok(ClearDataArgs)
    }
}
////////////////////////////////
