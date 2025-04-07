use gdnative::prelude::*;
use gdnative::api::*;
use crate::traits::{Instanced};
use crate::collision_layer;
use std::collections::{HashSet,HashMap};


#[derive(NativeClass)]
#[inherit(Area)]
#[register_with(Self::register_signals)]
pub struct CollisionBox{
    radius:f32,
    tracked_bodies:HashMap<String,Ref<KinematicBody>>,
    tracked_areas:HashSet<Ref<Area>>,
    affected_shape:Ref<SphereShape>,
}
impl Instanced<Area> for CollisionBox{
    fn make() -> Self{
        CollisionBox{
            radius:0.0,
            tracked_bodies:HashMap::new(),
            tracked_areas:HashSet::new(),
            affected_shape:SphereShape::new().into_shared(),
        }
    }
}
#[methods]
impl CollisionBox{
    fn register_signals(builder:&ClassBuilder<Self>){
        let _ = builder
            .signal("slow_to")
            .with_param("id",VariantType::GodotString)
            .with_param("speed",VariantType::F64);
    }
    #[method]
    fn _ready(&self,#[base] owner:TRef<Area>){
        let affected_shape = unsafe{self.affected_shape.assume_safe()};

        let affected_collision_obj = CollisionShape::new().into_shared();
        let affected_collision_obj = unsafe{affected_collision_obj.assume_safe()};
        affected_collision_obj.set_shape(affected_shape);
        owner.add_child(affected_collision_obj,true);

        owner.set_collision_layer_bit(collision_layer::SERVER_TERRAIN_COLLISION_LAYER.into(),false);
        owner.set_collision_mask_bit(collision_layer::SERVER_TERRAIN_COLLISION_LAYER.into(),false);

        owner.set_collision_mask_bit(collision_layer::SERVER_PLAYER_COLLISION_LAYER.into(),true);

        owner.set_collision_mask_bit(collision_layer::SERVER_AREA_COLLISION_LAYER.into(),true);
        owner.set_collision_layer_bit(collision_layer::SERVER_AREA_COLLISION_LAYER.into(),true);

        let _ = owner.connect("body_entered",owner,"start_tracking",VariantArray::new_shared(),0);
        let _ = owner.connect("body_exited",owner,"stop_tracking",VariantArray::new_shared(),0);
        let _ = owner.connect("area_entered",owner,"start_tracking_area",VariantArray::new_shared(),0);
        let _ = owner.connect("area_exited",owner,"stop_tracking_area",VariantArray::new_shared(),0);
    }
    #[method]
    fn _physics_process(&mut self,#[base] owner:TRef<Area>,delta:f64){
        for (id,body) in &self.tracked_bodies{
            let body = unsafe{body.assume_safe()};
            let owner_origin = owner.global_transform().origin;
            let body_origin = body.global_transform().origin;
            let diff_vec = body_origin - owner_origin;
            let new_loc = owner_origin +  self.radius * (diff_vec.normalized());
            let trans = new_loc - body_origin;
            body.translate(trans);
            //let post_set = body.global_transform().origin;
            //let radius = self.radius;
            //godot_print!("{}",format!("Collision Handled:{id:?},radius:{radius:?} self:{owner_origin:?},body:{body_origin:?}, diff:{diff_vec:?}, new_loc:{new_loc:?},set_loc:{post_set:?}"));
        }
        for area in &self.tracked_areas{
            let area = unsafe{area.assume_safe()};
            if !area.has_method("radius"){assert!(false,"No radius");}
            if !area.has_method("body"){assert!(false,"No body");}
            let radius = unsafe{area.call("radius",&[]).try_to::<f32>().expect("Body without radius entered collision")};
            let movement_body = unsafe{area.call("body",&[]).to_object::<KinematicBody>().expect("Area without movement body entered collision")};
            let movement_body = unsafe{movement_body.assume_safe()};
            let owner_origin = owner.global_transform().origin;
            let area_origin = area.global_transform().origin;
            let diff_vec = area_origin - owner_origin;
            let new_loc = owner_origin +  (self.radius + radius) * (diff_vec.normalized());
            let trans = new_loc - area_origin;
            movement_body.translate(trans);
            //let post_set = body.global_transform().origin;
            //let radius = self.radius;
            //godot_print!("{}",format!("Collision Handled:{id:?},radius:{radius:?} self:{owner_origin:?},body:{body_origin:?}, diff:{diff_vec:?}, new_loc:{new_loc:?},set_loc:{post_set:?}"));
        }
    }
    #[method]
    fn body(&self,#[base] owner:TRef<Area>)->Ref<KinematicBody>{
        owner.get_parent().map(|parent| {
            let parent = unsafe{parent.assume_safe()};
            parent.cast::<KinematicBody>().expect("Parent is not KinematicBody").claim()
        })
        .expect("Parent not found for collision area")
    }
    #[method]
    fn radius(&self) -> f32{
        self.radius
    }
    #[method]
    fn set_radius(&mut self,radius:f32){
        self.radius = radius;
        let affected_shape = unsafe{self.affected_shape.assume_safe()};
        affected_shape.set_radius(radius.into());
    }
    #[method]
    fn start_tracking(&mut self,#[base] owner:TRef<Area>,body:Ref<Node,Shared>){
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
                .map(|id| {
                    if self.tracked_bodies.contains_key(&id){return ;}
                    //let owner_origin = owner.global_transform().origin;
                    //let body_origin = body.global_transform().origin;
                    //let diff_vec = body_origin - owner_origin;
                    //let new_loc = owner_origin +  self.radius * (diff_vec.normalized());
                    //let radius = self.radius;
                    //godot_print!("{}",format!("Collision Entered:{id:?},radius:{radius:?} self:{owner_origin:?},body:{body_origin:?}, diff:{diff_vec:?}, new_loc:{new_loc:?}"));
                    self.tracked_bodies.insert(id.clone(),body.claim());
                })
                .map_err(|_err| assert!(false,"Incorrect type for id"));
        }
    }
    #[method]
    fn stop_tracking(&mut self,#[base] owner:TRef<Area>,body:Ref<Node,Shared>){
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
                .map(|id| {
                    self.tracked_bodies.remove(&id);
                })
                .map_err(|_err| assert!(false,"Incorrect type for id"));
        }
    }
    #[method]
    fn start_tracking_area(&mut self,#[base] owner:TRef<Area>,body:Ref<Area,Shared>){
        self.tracked_areas.insert(body);
    }
    #[method]
    fn stop_tracking_area(&mut self,#[base] owner:TRef<Area>,body:Ref<Area,Shared>){
        self.tracked_areas.remove(&body);
    }
}
