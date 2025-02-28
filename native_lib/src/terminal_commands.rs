use crate::traits::{CreateSignal,GetAll,Autocomplete,FromArgs};
use crate::socket_mode::SocketMode;
use crate::client_terminal::ClientTerminal;
use crate::data_display::DataType;
use crate::data_snapshots::DataSnapshots;
use std::{fmt,str::FromStr};
use serde_json::{Value};
use serde::{Deserialize,Serialize};
use gdnative::prelude::*;

///////////////////////////////
///COMMANDS////////////////////
#[derive(Debug,Deserialize)]
pub enum Command{
    SetEntitySocketMode(String,SocketMode),
    SetAllEntitySocketMode(SocketMode),
    StartDataStream(DataType),
    StopDataStream(DataType),
    ClearData,
    EntitiesAddMesh,
    EntitiesRemoveMesh,
    SaveSnapshot(String),
    LoadSnapshot(String),
    SetHealth(String,f32),
    GiveAbility(String,i32),
    //ToggleAggregateStats,
}
#[derive(Deserialize,Serialize,Debug)]
pub enum CommandType{
    set_entity_socket_mode,
    set_all_entity_socket_mode,
    start_data_stream,
    stop_data_stream,
    clear_data,
    entities_add_mesh,
    entities_remove_mesh,
    save_snapshot,
    load_snapshot,
    set_health,
    give_ability,
    //toggle_aggregate_stats,
}
impl CommandType{
    fn default(_args:Vec<&str>) -> Vec<String>{Vec::new()}
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
        builder
            .signal(&CommandType::set_health.to_string())
            .with_param("id",VariantType::GodotString)
            .with_param("value",VariantType::F64)
            .done();
        builder
            .signal(&CommandType::give_ability.to_string())
            .with_param("id",VariantType::GodotString)
            .with_param("value",VariantType::I64)
            .done();
        builder
            .signal(&CommandType::entities_add_mesh.to_string())
            .done();
        builder
            .signal(&CommandType::entities_remove_mesh.to_string())
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
        v.push( CommandType::entities_add_mesh);
        v.push( CommandType::entities_remove_mesh);
        v.push( CommandType::save_snapshot);
        v.push( CommandType::load_snapshot);
        v.push( CommandType::set_health);
        v.push( CommandType::give_ability);
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
            CommandType::save_snapshot => SaveSnapshotArgs::autocomplete_args,
            CommandType::load_snapshot => LoadSnapshotArgs::autocomplete_args,
            CommandType::set_health => SetHealthArgs::autocomplete_args,
            CommandType::give_ability => GiveAbilityArgs::autocomplete_args,
            CommandType::entities_add_mesh => CommandType::default,
            CommandType::entities_remove_mesh => CommandType::default,
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
                .map(|_parsed| Command::ClearData),
            CommandType::save_snapshot => 
                SaveSnapshotArgs::new(args)
                .map(|parsed| Command::SaveSnapshot(parsed.name)),
            CommandType::load_snapshot => 
                LoadSnapshotArgs::new(args)
                .map(|parsed| Command::LoadSnapshot(parsed.name)),
            CommandType::set_health => 
                SetHealthArgs::new(args)
                .map(|parsed| Command::SetHealth(parsed.id,parsed.value)),
            CommandType::give_ability => 
                GiveAbilityArgs::new(args)
                .map(|parsed| Command::GiveAbility(parsed.id,parsed.value)),
            CommandType::entities_add_mesh => Ok(Command::EntitiesAddMesh),
            CommandType::entities_remove_mesh => Ok(Command::EntitiesRemoveMesh),
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
            CommandType::set_health => {
                write!(f,"set_health")
            }
            CommandType::give_ability => {
                write!(f,"give_ability")
            }
            CommandType::clear_data => {
                write!(f,"clear_data")
            }
            CommandType::entities_add_mesh => {
                write!(f,"entities_add_mesh")
            }
            CommandType::entities_remove_mesh => {
                write!(f,"entities_remove_mesh")
            }
            CommandType::save_snapshot => {
                write!(f,"save_snapshot")
            }
            CommandType::load_snapshot => {
                write!(f,"load_snapshot")
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
            "entities_add_mesh" => Ok(CommandType::entities_add_mesh),
            "entities_remove_mesh" => Ok(CommandType::entities_remove_mesh),
            "save_snapshot" => Ok(CommandType::save_snapshot),
            "load_snapshot" => Ok(CommandType::load_snapshot),
            _ => Err(format!("No result found for command type {input:?}"))
        } 
    }
}
///////////////////////////////
///INPUT COMMANDS//////////////
#[derive(Deserialize,Serialize)]
pub struct InputCommand{
    pub typ: CommandType,
    pub args: Value
}
impl ArgsConstructor<Command,(),&'static str> for InputCommand{
    fn from_args(&self,_args:()) -> Result<Command,&'static str>{
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
                    .map_err(|_e| "Error while parsing args for SocketModeArgs")
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
                    .map_err(|_e| "Error while parsing args for SocketModeAllArgs")
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
                    .map_err(|_e| "could not map StartDataStreamArgs")
            }
            _ => {Err("unexpected value type for StartDataStreamArgs; expected Value::Array")}
        }
    }
}
#[derive(Deserialize,Serialize)]
pub struct SetHealthArgs{
    pub id:String,
    pub value:f32
}
impl FromArgs<Value> for SetHealthArgs{
    fn autocomplete_args(args:Vec<&str>) -> Vec<String>{
        match args.len(){
            0|1 => {
                let mut v = Vec::new();
                v.push("id:String".to_string());
                v
            }
            2 => {
                let mut v = Vec::new();
                v.push("value:f32".to_string());
                v
            }
            _ => {Vec::new()}
        }
    }
    fn new(args:&Value) -> Result<Self,&'static str> where Self:Sized{
        match args{
            Value::Array(values) => {
                if values.len() < 2{
                    return Err("too few arguments for SetHealthArgs")
                }
                let mut fmt_args = serde_json::Map::new();
                let id = &values[0];
                let value = &values[1];
                fmt_args.insert("id".to_string(),id.clone());
                fmt_args.insert("value".to_string(),value.clone().as_str().unwrap().parse::<f32>().unwrap().into());
                serde_json::from_value::<SetHealthArgs>(Value::Object(fmt_args))
                    .map_err(|e| {godot_print!("{}",format!("Error constructing set_health args; {e:?}"));"could not map SetHealthArgs"})
            }
            _ => {Err("unexpected value type for SetHealthArgs; expected Value::Array")}
        }
    }
}
#[derive(Deserialize,Serialize)]
pub struct GiveAbilityArgs{
    pub id:String,
    pub value:i32
}
impl FromArgs<Value> for GiveAbilityArgs{
    fn autocomplete_args(args:Vec<&str>) -> Vec<String>{
        match args.len(){
            0|1 => {
                let mut v = Vec::new();
                v.push("id:String".to_string());
                v
            }
            2 => {
                let mut v = Vec::new();
                v.push("value:i32".to_string());
                v
            }
            _ => {Vec::new()}
        }
    }
    fn new(args:&Value) -> Result<Self,&'static str> where Self:Sized{
        match args{
            Value::Array(values) => {
                if values.len() < 2{
                    return Err("too few arguments for GiveAbilityArgs")
                }
                let mut fmt_args = serde_json::Map::new();
                let id = &values[0];
                let value = &values[1];
                fmt_args.insert("id".to_string(),id.clone());
                fmt_args.insert("value".to_string(),value.clone().as_str().unwrap().parse::<i32>().unwrap().into());
                serde_json::from_value::<GiveAbilityArgs>(Value::Object(fmt_args))
                    .map_err(|e| {godot_print!("{}",format!("Error constructing give_ability args; {e:?}"));"could not map GiveAbilityArgs"})
            }
            _ => {Err("unexpected value type for GiveAbilityArgs; expected Value::Array")}
        }
    }
}
#[derive(Deserialize,Serialize)]
pub struct ClearDataArgs;
impl FromArgs<Value> for ClearDataArgs{
    fn autocomplete_args(_args:Vec<&str>) -> Vec<String> {Vec::new()}
    fn new(_args:&Value) -> Result<Self,&'static str> where Self:Sized{
        Ok(ClearDataArgs)
    }
}
#[derive(Deserialize,Serialize)]
pub struct SaveSnapshotArgs{
    pub name:String 
}
impl FromArgs<Value> for SaveSnapshotArgs{
    fn autocomplete_args(args:Vec<&str>) -> Vec<String>{
        match args.len(){
            0 => {
                DataType::get_all()
                    .into_iter()
                    .map(|data_type| data_type.to_string())
                    .collect()
            }
            1 => {
                let mut v = Vec::new();
                v.push("name:String".to_string());
                v
            }
            _ => {Vec::new()}
        }
    }
    fn new(args:&Value) -> Result<Self,&'static str> where Self:Sized{
        match args{
            Value::Array(values) => {
                if values.len() < 1{
                    return Err("too few arguments for SaveSnapshotArgs")
                }
                let mut fmt_args = serde_json::Map::new();
                let data_type = &values[0];
                fmt_args.insert("name".to_string(),data_type.clone());
                serde_json::from_value::<SaveSnapshotArgs>(Value::Object(fmt_args))
                    .map_err(|e| {let err = e.to_string();godot_print!("{}",err);"could not map SaveSnapshotArgs "})
            }
            _ => {Err("unexpected value type for SaveSnapshotArgs; expected Value::Array")}
        }
    }
}
#[derive(Deserialize,Serialize)]
pub struct LoadSnapshotArgs{
    pub name:String 
}
impl FromArgs<Value> for LoadSnapshotArgs{
    fn autocomplete_args(args:Vec<&str>) -> Vec<String>{
        match args.len(){
            0 => {
                DataType::get_all()
                    .into_iter()
                    .map(|data_type| data_type.to_string())
                    .collect()
            }
            1 => {
                DataSnapshots::get_available_snapshots("user://snapshots".to_string())
                    .into_iter()
                    .filter(|name| name.contains(&args[0]))
                    .collect::<Vec<String>>()
            }
            _ => {Vec::new()}
        }
    }
    fn new(args:&Value) -> Result<Self,&'static str> where Self:Sized{
        match args{
            Value::Array(values) => {
                if values.len() < 1{
                    return Err("too few arguments for LoadSnapshotArgs")
                }
                let mut fmt_args = serde_json::Map::new();
                let data_type = &values[0];
                fmt_args.insert("name".to_string(),data_type.clone());
                serde_json::from_value::<LoadSnapshotArgs>(Value::Object(fmt_args))
                    .map_err(|e| {let err = e.to_string();godot_print!("{}",err);"could not map LoadSnapshotArgs "})
            }
            _ => {Err("unexpected value type for LoadSnapshotArgs; expected Value::Array")}
        }
    }
}
////////////////////////////////
