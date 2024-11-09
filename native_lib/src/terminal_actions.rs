use crate::traits::{CreateSignal,GetAll,Autocomplete,FromArgs};
use crate::socket_mode::SocketMode;
use crate::client_terminal::ClientTerminal;
use std::{fmt,str::FromStr};
use gdnative::prelude::*;

pub enum Action{
    AutoCompleteAccept,
    SetActive(bool),
}
pub enum ActionType{
    autocomplete_accept,
    set_active,
}
impl CreateSignal<ClientTerminal> for ActionType{
    fn register(builder:&ClassBuilder<ClientTerminal>){
        builder
            .signal(&ActionType::set_active.to_string())
            .with_param("value",VariantType::Bool)
            .done();
    }
}
impl fmt::Display for ActionType{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self{
            ActionType::autocomplete_accept => {
                write!(f,"autocomplete_accept")
            }
            ActionType::set_active => {
                write!(f,"set_active")
            }
        }
    }
}
