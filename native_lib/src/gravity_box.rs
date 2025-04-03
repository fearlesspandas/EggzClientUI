
use gdnative::prelude::*;
use gdnative::api::*;
use crate::traits::{Instanced};
use crate::collision_layer;
use std::collections::{HashSet,HashMap};


#[derive(NativeClass)]
#[inherit(Area)]
#[register_with(Self::register_signals)]
pub struct GravityBox{
    terrain_id:Option<String>,
    mass:f64,
    tracked_bodies:HashMap<String,Ref<KinematicBody>>,
    collision:Ref<CollisionShape>,
    shape:Ref<BoxShape>,
    timer:Ref<Timer>,
}
impl Instanced<Area> for GravityBox{
    fn make() -> Self{
        GravityBox{
            terrain_id:None,
            mass:0.0,
            tracked_bodies:HashMap::new(),
            collision:CollisionShape::new().into_shared(),
            shape:BoxShape::new().into_shared(),
            timer:Timer::new().into_shared(),
        }
    }
}
#[methods]
impl GravityBox{
    fn register_signals(builder:&ClassBuilder<Self>){
        builder
            .signal("apply_gravity")
            .with_param("terrain_id",VariantType::GodotString)
            .with_param("entity_id",VariantType::GodotString)
            .with_param("vec",VariantType::Float32Array)
            .done()
    }
    #[method]
    fn _ready(&self,#[base] owner:TRef<Area>){
        let collision_obj = unsafe{self.collision.assume_safe()};
        let shape = unsafe{self.shape.assume_safe()};
        let timer = unsafe{self.timer.assume_safe()};

        collision_obj.set_shape(shape);
        owner.add_child(collision_obj,true);
        owner.set_collision_layer_bit(collision_layer::SERVER_TERRAIN_COLLISION_LAYER.into(),false);
        owner.set_collision_mask_bit(collision_layer::SERVER_TERRAIN_COLLISION_LAYER.into(),false);
        owner.set_collision_mask_bit(collision_layer::SERVER_PLAYER_COLLISION_LAYER.into(),true);
        let _ = owner.connect("body_entered",owner,"start_tracking",VariantArray::new_shared(),0);
        let _ = owner.connect("body_exited",owner,"stop_tracking",VariantArray::new_shared(),0);
        let _ = timer.connect("timeout",owner,"add_gravity_to_tracked",VariantArray::new_shared(),0);
        owner.add_child(timer,true);
        timer.start(0.05);
    }
    #[method]
    fn set_extents(&self,extents:Vector3){
        let shape = unsafe{self.shape.assume_safe()};
        shape.set_extents(extents);
    }
    #[method]
    fn set_id(&mut self,id:String){
        self.terrain_id = Some(id);
    }
    #[method]
    fn center_at(&self,#[base] owner:TRef<Area>,location:Vector3){
        let shape = unsafe{self.shape.assume_safe()};
        let extents = shape.extents(); 
        let mut transform = owner.global_transform();
        transform.origin = location;
        owner.set_global_transform(transform);
    }
    #[method]
    fn set_mass(&mut self,value:f64){
        self.mass = value;
    }
    #[method]
    fn add_gravity_to_tracked(&self,#[base] owner:TRef<Area>){
        for (id,body) in &self.tracked_bodies{
            let body = unsafe{body.assume_safe()};
            let vec =  owner.global_translation() - body.global_translation();
            owner.emit_signal(
                "apply_gravity",
                &[
                    Variant::new(self.terrain_id.as_ref().expect("TerrainId not set")),
                    Variant::new(id),
                    Variant::new(Vec::from([vec.x,vec.y,vec.z]))
                ]
            );
        }
    }
    #[method]
    fn start_tracking(&mut self,body:Ref<Node,Shared>){
        let body = unsafe{body.assume_safe()};
        let body = body.cast::<KinematicBody>().expect("GravityBoxErr:Entered Body is not Kinematic Body");
        let parent = body.get_parent().map(|parent| {
            let parent = unsafe{parent.assume_safe()};
            parent.cast::<Spatial>().expect("GravityBoxErr:Parent is not spatial")
        });
        let entity_id = parent.expect("GravityBoxErr:Could not get correct parent").get("id");
        if entity_id.is_nil(){
            assert!(false,"Body id is null");
        }else{
            let _ = entity_id.try_to::<String>()
                .map(|id| self.tracked_bodies.insert(id,body.claim()))
                .map_err(|_err| assert!(false,"Incorrect type for id"));
        }
    }
    #[method]
    fn stop_tracking(&mut self,body:Ref<Node,Shared>){
        let body = unsafe{body.assume_safe()};
        let parent = body.get_parent().map(|parent| {
            let parent = unsafe{parent.assume_safe()};
            parent.cast::<Spatial>().expect("GravityBoxErr:Parent is not spatial")
        });
        let entity_id = parent.expect("GravityBoxErr:Could not get correct parent").get("id");
        if entity_id.is_nil(){
            assert!(false,"Body id is null");
        }else{
            let _ = entity_id.try_to::<String>()
                .map(|id| self.tracked_bodies.remove(&id))
                .map_err(|_err| assert!(false,"Incorrect type for id"));
        }
    }
}
