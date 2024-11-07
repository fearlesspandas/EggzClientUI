use std::{fmt,str::FromStr};
use crate::traits::{GetAll};
use serde::{Deserialize,Serialize};

#[derive(Deserialize,Serialize,Debug)]
pub enum SocketMode{
    Native,
    NativeProcess,
    GodotClient,
}
impl GetAll for SocketMode{
    fn get_all() -> Vec<Self> where Self:Sized{
        let mut v = Vec::new();
        v.push(SocketMode::Native);
        v.push(SocketMode::NativeProcess);
        v.push(SocketMode::GodotClient);
        v
    }
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
