
use gdnative::prelude::*;
use gdnative::api::*;
use gdnative::export::StaticallyNamed;
use crate::traits::{Instanced};
use crate::collision_layer;
use crate::assets::{Assets};
use std::collections::{HashSet,HashMap};


type TerrainId = Box<[u8]>;
type TerrainKey = i64;

#[derive(Hash,Eq,PartialEq,Clone)]
pub struct Location{
    x:u32,
    y:u32,
    z:u32
}
impl Location{
    fn new(x:f32,y:f32,z:f32) -> Self {
        Location{x:x.to_bits(),y:y.to_bits(),z:z.to_bits()}
    }
}
impl From<Vector3> for Location{
    fn from(vec:Vector3) -> Self{
        Location{x:vec.x.to_bits(),y:vec.y.to_bits(),z:vec.z.to_bits()}
    }
}
#[derive(NativeClass)]
#[inherit(MultiMesh)]
pub struct ChunkMesh{
    is_point_mesh:bool,
    terrain_type:Option<TerrainKey>,
    locations:Vec<Vector3>,
    mesh:Option<Ref<Mesh>>,
    point_mesh:Option<Ref<Mesh>>,
}
const TOTAL_TERRAIN_TYPES:usize = 128;
const MINIMUM_TERRAIN_IN_CHUNKS:usize = 512;
impl Instanced<MultiMesh> for ChunkMesh{
    fn make() -> Self{
        //256 is currently more than the number of meshes we have; size accordingly
        //let mut terrain = Vec::with_capacity(TOTAL_TERRAIN_TYPES);
        //for _ in 0..terrain.capacity(){
        //    terrain.push(HashSet::with_capacity(MINIMUM_TERRAIN_IN_CHUNKS));
        //}
        ChunkMesh{
            is_point_mesh:false,
            terrain_type:None,
            locations:Vec::with_capacity(MINIMUM_TERRAIN_IN_CHUNKS),
            mesh:None,
            point_mesh:None,
        }
    }
}
#[methods]
impl ChunkMesh{
    #[method]
    fn add_terrain(&mut self,location:Vector3){
        self.locations.push(location);
    }
    #[method]
    fn set_terrain_type(&mut self,terrain_type:TerrainKey){
        self.terrain_type = Some(terrain_type.clone());
        let terrain_type = Assets::from(terrain_type);
        let mesh = terrain_type.to_mesh_resource().expect("ChunkMeshErr:Resource not found for type");
        let point_mesh = terrain_type.to_point_mesh_resource().expect("ChunkMeshErr:Resource not found for type");
        self.mesh = Some(mesh);
        self.point_mesh = Some(point_mesh);
    }
    #[method]
    fn move_mesh_locations(&self,#[base] owner:TRef<MultiMesh>){
        let terrain_locations = &self.locations;
        for i in 0..owner.instance_count(){
            let idx:usize = i.try_into().expect("ChunkMeshErr:Failed to Index locations array");
            let loc = terrain_locations[idx];
            let (x,y,z) = (loc.x,loc.y,loc.z);
            let mut transform = owner.get_instance_transform(i);
            transform.origin = Vector3::new(x,y,z);
            owner.set_instance_transform(i,transform);
        }
    }
    #[method]
    fn bake_at(&self,#[base] owner:TRef<MultiMesh>){
        let mesh = self.mesh.clone().expect("ChunkMeshErr:bake called without mesh set; set terrain_type using set_terrain_type");
        let point_mesh = self.point_mesh.clone().expect("ChunkMeshErr:bake called without mesh set; set terrain_type using set_terrain_type");
        owner.set_instance_count(0);
        owner.set_transform_format(MultiMesh::TRANSFORM_3D);
        owner.set_instance_count(self.locations.len().try_into().expect("ChunkMeshErr:Inappropriate length for terrain"));
        self.move_mesh_locations(owner);
        if self.is_point_mesh{
            owner.set_mesh(point_mesh);
        }else{
            owner.set_mesh(mesh);
        }
    }
    //must be used BEFORE bake or commit to take affect
    #[method]
    fn set_point_mesh(&mut self,is_point_mesh:bool){
        self.is_point_mesh = is_point_mesh;
    }
    #[method]
    fn commit_point_mesh(&self,#[base] owner:TRef<MultiMesh>){
        if self.is_point_mesh{
            owner.set_mesh(self.point_mesh.clone().expect("ChunkMeshErr:Point Mesh Not found"));
        }else{
            owner.set_mesh(self.mesh.clone().expect("ChunkMeshErr:Mesh Not found"));
        }
    }
}
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
    mesh_map:HashMap<TerrainKey,Instance<ChunkMesh>>,
    terrain_map:HashMap<String,(TerrainKey,Vector3)>,
    instance_map:HashMap<TerrainKey,Ref<MultiMeshInstance>>,
    //terrain that has already been sent to the waypoints mesh handler
    sent_terrain:HashSet<String>,
    shape:Ref<BoxShape>,
    sent_all_terrain:bool,
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
            mesh_map:HashMap::with_capacity(8),
            terrain_map:HashMap::with_capacity(MINIMUM_TERRAIN_IN_CHUNKS),
            instance_map:HashMap::with_capacity(8),
            sent_terrain:HashSet::with_capacity(MINIMUM_TERRAIN_IN_CHUNKS),
            shape:BoxShape::new().into_shared(),
            sent_all_terrain:true,
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
        builder
            .signal("location")
            .with_param("region_id",VariantType::GodotString)
            .with_param("terrain_type",VariantType::I64)
            .with_param("location",VariantType::Vector3)
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
    fn add_terrain_mesh(&mut self,#[base] owner:TRef<Area>,terrain_id:String,terrain_type:TerrainKey,location:Vector3){
        if self.terrain_map.contains_key(&terrain_id){return ;}
        self.sent_all_terrain = false;
        self.terrain_map.insert(terrain_id,(terrain_type.clone(),location.clone()));

        let location = owner.to_local(location);
        let mesh_for_type = self.mesh_map.get(&terrain_type).map_or_else(||{
            let id = self.id.clone();
            let mesh = ChunkMesh::make_instance().into_shared();

            let mesh_obj = unsafe{mesh.assume_safe()};
            let _ = mesh_obj.map_mut(|obj,_| {
                obj.set_terrain_type(terrain_type);
            });
            mesh
        },
        |x|{
            x.clone()
        });
        let mesh_for_type = unsafe{mesh_for_type.assume_safe()};
        let _ = mesh_for_type.map_mut(|obj,_|obj.add_terrain(location));
        self.mesh_map.insert(terrain_type,mesh_for_type.claim());
    }
    #[method]
    fn send_locations(&self,#[base] owner:TRef<Area>) -> (String,Vec<(String,TerrainKey,Vector3)>){
        let id = self.id.as_ref().map(|id|std::str::from_utf8(id).expect("uhoh2")).clone().expect("Uhoh");
        if self.terrain_map.len() == self.sent_terrain.len(){
            return (id.to_string(),Vec::new())
        }
        let v = self.terrain_map
            .clone()
            .into_iter()
            .filter(|(k,_)| !self.sent_terrain.contains(k)).map(|(k,(tk,loc))|{
            (k,tk,loc)
        }).collect::<Vec<(String,TerrainKey,Vector3)>>(); 
        let num = v.len();
        godot_print!("{}",format!("Sending {num:?} locations from chunk:{id:?}"));
        (id.to_string(),v)
    }
    #[method]
    fn update_cached_send(&mut self,#[base] owner:TRef<Area>){
        for terrain_id in self.terrain_map.keys(){
            self.sent_terrain.insert(terrain_id.clone());
        }
        self.sent_all_terrain = true;
    }
    #[method]
    fn bake(&mut self,#[base] owner:TRef<Area>){
        for (terrain_type,mesh) in &self.mesh_map{
            let mesh = unsafe{mesh.assume_safe()};
            let mesh_instance = MultiMeshInstance::new().into_shared();
            self.instance_map.insert(terrain_type.clone(),mesh_instance);
            let mesh_instance = unsafe{mesh_instance.assume_safe()};
            let _ = mesh.map(|_,msh| {
                mesh_instance.set_multimesh(msh);
            });
            owner.add_child(mesh_instance,true);
            let _ = mesh.map(|obj,msh| obj.bake_at(msh));
            //let origin = owner.global_transform().origin;
            //let mesh_origin = mesh_instance.global_transform().origin;
            //godot_print!("{}",format!("Baking at {origin:?}"));
            //godot_print!("{}",format!("Mesh at {mesh_origin:?}"));

            //let _ = mesh.map(|obj,msh|{
            //    for i in 0..msh.instance_count(){
            //        let instance_origin = owner.to_global(msh.get_instance_transform(i).origin);
            //        godot_print!("{}",format!("MeshInstance Origin: {instance_origin:?}"));
            //    }
            //});
        }
    }
    #[method]
    fn set_point_mesh(&self,#[base] owner:TRef<Area>,is_point_mesh:bool){
        for mesh in self.mesh_map.values(){
            let mesh = unsafe{mesh.assume_safe()};
            let _ = mesh.map_mut(|obj,_| obj.set_point_mesh(is_point_mesh));
        }
    }
    #[method]
    fn commit_point_mesh(&self,#[base] owner:TRef<Area>){
        for mesh in self.mesh_map.values(){
            let mesh = unsafe{mesh.assume_safe()};
            let _ = mesh.map(|obj,msh| obj.commit_point_mesh(msh));
        }
    }
    #[method]
    fn set_initialized(&mut self){
        self.initialized = true;
    }
    #[method]
    fn id(&self) -> String{
        self
            .id
            .as_ref()
            .map(|val| std::str::from_utf8(val).expect("ChunkErr:could not format chunk id").to_string())
            .expect("ChunkErr:No id found for chunk")
    }
    #[method]
    fn sent_all(&self) -> bool{
        self.sent_all_terrain
    }
    #[method]
    fn set_hidden(&self,value:bool){
        for mesh_instance in self.instance_map.values(){
            let mesh_instance = unsafe{mesh_instance.assume_safe()};
            mesh_instance.set_visible(!value);
        }
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
    fn server(&self) -> bool{
        self.is_server
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
