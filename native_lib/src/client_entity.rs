
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


trait ClientEntity{
    fn body(&self) -> &KinematicBody;
    fn default_physics_process(delta:f64){
    }
}
