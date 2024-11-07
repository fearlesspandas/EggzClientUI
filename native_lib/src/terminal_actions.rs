use crate::traits::{CreateSignal,GetAll,Autocomplete,FromArgs};
use crate::socket_mode::SocketMode;
use crate::client_terminal::ClientTerminal;
use std::{fmt,str::FromStr};
use gdnative::prelude::*;
pub enum Actions{
    autocomplete_accept,
}
impl CreateSignal<ClientTerminal> for Actions{
    fn register(builder:&ClassBuilder<ClientTerminal>){
        builder
            .signal(&Actions::autocomplete_accept.to_string())
            .done();
    }
}
impl fmt::Display for Actions{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self{
            Actions::autocomplete_accept => {
                write!(f,"autocomplete_accept")
            }
        }
    }
}
