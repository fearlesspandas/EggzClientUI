
use gdnative::prelude::*;
use gdnative::api::*;
use gdnative::export::StaticallyNamed;
use crate::traits::{Instanced};
use crate::collision_layer;
use std::collections::{HashSet,HashMap};


type TerrainId = Box<[u8]>;
#[derive(NativeClass)]
#[inherit(Area)]
#[register_with(Self::register_signals)]
pub struct Chunk{
    id:Option<TerrainId>,
    is_empty:bool,
    is_server:bool,
    has_loaded:bool,
    initialized:bool,
    radius:f32,
    terrain_map:HashMap<TerrainId,Vector3>,
    mesh_map:HashMap<TerrainId,MultiMeshInstance>,
    shape:Ref<BoxShape>,
}

impl Instanced<Area> for Chunk{
    fn make() -> Self{
        Chunk{
            id:None,
            is_empty:false,
            is_server:false,
            has_loaded:false,
            initialized:false,
            radius:0.0,
            terrain_map:HashMap::new(),
            mesh_map:HashMap::new(),
            shape:BoxShape::new().into_shared(),
        }
    }
}
#[methods]
impl Chunk{
    fn register_signals(builder:&ClassBuilder<Self>){
        builder
            .signal("fill_empty_terrain")
            .with_param("terrain_uuid",VariantType::GodotString)
            .with_param("entity_id",VariantType::GodotString)
            .done();
        builder
            .signal("load_terrain")
            .with_param("terrain_uuid",VariantType::GodotString)
            .with_param("radius",VariantType::F64)
            .done();
    }
    #[method]
    fn _ready(&self,#[base] owner:TRef<Area>){
        let shape = unsafe{self.shape.assume_safe()};
        let collision_obj = CollisionShape::new().into_shared();
        let collision_obj = unsafe{collision_obj.assume_safe()};
        collision_obj.set_shape(shape);
        owner.add_child(collision_obj,true);

	owner.set_collision_mask_bit(collision_layer::SERVER_TERRAIN_COLLISION_LAYER.into(),false);
	owner.set_collision_layer_bit(collision_layer::SERVER_TERRAIN_COLLISION_LAYER.into(),false);
        let _ = owner.connect("body_entered" , owner, "body_entered",VariantArray::new_shared(),0);
        //owner.set_process(false);
    }

    #[method]
    fn set_initialized(&mut self){
        self.initialized = true;
    }
    #[method]
    fn set_id(&mut self,id:String){
        self.id = Some(id.as_bytes().into());
    }
    #[method]
    fn set_empty(&mut self,value:bool){
        self.is_empty = value;
    }
    #[method]
    fn set_server(&mut self,#[base] owner:TRef<Area>,value:bool){
        self.is_server = value;
        if value{
            owner.set_collision_mask_bit(collision_layer::SERVER_PLAYER_COLLISION_LAYER.into(),true);
        }else{
            owner.set_collision_mask_bit(collision_layer::CLIENT_PLAYER_COLLISION_LAYER.into(),true);
        }

    }
    #[method]
    fn radius(&self) -> f32{
        self.radius
    }
    #[method]
    fn set_radius(&mut self,radius:f32){
        self.radius = radius;
        self.set_extents(Vector3::new(radius,radius,radius));
    }
    #[method]
    fn set_extents(&self,extents:Vector3){
        let shape = unsafe{self.shape.assume_safe()};
        shape.set_extents(extents);
    }
    #[method]
    fn body_entered(&mut self,#[base] owner:TRef<Area>,body:Ref<Node>){
        assert!(self.initialized);
        if self.is_empty && self.is_server{
            let body = unsafe{body.assume_safe()};
            let body = body.cast::<KinematicBody>().expect("not kinematic body colliding with chunk");
            let parent = body.get_parent().map(|parent| {
                let parent = unsafe{parent.assume_safe()};
                parent
            });
            let entity_id = parent.expect("ChunkErr:Could not get correct parent").get("id");
            if entity_id.is_nil(){
                assert!(false,"Body id is null");
            }else{
                let _ = entity_id.try_to::<String>()
                    .map(|id| { self.fill_empty_terrain(owner,id); })
                    .map_err(|_err| assert!(false,"Incorrect type for id"));
            }
        }else{
            self.load_terrain(owner);
        }
    }
    #[method]
    fn fill_empty_terrain(&mut self,#[base] owner:TRef<Area>,entity_id:String){
        assert!(self.initialized);
        godot_print!("{}",format!("filled with entity_id: {entity_id:?}"));
        if self.is_empty && !self.has_loaded{
            owner.emit_signal("fill_empty_terrain",
                &[
                    Variant::new(std::str::from_utf8(self.id.as_ref().expect("Terrain UUID not set")).unwrap()),
                    Variant::new(entity_id)
                ]
            );
            self.has_loaded = true;
        }
    }
    #[method]
    fn load_terrain(&mut self,#[base] owner:TRef<Area>){
        assert!(self.initialized);
        //if self.is_empty || self.has_loaded{return ;}
        assert!(self.radius > 0.0, "Chunk radius is zero on terrain load request");
        if !self.has_loaded && !self.is_empty{
            self.has_loaded = true;
            owner.emit_signal("load_terrain",
                &[
                    Variant::new(std::str::from_utf8(self.id.as_ref().expect("Terrain UUID not set")).unwrap()),
                    Variant::new(self.radius),
                ]
            );
            let radius = self.radius;
            //godot_print!("{}",format!("Loaded with radius: {radius:?}"));
        }
    }
    #[method]
    fn is_within_chunk(&self,#[base] owner:TRef<Area>,loc:Vector3) -> bool{
        let origin = owner.global_transform().origin;
        let x_diff = (loc.x - origin.x).abs();
        let y_diff = (loc.y - origin.y).abs();
        let z_diff = (loc.z - origin.z).abs();
        x_diff <= self.radius && y_diff <= self.radius && z_diff <= self.radius
    }
    #[method]
    fn is_within_distance(&self,#[base] owner:TRef<Area>, loc:Vector3,distance:f32) -> bool{
        let diffs = loc - owner.global_transform().origin; 
        let dist = distance + self.radius;
        diffs.x.abs() <= dist && diffs.y.abs() <= dist && diffs.z.abs() <= dist
        //godot_print!("{}",format!("Is within distance: {loc:?} within {dist:?} of {owner_origin:?}"));
    }
    #[method]
    fn check_load(&mut self,#[base] owner:TRef<Area>){
        let entities = owner.get_overlapping_bodies();
        if entities.into_iter().collect::<Vec<_>>().len() > 0{
            self.load_terrain(owner);
        }
    }
}
