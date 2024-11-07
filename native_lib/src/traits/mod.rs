use gdnative::prelude::*;
use gdnative::api::*;
use serde_json::{Result as JResult, Value};
use serde::{Deserialize,Serialize};
use tokio::sync::mpsc;
use std::{fmt,str::FromStr};

//useful for making sure we can retrieve all values of an enum
pub trait GetAll{
    fn get_all() -> Vec<Self> where Self:Sized;
}
//matches enum type variant to an autocomplete function
pub trait Autocomplete{
    fn auto_complete(&self) -> fn(Vec<&str>) -> Vec<String>;
}
//handles creation of typed args as well as expects an autocomplete function implementation
pub trait FromArgs<T>{
    fn new(args:&T) -> Result<Self,&'static str> where Self:Sized;
    fn autocomplete_args(args:Vec<&str>) -> Vec<String>;
}
pub trait CreateSignal<T>{
    fn register(builder:&ClassBuilder<T>);
}
pub trait EmitSignal{
    fn emit_signal<T>(&self, owner:T);
}
