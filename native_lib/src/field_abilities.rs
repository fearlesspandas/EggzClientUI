
use gdnative::prelude::*;
use gdnative::api::*;
use crate::traits::{Defaulted};

#[derive(Copy,Clone,Eq,Hash,PartialEq,Debug)]
pub enum OpType{
    empty,
    smack,
    globular_teleport
}
impl Defaulted for OpType{
    fn default() -> Self{
        OpType::empty
    }
}
impl From<u8> for OpType{
    fn from(value:u8) -> Self{
        match value{
            255 => OpType::empty,
            0 => OpType::smack,
            1 => OpType::globular_teleport,
            _ => todo!(),
        }
    }
}
impl Into<u8> for OpType{
    fn into(self) -> u8 {
        match self{
            OpType::empty => 255,
            OpType::smack => 0,
            OpType::globular_teleport => 1,
        }
    }
}
#[derive(Copy,Clone,Eq,Hash,PartialEq)]
pub enum SubOpType{
    empty,
    globular_teleport_anchor,
    globular_teleport_vertex,
}
impl From<u8> for SubOpType{
    fn from(value:u8) -> Self{
        match value{
            255 => SubOpType::empty,
            0 => SubOpType::globular_teleport_anchor,
            1 => SubOpType::globular_teleport_vertex,
            _ => todo!(),
        }
    }
}
impl Into<u8> for SubOpType{
    fn into(self) -> u8 {
        match self{
            SubOpType::empty => 255,
            SubOpType::globular_teleport_anchor => 0,
            SubOpType::globular_teleport_vertex => 1,
        }
    }
}
