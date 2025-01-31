
use std::collections::HashMap;
use gdnative::prelude::*;
use gdnative::api::*;
use crate::traits::{CreateSignal,Instanced,InstancedDefault,Defaulted};
use crate::field::{OpType,SubOpType,Location,FieldZone,FieldCommand};
use tokio::sync::mpsc;

type Sender<T> = mpsc::UnboundedSender<T>;
pub trait ToAction{
    fn to_action(&self,tx:Sender<FieldCommand>,location:&Location,field_state:&HashMap<Location,Instance<FieldZone>>);
}
impl ToAction for OpType{
    fn to_action(&self,tx:Sender<FieldCommand>,location:&Location,field_state:&HashMap<Location,Instance<FieldZone>>){
        match self{
            OpType::empty => { }
            OpType::smack => { 
                tx.send(FieldCommand::DoAbility(*location,OpType::smack));
            }
            OpType::globular_teleport => {
                let zone = field_state.get(location).unwrap();
                let zone = unsafe{zone.assume_safe()};
                let matching_zones = field_state.values().into_iter().filter(|mzone|{
                    let mzone = unsafe{mzone.assume_safe()};
                    mzone.map(|obj,_| obj.proc &&
                        obj
                        .abilities
                        .clone()
                        .into_iter()
                        .filter(|pair| {
                            let (key,_) = pair;
                            *key == OpType::globular_teleport
                        })
                        .collect::<Vec<_>>()
                        .len() > 0
                        ).unwrap_or(false)
                }).collect::<Vec<_>>();

                let matching_num = matching_zones.len();
                godot_print!("{}",format!("Matching:{matching_num:?}"));
                match matching_zones.len() {
                    0 => {
                        tx.send(FieldCommand::ModifyAbility(*location,SubOpType::globular_teleport_anchor));
                        zone.map_mut(|obj,_| obj.proc());
                    }
                    1 | 2 | 3 => {
                        tx.send(FieldCommand::ModifyAbility(*location,SubOpType::globular_teleport_vertex));
                        zone.map_mut(|obj,_| obj.proc());
                    }
                    _ => {
                        tx.send(FieldCommand::ModifyAbility(*location,SubOpType::globular_teleport_vertex));
                        tx.send(FieldCommand::DoAbility(*location,OpType::globular_teleport));
                        for mzone in matching_zones{
                            let mzone = unsafe{mzone.assume_safe()};
                            mzone.map_mut(|obj,body| obj.remove_ability(body,(*self).into()));
                            mzone.map_mut(|obj,body| obj.unproc());
                        }
                        zone.map_mut(|obj,body| obj.remove_ability(body,(*self).into()));
                    }
                }
            }
        }
    }
}

