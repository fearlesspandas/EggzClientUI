
use std::collections::HashMap;
use gdnative::prelude::*;
use gdnative::api::*;
use crate::traits::{Instanced,InstancedDefault};
use crate::field::{Location,ZONE_HEIGHT,ZONE_WIDTH};
use crate::field_abilities::{AbilityType};
use crate::field_ability_colliders::ToCollider;
use crate::field_ability_actions::ServerEnteredAction;
use tokio::sync::mpsc;

type Sender<T> = mpsc::UnboundedSender<T>;
type Receiver<T> = mpsc::UnboundedReceiver<T>;

pub enum FieldZoneCommand{
}
#[derive(NativeClass)]
#[inherit(Area)]
#[register_with(Self::register_signals)]
pub struct FieldZoneServer{
    typ:AbilityType,
    location:Location,
    abilities:Option<Ref<Area>>,
    //zone_tx:Sender<FieldZoneCommand>,
    //zone_rx:Receiver<FieldZoneCommand>,
    field_tx:Option<Sender<FieldCommand>>,
}
impl InstancedDefault<Area,Location> for FieldZoneServer{
    fn make(args:&Location) -> Self{
        FieldZoneServer{
            typ:AbilityType::empty,
            location:*args,
            abilities:None,
            //zone_tx: tx,
            //zone_rx: rx,
            field_tx:None,
        }
    }
}
#[methods]
impl FieldZoneServer{
    fn register_signals(builder:&ClassBuilder<Self>){
        builder
            .signal("damage")
            .with_param("id",VariantType::GodotString)
            .with_param_default("amount",Variant::new(0.0))
            .done();
    }
    #[method]
    fn _ready(&self,#[base] owner:TRef<Area>){
        //initialize
        let collision_shape = BoxShape::new().into_shared();
        let collision_shape = unsafe{collision_shape.assume_safe()};
        collision_shape.set_extents(Vector3{x:ZONE_WIDTH/2.0,y:ZONE_HEIGHT/2.0,z:ZONE_WIDTH/2.0});

        let collider = CollisionShape::new().into_shared();
        let collider = unsafe{collider.assume_safe()};
        collider.set_shape(collision_shape);

        //add children
        owner.add_child(collider,true);
        owner.set_collision_mask_bit(0,false);
        owner.set_collision_layer_bit(0,false);
        //add signals
        
    }
    #[method]
    fn _process(&mut self,#[base] _owner:TRef<Area>,_delta:f64){
    }
    #[method]
    fn handle_body(&self,#[base] _owner:TRef<Area>,body:Ref<Node,Shared>){
        let body = unsafe{body.assume_safe()};
        let entity_id = body.get_parent().map(|parent|{
            let parent = unsafe{parent.assume_safe()};
            parent.get("id")
        }).unwrap();
        if entity_id.is_nil(){assert!(false)}
        let entity_id = entity_id.try_to::<String>();
        let field_tx = self.field_tx.clone().unwrap();
        let _ = entity_id.map(|id| self.typ.server_body_entered(field_tx,&self.location,id));
    }
    #[method]
    fn place_ability(&mut self,#[base] owner:TRef<Area>,typ:AbilityType){
        let collider = typ.to_collider(Vector3{x:ZONE_WIDTH/2.0,y:ZONE_WIDTH/2.0,z:ZONE_WIDTH/2.0});
        collider.map(|area| {
            let area = unsafe{area.assume_safe()};
            let _ = area.connect("body_entered",owner,"handle_body",VariantArray::new_shared(),0);
        });
        collider.map(|area| owner.add_child(area,true));
        self.abilities = collider;
        self.typ = typ;
    }
    #[method]
    fn remove_ability(&mut self,#[base] owner:TRef<Area>,typ:AbilityType){
        let collider = self.abilities;
        collider.map(|area| {
            let area = unsafe{area.assume_safe()};
            let _ = area.disconnect("body_entered",owner,"handle_body");
            owner.remove_child(area);
            area.queue_free();
        });
        self.abilities = None;
        self.typ = AbilityType::empty;
    }
    fn set_tx(&mut self,tx:Sender<FieldCommand>){
        self.field_tx = Some(tx);
    }
}
pub enum FieldCommand{
    Damage(String,f64),
}

#[derive(NativeClass)]
#[inherit(Spatial)]
#[register_with(Self::register_signals)]
pub struct FieldServer{
    zones:HashMap<Location,Instance<FieldZoneServer>>,
    tx:Sender<FieldCommand>,
    rx:Receiver<FieldCommand>
}
impl Instanced<Spatial> for FieldServer{
    fn make() -> Self{
        let (tx,rx) = mpsc::unbounded_channel::<FieldCommand>();
        FieldServer{
            zones:HashMap::new(),
            tx:tx,
            rx:rx,
        }
    }
}
#[methods]
impl FieldServer{
    fn register_signals(builder:&ClassBuilder<Self>){
        builder
            .signal("damage")
            .with_param("id",VariantType::GodotString)
            .with_param_default("amount",Variant::new(0.0))
            .done();
    }
    #[method]
    fn _ready(&self, #[base] _owner:TRef<Spatial>){
    }
    #[method]
    fn _process(&mut self,#[base] owner:TRef<Spatial>,_delta:f64){
        match self.rx.try_recv(){
            Ok(cmd) => {
                match cmd{
                    FieldCommand::Damage(id,amount) => {
                        owner.emit_signal("damage",&[Variant::new(id),Variant::new(amount)]);
                    }
                }
            }
            Err(_) => {}
        }
    }
    #[method]
    fn get_point_from_location(&self,x:i64,y:i64) -> Vector3{
        Vector3{x:ZONE_WIDTH * (x as f32),y:0.0,z:ZONE_WIDTH * (y as f32)}
    }
    #[method]
    fn add_zone(&mut self,#[base] owner:TRef<Spatial>,location:(i64,i64)){
        let (x,y) = location;
        let location = Location{x:x,y:y};
        if self.zones.contains_key(&location){return ;}
        let zone = FieldZoneServer::make_instance(&location).into_shared();
        self.zones.insert(location,zone.clone());
        let zone = unsafe{zone.assume_safe()};
        let _ = zone.map_mut(|obj,_| obj.set_tx(self.tx.clone()));
        owner.add_child(zone.clone(),true);
        let _ = zone.map(|_,spatial| {
            let mut transform = spatial.transform();
            transform.origin = Vector3{x:ZONE_WIDTH * (location.x as f32),y:0.0,z:ZONE_WIDTH * (location.y as f32)};
            spatial.set_transform(transform);
        });
    }
    #[method]
    fn add_field_ability(&self,#[base] _owner:TRef<Spatial>,ability_id:AbilityType,location:(i64,i64)){
        let (x,y) = location;
        let location = Location{x:x,y:y};
        if !self.zones.contains_key(&location){return ;}
        let zone = self.zones.get(&location).unwrap();
        let zone = unsafe{zone.assume_safe()};
        let _ = zone.map_mut(|obj,body|{
            obj.place_ability(body,ability_id)
        });
    }
    #[method]
    fn remove_field_ability(&self,#[base] _owner:TRef<Spatial>,ability_id:AbilityType){
        for zone in self.zones.values(){
            let zone = unsafe{zone.assume_safe()};
            let _ = zone.map_mut(|obj,body|{
                if obj.typ == ability_id{
                    obj.remove_ability(body,ability_id.into())
                }
            });
        }
    }
}

