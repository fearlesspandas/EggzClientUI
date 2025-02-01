
use std::collections::HashMap;
use gdnative::prelude::*;
use gdnative::api::*;
use crate::traits::{CreateSignal,Instanced,InstancedDefault,Defaulted};
use crate::field_ability_mesh::{FieldAbilityMesh,ToMesh};
use crate::field::{Location,zone_height,zone_width};
use crate::field_abilities::{OpType};
use tokio::sync::mpsc;

type Sender<T> = mpsc::UnboundedSender<T>;
type Receiver<T> = mpsc::UnboundedReceiver<T>;

pub enum FieldZoneCommand{
    Selected(OpType),
}
#[derive(NativeClass)]
#[inherit(Area)]
pub struct FieldZoneServer{
    location:Location,
    abilities:HashMap<OpType,Instance<FieldAbilityMesh>>,
    zone_tx:Sender<FieldZoneCommand>,
    zone_rx:Receiver<FieldZoneCommand>,
    field_tx:Option<Sender<FieldCommand>>,
}
impl InstancedDefault<Area,Location> for FieldZoneServer{
    fn make(args:&Location) -> Self{
        let (tx,rx) = mpsc::unbounded_channel::<FieldZoneCommand>();
        FieldZoneServer{
            location:*args,
            abilities:HashMap::new(),
            zone_tx: tx,
            zone_rx: rx,
            field_tx:None,
        }
    }
}
#[methods]
impl FieldZoneServer{
    #[method]
    fn _ready(&self,#[base] owner:TRef<Area>){
        let cube_size = Vector3{x:zone_width,y:zone_height,z:zone_width}; 
        //initialize
        let collision_shape = BoxShape::new().into_shared();
        let collision_shape = unsafe{collision_shape.assume_safe()};
        collision_shape.set_extents(Vector3{x:zone_width/2.0,y:zone_height/2.0,z:zone_width/2.0});

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
    fn _process(&mut self,#[base] owner:TRef<Area>,delta:f64){
        match self.zone_rx.try_recv() {
            Ok(FieldZoneCommand::Selected(typ)) => {
                self.field_tx
                    .as_ref()
                    .expect("field_tx not set for zone")
                    .send(FieldCommand::AddAbility(self.location,typ));
            }
            Err(_) => {}
        }
    }
    #[method]
    fn place_ability(&mut self,#[base] owner:TRef<Area>,typ:u8){
        let typ = OpType::from(typ);
        if self.abilities.contains_key(&typ){return ;}
        //let mesh = FieldAbilityMesh::make_instance(&typ).into_shared();
        //owner.add_child(mesh.clone(),true);
        //self.abilities.insert(typ,mesh);
    }
    fn set_tx(&mut self,tx:Sender<FieldCommand>){
        self.field_tx = Some(tx);
    }
}
pub enum FieldCommand{
    AddAbility(Location,OpType),
    DoAbility(Location,OpType),
}
impl ToString for FieldCommand{
    fn to_string(&self) -> String{
        match self{
            FieldCommand::AddAbility(_,_) => "add_ability".to_string(),
            FieldCommand::DoAbility(_,_) => "do_ability".to_string(),
        }
    }
}
impl CreateSignal<FieldServer> for FieldCommand{
    fn register(builder:&ClassBuilder<FieldServer>){
        builder
            .signal(&FieldCommand::AddAbility(Location::default(),OpType::empty).to_string())
            .with_param("location",VariantType::VariantArray)
            .with_param("type",VariantType::I64)
            .done();
        builder
            .signal(&FieldCommand::DoAbility(Location::default(),OpType::empty).to_string())
            .with_param("location",VariantType::VariantArray)
            .with_param("type",VariantType::I64)
            .done();
    }
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
        FieldCommand::register(builder);
    }
    #[method]
    fn _ready(&self, #[base] owner:TRef<Spatial>){
    }
    #[method]
    fn _process(&mut self,#[base] owner:TRef<Spatial>,delta:f64){
        match self.rx.try_recv(){
            Ok(cmd) => {
                match cmd{
                    FieldCommand::AddAbility(location,typ) => {
                        godot_print!("Ability Selected!");
                        //todoin
                        let mut loc:Vec<i64> = Vec::new();
                        loc.push(location.x);
                        loc.push(location.y);
                        owner.emit_signal(cmd.to_string(),&[Variant::new(loc),Variant::new(Into::<u8>::into(typ))]);
                    }
                    FieldCommand::DoAbility(location,typ) => {
                        godot_print!("Ability Selected!");
                        //todoin
                        let mut loc:Vec<i64> = Vec::new();
                        loc.push(location.x);
                        loc.push(location.y);
                        owner.emit_signal(cmd.to_string(),&[Variant::new(loc),Variant::new(Into::<u8>::into(typ))]);
                    }
                }
            }
            Err(_) => {}
        }
    }
    #[method]
    fn get_point_from_location(&self,x:i64,y:i64) -> Vector3{
        Vector3{x:zone_width * (x as f32),y:0.0,z:zone_width * (y as f32)}
    }
    #[method]
    fn add_zone(&mut self,#[base] owner:TRef<Spatial>,location:(i64,i64)){
        let (x,y) = location;
        let location = Location{x:x,y:y};
        if self.zones.contains_key(&location){return ;}
        let zone = FieldZoneServer::make_instance(&location).into_shared();
        self.zones.insert(location,zone.clone());
        let zone = unsafe{zone.assume_safe()};
        zone.map_mut(|obj,_| obj.set_tx(self.tx.clone()));
        owner.add_child(zone.clone(),true);
        zone.map(|obj,spatial| {
            let mut transform = spatial.transform();
            transform.origin = Vector3{x:zone_width * (location.x as f32),y:0.0,z:zone_width * (location.y as f32)};
            spatial.set_transform(transform);
        });
        
    }
    #[method]
    fn add_field_ability(&self,#[base] owner:TRef<Spatial>,ability_id:u8,location:(i64,i64)){
        let (x,y) = location;
        let location = Location{x:x,y:y};
        if !self.zones.contains_key(&location){return ;}
        let zone = self.zones.get(&location).unwrap();
        let zone = unsafe{zone.assume_safe()};
        let typ = OpType::from(ability_id);
        zone.map_mut(|obj,body|{
            obj.place_ability(body,ability_id)
        });
    }
}

