
use gdnative::prelude::*;
use gdnative::api::*;
use crate::traits::{Defaulted};

#[derive(Copy,Clone,Eq,Hash,PartialEq,Debug)]
pub enum AbilityType{
    empty,
    smack,
    globular_teleport
}
impl Defaulted for AbilityType{
    fn default() -> Self{
        AbilityType::empty
    }
}
impl From<u8> for AbilityType{
    fn from(value:u8) -> Self{
        match value{
            255 => AbilityType::empty,
            0 => AbilityType::smack,
            1 => AbilityType::globular_teleport,
            _ => todo!(),
        }
    }
}
impl Into<u8> for AbilityType{
    fn into(self) -> u8 {
        match self{
            AbilityType::empty => 255,
            AbilityType::smack => 0,
            AbilityType::globular_teleport => 1,
        }
    }
}
trait ToLabel{
    fn to_label(&self) -> String;
}
impl ToString for AbilityType{
    fn to_string(&self) -> String{
        match self{
            AbilityType::empty => "Empty".to_string(),
            AbilityType::smack => "Smack".to_string(),
            AbilityType::globular_teleport => "Globular Teleport".to_string(),
        }
    }
}
#[derive(Copy,Clone,Eq,Hash,PartialEq)]
pub enum SubAbilityType{
    empty,
    globular_teleport_anchor,
    globular_teleport_vertex,
}
impl From<u8> for SubAbilityType{
    fn from(value:u8) -> Self{
        match value{
            255 => SubAbilityType::empty,
            0 => SubAbilityType::globular_teleport_anchor,
            1 => SubAbilityType::globular_teleport_vertex,
            _ => todo!(),
        }
    }
}
impl Into<u8> for SubAbilityType{
    fn into(self) -> u8 {
        match self{
            SubAbilityType::empty => 255,
            SubAbilityType::globular_teleport_anchor => 0,
            SubAbilityType::globular_teleport_vertex => 1,
        }
    }
}
