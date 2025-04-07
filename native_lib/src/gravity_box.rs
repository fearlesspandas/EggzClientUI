
use gdnative::prelude::*;
use gdnative::api::*;
use crate::traits::{Instanced};
use crate::collision_layer;
use std::collections::{HashSet,HashMap};

#[derive(NativeClass)]
#[inherit(Spatial)]
#[register_with(Self::register_signals)]
pub struct GravityBox{
    terrain_id:Option<String>,
    mass:f64,
    tracked_bodies:HashMap<String,Ref<KinematicBody>>,
    untracked_bodies:HashSet<String>,
    captured_bodies:HashSet<String>,
    affected_area:Ref<Area>,
    affected_shape:Ref<SphereShape>,
    unaffected_area:Ref<Area>,
    unaffected_shape:Ref<SphereShape>,
    unaffected_radius:f32,
    timer:Ref<Timer>,
}
impl Instanced<Spatial> for GravityBox{
    fn make() -> Self{
        GravityBox{
            terrain_id:None,
            mass:0.0,
            tracked_bodies:HashMap::new(),
            untracked_bodies:HashSet::new(),
            captured_bodies:HashSet::new(),
            affected_area:Area::new().into_shared(),
            affected_shape:SphereShape::new().into_shared(),
            unaffected_area:Area::new().into_shared(),
            unaffected_shape:SphereShape::new().into_shared(),
            unaffected_radius:0.0,
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
            .done();
        builder
            .signal("entered_affected")
            .with_param("entity_id",VariantType::GodotString)
            .done();
        builder
            .signal("exited_affected")
            .with_param("entity_id",VariantType::GodotString)
            .done();
        builder
            .signal("entered_unaffected")
            .with_param("entity_id",VariantType::GodotString)
            .with_param("terrain_id",VariantType::GodotString)
            .done();
        builder
            .signal("exited_unaffected")
            .with_param("entity_id",VariantType::GodotString)
            .with_param("terrain_id",VariantType::GodotString)
            .done();
    }
    #[method]
    fn _ready(&self,#[base] owner:TRef<Spatial>){
        let affected_area = unsafe{self.affected_area.assume_safe()};
        let affected_shape = unsafe{self.affected_shape.assume_safe()};
        let unaffected_area = unsafe{self.unaffected_area.assume_safe()};
        let unaffected_shape = unsafe{self.unaffected_shape.assume_safe()};
        let timer = unsafe{self.timer.assume_safe()};

        let affected_collision_obj = CollisionShape::new().into_shared();
        let affected_collision_obj = unsafe{affected_collision_obj.assume_safe()};
        affected_collision_obj.set_shape(affected_shape);
        affected_area.add_child(affected_collision_obj,true);

        affected_area.set_collision_layer_bit(collision_layer::SERVER_TERRAIN_COLLISION_LAYER.into(),false);
        affected_area.set_collision_mask_bit(collision_layer::SERVER_TERRAIN_COLLISION_LAYER.into(),false);
        affected_area.set_collision_mask_bit(collision_layer::SERVER_PLAYER_COLLISION_LAYER.into(),true);
        affected_area.set_collision_mask_bit(collision_layer::SERVER_GRAVITY_COLLISION_LAYER.into(),true);

        let _ = affected_area.connect("body_entered",owner,"start_tracking",VariantArray::new_shared(),0);
        let _ = affected_area.connect("body_exited",owner,"stop_tracking",VariantArray::new_shared(),0);

        let unaffected_collision_obj = CollisionShape::new().into_shared();
        let unaffected_collision_obj = unsafe{unaffected_collision_obj.assume_safe()};
        unaffected_collision_obj.set_shape(unaffected_shape);
        unaffected_area.add_child(unaffected_collision_obj,true);

        unaffected_area.set_collision_layer_bit(collision_layer::SERVER_TERRAIN_COLLISION_LAYER.into(),false);
        unaffected_area.set_collision_mask_bit(collision_layer::SERVER_TERRAIN_COLLISION_LAYER.into(),false);
        unaffected_area.set_collision_mask_bit(collision_layer::SERVER_PLAYER_COLLISION_LAYER.into(),true);
        unaffected_area.set_collision_mask_bit(collision_layer::SERVER_GRAVITY_COLLISION_LAYER.into(),true);

        let _ = unaffected_area.connect("body_entered",owner,"add_unaffected",VariantArray::new_shared(),0);
        let _ = unaffected_area.connect("body_exited",owner,"remove_unaffected",VariantArray::new_shared(),0);

        owner.add_child(unaffected_area,true);
        owner.add_child(affected_area,true);
        let _ = timer.connect("timeout",owner,"add_gravity_to_tracked",VariantArray::new_shared(),0);
        owner.add_child(timer,true);
        timer.start(0.05);
    }
    #[method]
    fn set_affected_radius(&self,radius:f64){
        let affected_shape = unsafe{self.affected_shape.assume_safe()};
        affected_shape.set_radius(radius);
    }
    #[method]
    fn set_unaffected_radius(&mut self,radius:f64){
        let unaffected_shape = unsafe{self.unaffected_shape.assume_safe()};
        unaffected_shape.set_radius(radius);
        self.unaffected_radius = radius as f32;
    }
    #[method]
    fn set_id(&mut self,id:String){
        self.terrain_id = Some(id);
    }
    #[method]
    fn set_mass(&mut self,value:f64){
        self.mass = value;
    }
    #[method]
    fn add_gravity_to_tracked(&self,#[base] owner:TRef<Spatial>){
        for (id,body) in &self.tracked_bodies{
            if !&self.untracked_bodies.contains(id) && !&self.captured_bodies.contains(id){
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
    }
    #[method]
    fn start_tracking(&mut self,#[base] owner:TRef<Spatial>,body:Ref<Node,Shared>){
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
                    godot_print!("{}",format!("Body tracked:{id:?}"));
                    self.tracked_bodies.insert(id.clone(),body.claim());
                    owner.emit_signal( "entered_affected", &[ Variant::new(id) ]); 
                })
                .map_err(|_err| assert!(false,"Incorrect type for id"));
        }
    }
    #[method]
    fn stop_tracking(&mut self,#[base] owner:TRef<Spatial>,body:Ref<Node,Shared>){
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
                    owner.emit_signal( "exited_affected", &[ Variant::new(id) ]);
                })
                .map_err(|_err| assert!(false,"Incorrect type for id"));
        }
    }
    #[method]
    fn add_unaffected(&mut self,#[base] owner:TRef<Spatial>,body:Ref<Node,Shared>){
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
                    self.untracked_bodies.insert(id.clone());
                    owner.emit_signal( "entered_unaffected", &[ Variant::new(id),Variant::new(self.terrain_id.clone().unwrap()) ]);
                })
                .map_err(|_err| assert!(false,"Incorrect type for id"));
        }
    }
    #[method]
    fn remove_unaffected(&mut self,#[base] owner:TRef<Spatial>,body:Ref<Node,Shared>){
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
                    self.untracked_bodies.remove(&id);
                    owner.emit_signal( "exited_unaffected", &[ Variant::new(id),Variant::new(self.terrain_id.clone().unwrap()) ]); 
                })
                .map_err(|_err| assert!(false,"Incorrect type for id"));
        }
    }
    #[method]
    fn add_captured(&mut self,id:String){
        self.captured_bodies.insert(id);
    }
    #[method]
    fn remove_captured(&mut self,id:String){
        self.captured_bodies.remove(&id);
    }
}
