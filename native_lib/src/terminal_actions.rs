use crate::traits::{CreateSignal};
use crate::client_terminal::ClientTerminal;
use crate::data_display::DataType;
use std::{fmt};
use gdnative::prelude::*;

pub enum Action{
    AutoCompleteAccept,
    SetActive(bool),
    RequestData(DataType)
}
pub enum ActionType{
    autocomplete_accept,
    set_active,
    request_data,
}
impl CreateSignal<ClientTerminal> for ActionType{
    fn register(builder:&ClassBuilder<ClientTerminal>){
        builder
            .signal(&ActionType::set_active.to_string())
            .with_param("value",VariantType::Bool)
            .done();
        builder
            .signal(&ActionType::request_data.to_string())
            .with_param("data_type",VariantType::GodotString)
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
            ActionType::request_data => {
                write!(f,"request_data")
            }
        }
    }
}
