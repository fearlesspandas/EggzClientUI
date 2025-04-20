use gdnative::prelude::*;
use gdnative::api::*;
use serde::{Serialize,Deserialize}; 
use serde_json::{Value};
use tokio::sync::mpsc;
use std::collections::HashMap;
use crate::traits::{Defaulted,Instanced};
use crate::field::{Location,FieldCommand,FieldZone};
use crate::field_abilities::{AbilityType};
use crate::field_ability_colliders::{ToCollider,FieldCollider,AbilityCollider};
use crate::field_ability_mesh::{ToMesh,FieldMesh,AbilityMesh};
use crate::field_ability_actions::{ToAction};
use crate::assets::{array_mesh,point_material,point_mesh};

type Sender<T> = mpsc::UnboundedSender<T>;

trait Signals<T>{
    fn command_tx(&self) -> &Sender<T>;
    fn set_command_tx(&mut self,tx:Sender<T>);
}
pub enum AbilityOps<T>{
    set_tx(T,Sender<FieldCommand>),
    update_icount(T,i64),
    update_pcount(T,i64),
}
impl <T> AbilityOps<T>{
    fn update<F,E>(&mut self,f:F) -> Result<&mut Self,E> 
        where 
            F:FnOnce(&mut Self) -> Result<(),E>
    {
        f(self).map(|_| self)
    }

}
impl AbilityOps<Instance<Smack>>{
    fn exec(&self){
        match self{
            AbilityOps::<Instance<Smack>>::set_tx(smack,tx) => {
                let smack = unsafe{smack.assume_safe()};
                let _ = smack.map_mut(|obj,_| obj.set_command_tx(tx.clone()));
            }
            AbilityOps::<Instance<Smack>>::update_icount(smack,value) => {
                let smack = unsafe{smack.assume_safe()};
                let _ = smack.map_mut(|obj,_| obj.set_instance_count(value.clone()));
            }
            AbilityOps::<Instance<Smack>>::update_pcount(smack,value) => {
                let smack = unsafe{smack.assume_safe()};
                let _ = smack.map_mut(|obj,_| obj.set_proc_count(value.clone()));
            }
            _ => {}
        }
    }
}
trait ToAbilityType{
    fn typ(&self) -> AbilityType;
}
//SERIALIZATION///////
#[derive(Serialize,Deserialize)]
pub struct AbilityData{
    typ:AbilityType,
    location:(i64,i64),
    data:Value,
}
////FIELD ABILITIES////
////ability representations
////while on the field;
////activation logic,
////state representation
pub trait FieldAbility:Sized +  ToAbilityType{
    fn instance_count(&self) -> i64;
    fn set_instance_count(&mut self,value:i64);
    fn proc_count(&self) -> i64;
    fn set_proc_count(&mut self,value:i64);
}
pub trait ClientFieldAbility:FieldAbility + FieldMesh + ToAction{
    fn radius(&self) -> f32;
    fn modify_data(&self,data:AbilityData);
}
pub trait ServerFieldAbility:FieldAbility + ToAction + FieldCollider{
    fn radius(&self) -> f32;
    fn modify_data(&self,data:AbilityData);
}
////ABILITIES////
////ability activations and affects on the world
////collider and mesh handling during ability affects

type AbilityId = i64;
pub trait Ability:Sized + ToAbilityType{
    fn id(&self) -> AbilityId;
}
pub trait ClientAbility:Ability + AbilityMesh + ToAction + Sized{
    fn mesh(&self) -> Ref<Spatial>;
    fn duration(&self) -> f32;
    fn set_duration(&self , value:f32);

