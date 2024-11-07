use tokio::sync::mpsc;
use std::{fmt,str::FromStr};
use crate::traits::{CreateSignal,GetAll,Autocomplete,FromArgs};
use crate::socket_mode::SocketMode;
use serde_json::{Result as JResult, Value};
use serde::{Deserialize,Serialize};
use gdnative::prelude::*;
use gdnative::api::*;
use crate::client_terminal::ClientTerminal;
#[derive(Debug,Deserialize)]
pub enum Command{
    SetEntitySocketMode(String,SocketMode),
    SetAllEntitySocketMode(SocketMode),
}
#[derive(Deserialize,Serialize,Debug)]
pub enum CommandType{
    //set_active,
    set_entity_socket_mode,
    set_all_entity_socket_mode,
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
    }
}
impl GetAll for CommandType{
    fn get_all() -> Vec<Self> where Self:Sized{
        let mut v = Vec::new();
        v.push( CommandType::set_entity_socket_mode);
        v.push( CommandType::set_all_entity_socket_mode);
        v
    }
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

#[derive(Deserialize,Serialize)]
pub struct InputCommand{
    pub typ:CommandType,
    pub args: Value
}
#[derive(Deserialize,Serialize)]
pub struct SocketModeArgs{
    pub id:String,
    pub mode:SocketMode,
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