    fn default_ability_mesh(&self) -> Ref<Spatial>{
        <Self as AbilityMesh>::to_mesh(self,25.0,25.0)
    }
}
pub trait ServerAbility:Ability + AbilityCollider{
    fn radius(&self) -> f32;
    fn duration(&self) -> f32;
    fn set_duration(&self , value:f32);
}
pub trait TimedAbility:Ability{
    fn duration(&self) -> f32;
    fn destroy(&self,owner:TRef<Node>){
        owner.get_parent().map(|parent| {
            let parent = unsafe{parent.assume_safe()};
            parent.remove_child(owner);
        });
        owner.queue_free();
    }
    fn ready(&self,owner:TRef<Node>){
        assert!(owner.has_method("destroy"),"owner does not have destroy method");
        let timer = Timer::new().into_shared();
        let timer = unsafe{timer.assume_safe()};
        owner.add_child(timer,true);
        let _ = timer.connect("timeout",owner,"destroy",VariantArray::new_shared(),0);
        timer.start(self.duration().into());
    }
}
impl From<AbilityType> for Abilities{
    fn from(item:AbilityType) -> Self{
        match item{
            AbilityType::smack => {
                let smack = Smack::make_instance().into_shared();
                Abilities::smack(smack)
            }
            AbilityType::globular_teleport => {
                let gt = GlobularTeleport::make_instance().into_shared();
                Abilities::globular_teleport(gt)
            }
            _ => todo!()
        }
    }
}
pub enum Abilities{
    smack(Instance<Smack>),
    globular_teleport(Instance<GlobularTeleport>),
}
impl ClientFieldAbility for Abilities{
    fn radius(&self) -> f32{
        match self{
            Abilities::smack(smack) => {
                let smack = unsafe{smack.assume_safe()};
                smack.map(|obj,_| (obj as dyn ClientFieldAbility).radius()).expect("Abilities:Could not get Smack radius")
            }
            Abilities::globular_teleport(globular_teleport) => {
                let globular_teleport = unsafe{globular_teleport.assume_safe()};
                globular_teleport.map(|obj,_| (obj as dyn ClientFieldAbility).radius()).expect("Abilities:Could not get GlobularTeleport radius")
            }
        }
    }
    fn modify_data(&self,data:AbilityData){}
}
impl ToAbilityType for Abilities{
    fn typ(&self) -> AbilityType;
}
impl Signals<FieldCommand> for Abilities{
    fn command_tx(&self) -> &Sender<FieldCommand>{
        match self{
            Abilities::smack(smack) => {
                let smack = unsafe{smack.assume_safe()};
                smack.map(|obj,_| (obj as dyn Signals<FieldCommand>).command_tx()).expect("Abilities:Could not get Smack radius")
            }
            Abilities::globular_teleport(globular_teleport) => {
                let globular_teleport = unsafe{globular_teleport.assume_safe()};
                globular_teleport.map(|obj,_| (obj as dyn Signals<FieldCommand>).command_tx()).expect("Abilities:Could not get GlobularTeleport radius")
            }
        }
    }
    fn set_command_tx(&mut self,tx:Sender<FieldCommand>){
        match self{
            Abilities::smack(smack) => {
                let smack = unsafe{smack.assume_safe()};
                smack.map(|obj,_| (obj as dyn Signals<FieldCommand>).set_command_tx(tx));
            }
            Abilities::globular_teleport(globular_teleport) => {
                let globular_teleport = unsafe{globular_teleport.assume_safe()};
                globular_teleport.map(|obj,_| (obj as dyn Signals<FieldCommand>).set_command_tx(tx));
            }
        }
    }
}
impl FieldMesh for Abilities{
    fn to_mesh(&self,length:f32,radius:f32) -> Ref<Spatial>{
        match self{
            Abilities::smack(smack) => {
                let smack = unsafe{smack.assume_safe()};
                smack.map(|obj,_| (obj as dyn FieldMesh).to_mesh(length,radius))
                    .expect("Abilities:Could not get Smack radius")
            }
            Abilities::globular_teleport(globular_teleport) => {
                let globular_teleport = unsafe{globular_teleport.assume_safe()};
                globular_teleport.map(|obj,_| (obj as dyn FieldMesh).to_mesh(length,radius))
                    .expect("Abilities:Could not get GlobularTeleport radius")
            }
        }
    }
}
impl AbilityMesh for Abilities{
    fn to_mesh(&self,length:f32,radius:f32) -> Ref<Spatial>{
        match self{
            Abilities::smack(smack) => {
                let smack = unsafe{smack.assume_safe()};
                smack.map(|obj,_| (obj as dyn AbilityMesh).to_mesh(length,radius))
                    .expect("Abilities:Could not get Smack radius")
            }
            Abilities::globular_teleport(globular_teleport) => {
                let globular_teleport = unsafe{globular_teleport.assume_safe()};
                globular_teleport.map(|obj,_| (obj as dyn AbilityMesh).to_mesh(length,radius))
                    .expect("Abilities:Could not get GlobularTeleport radius")
            }
        }
    }
}
impl ToAction for Abilities{
    fn to_action(&self,tx:Sender<FieldCommand>,location:&Location,field_state:&HashMap<Location,Instance<FieldZone>>){
        match self{
            Abilities::smack(smack) => {
                let smack = unsafe{smack.assume_safe()};
                smack.map(|obj,_| (obj as dyn ToAction).to_action(tx,location,field_state));
            }
            Abilities::globular_teleport(globular_teleport) => {
                let globular_teleport = unsafe{globular_teleport.assume_safe()};
                globular_teleport.map(|obj,_| (obj as dyn ToAction).to_action(tx,location,field_state));
            }
        }

    }
}
impl AbilityCollider for Abilities{
    fn to_collider(&self,extents:Vector3) -> Option<Ref<Area>> {
        match self{
            Abilities::smack(smack) => {
                let smack = unsafe{smack.assume_safe()};
                smack.map(|obj,_| (obj as dyn AbilityCollider).to_collider(extents))
                    .expect("Abilities:Could not get Smack radius")
            }
            Abilities::globular_teleport(globular_teleport) => {
                let globular_teleport = unsafe{globular_teleport.assume_safe()};
                globular_teleport.map(|obj,_| (obj as dyn AbilityCollider).to_collider(extents))
                    .expect("Abilities:Could not get GlobularTeleport radius")
            }
        }
    }
}
impl FieldCollider for Abilities{
    fn to_collider(&self,extents:Vector3) -> Option<Ref<Area>> {
        match self{
            Abilities::smack(smack) => {
                let smack = unsafe{smack.assume_safe()};
                smack.map(|obj,_| (obj as dyn FieldCollider).to_collider(extents))
                    .expect("Abilities:Could not get Smack radius")
            }
            Abilities::globular_teleport(globular_teleport) => {
                let globular_teleport = unsafe{globular_teleport.assume_safe()};
                globular_teleport.map(|obj,_| (obj as dyn FieldCollider).to_collider(extents))
                    .expect("Abilities:Could not get GlobularTeleport radius")
            }
        }
    }
}
impl FieldAbility for Abilities{
    fn instance_count(&self) -> i64{
        match self{
            Abilities::smack(smack) => {
                let smack = unsafe{smack.assume_safe()};
                smack.map(|obj,_| (obj as dyn FieldAbility).instance_count())
                    .expect("Abilities:Could not get Smack radius")
            }
            Abilities::globular_teleport(globular_teleport) => {
                let globular_teleport = unsafe{globular_teleport.assume_safe()};
                globular_teleport.map(|obj,_| (obj as dyn FieldAbility).instance_count())
                    .expect("Abilities:Could not get GlobularTeleport radius")
            }
        }
    }
    fn set_instance_count(&mut self,value:i64){
        match self{
            Abilities::smack(smack) => {
                let smack = unsafe{smack.assume_safe()};
                smack.map(|obj,_| (obj as dyn FieldAbility).set_instance_count(value));
            }
            Abilities::globular_teleport(globular_teleport) => {
                let globular_teleport = unsafe{globular_teleport.assume_safe()};
                globular_teleport.map(|obj,_| (obj as dyn FieldAbility).set_instance_count(value));
            }
        }
    }
    fn proc_count(&self) -> i64{
        match self{
            Abilities::smack(smack) => {
                let smack = unsafe{smack.assume_safe()};
                smack.map(|obj,_| (obj as dyn FieldAbility).proc_count())
                    .expect("Abilities:Could not get Smack radius")
            }
            Abilities::globular_teleport(globular_teleport) => {
                let globular_teleport = unsafe{globular_teleport.assume_safe()};
                globular_teleport.map(|obj,_| (obj as dyn FieldAbility).proc_count())
                    .expect("Abilities:Could not get GlobularTeleport radius")
            }
        }
    }
    fn set_proc_count(&mut self,value:i64){
        match self{
            Abilities::smack(smack) => {
                let smack = unsafe{smack.assume_safe()};
                smack.map(|obj,_| (obj as dyn FieldAbility).set_proc_count(value));
            }
            Abilities::globular_teleport(globular_teleport) => {
                let globular_teleport = unsafe{globular_teleport.assume_safe()};
                globular_teleport.map(|obj,_| (obj as dyn FieldAbility).set_proc_count(value));
            }
        }
    }
}
////ABILITY IMPLEMENTATIONS////
//////////////////////////////
////Smack
#[derive(NativeClass,Clone)]
#[inherit(Node)]
pub struct Smack{
    id:AbilityId,
    radius:f32,
    cmd_tx:Option<Sender<FieldCommand>>,
    instance_count:i64,
    proc_count:i64,
}
impl Instanced<Node> for Smack{
    fn make() -> Self{
        Smack{
            id:-1,
            radius:0.0,
            cmd_tx:None,
            instance_count:0,
            proc_count:0,
        }
    }
}
#[methods]
impl Smack{
    #[method]
    fn _ready(&self,#[base] owner:TRef<Node>){
        let mesh = <Self as FieldMesh>::to_mesh(self,25.0,25.0);
        let collider = <Self as FieldCollider>::to_collider(self,Vector3::new(25.0,25.0,25.0)).expect("ability:No collider found for smack client");
        owner.add_child(mesh,true);
        owner.add_child(collider,true);
        <Self as TimedAbility>::ready(self,owner);
        
    }
    #[method]
    fn destroy(&self,#[base] owner:TRef<Node>){
        <Self as TimedAbility>::destroy(self,owner);
    }
}
impl FieldAbility for Smack{ 
    fn instance_count(&self) -> i64{self.instance_count}
    fn set_instance_count(&mut self,value:i64){self.instance_count = value;}
    fn proc_count(&self) -> i64{self.proc_count}
    fn set_proc_count(&mut self,value:i64){self.proc_count = value;}
}
impl Ability for Smack{
    fn id(&self) -> AbilityId {
        self.id
    }
}
impl ToAbilityType for Smack{
    fn typ(&self) -> AbilityType;
}
impl Signals<FieldCommand> for Smack{
    fn command_tx(&self) -> &Sender<FieldCommand>{&self.cmd_tx.as_ref().expect("SmackErr:No outbound tx found")}
    fn set_command_tx(&mut self,tx:Sender<FieldCommand>){self.cmd_tx = Some(tx)}
}
impl ClientFieldAbility for Smack{
    fn radius(&self) -> f32{self.radius}
    fn modify_data(&self,data:AbilityData){}
}
impl FieldMesh for Smack{
    fn to_mesh(&self,length:f32,radius:f32) -> Ref<Spatial>{
        self.typ().to_mesh(length,radius)
    }
}
impl AbilityMesh for Smack{
    fn to_mesh(&self,length:f32,radius:f32) -> Ref<Spatial>{
        self.typ().to_mesh(length,radius)
    }
}
impl ToAction for Smack{
    fn to_action(&self,tx:Sender<FieldCommand>,location:&Location,field_state:&HashMap<Location,Instance<FieldZone>>){
        let _  = tx.send(FieldCommand::DoAbility(location.clone(),self.typ()));
    }
}
impl AbilityCollider for Smack{
    fn to_collider(&self,extents:Vector3) -> Option<Ref<Area>> {
        let sphere_shape = SphereShape::new().into_shared();
        let sphere_shape = unsafe{sphere_shape.assume_safe()};
        sphere_shape.set_radius(35.0);
        let collider = CollisionShape::new().into_shared();
        let collider = unsafe{collider.assume_safe()};
        collider.set_shape(sphere_shape);
        let area = Area::new().into_shared();
        let area = unsafe{area.assume_safe()};
        area.add_child(collider,true);
        Some(area.claim())
    }
}
impl FieldCollider for Smack{
    fn to_collider(&self,extents:Vector3) -> Option<Ref<Area>> {
        None
    }
}
impl TimedAbility for Smack{
    fn duration(&self) -> f32{
        3.0
    }
}
////Globular Teleport////
#[derive(NativeClass)]
#[inherit(Node)]
pub struct GlobularTeleport{
    radius:f32,
    cmd_tx:Sender<FieldCommand>,
    base:Vector3,
    points:PoolArray<Vector3>,
    instance_count:i64,
    proc_count:i64,
}
impl Instanced<Node> for GlobularTeleport{
    fn make() -> Self{
        GlobularTeleport{
            radius:0.0,
            cmd_tx:todo!(),
            base:todo!(),
            points:PoolArray::new(),
            instance_count:0,
            proc_count:0,
        }
    }
}
impl Signals<FieldCommand> for GlobularTeleport{
    fn command_tx(&self) -> &Sender<FieldCommand>{&self.cmd_tx}
    fn set_command_tx(&mut self,tx:Sender<FieldCommand>){self.cmd_tx = tx}
}
impl ClientFieldAbility for GlobularTeleport{
    fn radius(&self) -> f32{self.radius}
    fn modify_data(&self,data:AbilityData){}
}
impl ToAbilityType for GlobularTeleport{
    fn typ(&self) -> AbilityType;
}
impl FieldMesh for GlobularTeleport{
    fn to_mesh(&self,length:f32,radius:f32) -> Ref<Spatial>{
        self.typ().to_mesh(length,radius)
    }
}
impl AbilityMesh for GlobularTeleport{
    fn to_mesh(&self,length:f32,radius:f32) -> Ref<Spatial>{
        let glob_mesh = array_mesh(self.points.clone(),Mesh::PRIMITIVE_LINES);
        let base_mesh = point_mesh(20.0,Color::from_rgba(0.0,0.0,100.0,1.0));
        
        let mesh_instance = MeshInstance::new().into_shared();
        let mesh_instance = unsafe{mesh_instance.assume_safe()};
        mesh_instance.set_mesh(glob_mesh);

        let base_mesh_instance = MeshInstance::new().into_shared();
        let base_mesh_instance = unsafe{base_mesh_instance.assume_safe()};
        base_mesh_instance.set_mesh(base_mesh);

        let spatial = Spatial::new().into_shared();
        let spatial = unsafe{spatial.assume_safe()};

        spatial.add_child(mesh_instance,true);
        spatial.add_child(base_mesh_instance,true);
        spatial.claim()
    }
}
impl ToAction for GlobularTeleport{
    fn to_action(&self,tx:Sender<FieldCommand>,location:&Location,field_state:&HashMap<Location,Instance<FieldZone>>){

    }
}
impl AbilityCollider for GlobularTeleport{
    fn to_collider(&self,extents:Vector3) -> Option<Ref<Area>> {
        None
    }
}
impl FieldCollider for GlobularTeleport{
    fn to_collider(&self,extents:Vector3) -> Option<Ref<Area>> {
        None
    }
}
impl FieldAbility for GlobularTeleport{
    fn instance_count(&self) -> i64{self.instance_count}
    fn set_instance_count(&mut self,value:i64){self.instance_count = value;}
    fn proc_count(&self) -> i64{self.proc_count}
    fn set_proc_count(&mut self,value:i64){self.proc_count = value;}
}
