use gdnative::prelude::*;
use gdnative::api::*;
use gdnative::export::StaticallyNamed;
use crate::traits::{Instanced};
use crate::collision_layer;
use crate::assets::{Assets};
use std::collections::{HashSet,HashMap};
use tokio::{
    runtime::Runtime,
    sync::mpsc,
    time::Duration,
    net::TcpStream,
    io::{
        BufReader,
        ReadHalf,
    },
};
type Sender<T> = mpsc::Sender<T>;
type Receiver<T> = mpsc::Receiver<T>;
type TerrainKey = i64;
type ChunkId = String;

#[derive(NativeClass)]
#[inherit(MultiMesh)]
pub struct NavigationPoints{
    terrain_type:Option<TerrainKey>,
    locations:Vec<Vector3>,
    terrain_ids:HashSet<TerrainId>,
    mesh:Option<Ref<Mesh>>,
}
const MINIMUM_TERRAIN_IN_CHUNKS:usize = 512;
impl Instanced<MultiMesh> for NavigationPoints{
    fn make() -> Self{
        NavigationPoints{
            terrain_type:None,
            //locations are local to MultiMeshInstance
            locations:Vec::with_capacity(MINIMUM_TERRAIN_IN_CHUNKS),
            terrain_ids:HashSet::with_capacity(256),
            mesh:None,
        }
    }
}
#[methods]
impl NavigationPoints{
    #[method]
    fn add_terrain(&mut self,terrain_id:TerrainId,location:Vector3){
        if self.terrain_ids.contains(&terrain_id){return ;}
        self.locations.push(location);
        self.terrain_ids.insert(terrain_id);
    }
    #[method]
    fn clear_terrain(&mut self){
        self.terrain_ids.clear();
    }
    fn remove_terrain(&mut self,terrain_ids:&HashSet<TerrainId>){
        for id in terrain_ids{
            self.terrain_ids.remove(id);
        }
    }
    #[method]
    fn set_terrain_type(&mut self,terrain_type:TerrainKey){
        self.terrain_type = Some(terrain_type.clone());
        let terrain_type = Assets::from(terrain_type);
        let mesh = terrain_type.to_point_mesh_resource().expect("NavigationMeshErr:Resource not found for type");
        self.mesh = Some(mesh);
    }
    fn set_mesh_locations(&self,owner:TRef<MultiMesh>,terrain_locations:&Vec<Vector3>){
        for i in 0..owner.instance_count(){
            let idx:usize = i.try_into().expect("NavigationMeshErr:Failed to Index locations array");
            let loc = terrain_locations[idx];
            let (x,y,z) = (loc.x,loc.y,loc.z);
            let mut transform = owner.get_instance_transform(i);
            transform.origin = Vector3::new(x,y,z);
            owner.set_instance_transform(i,transform);
        }
    }
    #[method]
    fn bake_at(&self,#[base] owner:TRef<MultiMesh>){
        let mesh = self.mesh.clone().expect("NavigationMeshErr:bake called without mesh set; set terrain_type using set_terrain_type");
        owner.set_instance_count(0);
        owner.set_transform_format(MultiMesh::TRANSFORM_3D);
        owner.set_instance_count(self.terrain_ids.len().try_into().expect("NavigationMeshErr:Inappropriate length for terrain"));
        self.set_mesh_locations(owner,&self.locations);
        owner.set_mesh(mesh);
    }
}
enum Command{
    PlayerReposition(TerrainKey)
}
#[derive(NativeClass)]
#[inherit(Spatial)]
pub struct ChunkNavMesh{
    id:Option<ChunkId>,
    mesh_map:HashMap<TerrainKey,Instance<NavigationPoints>>,
    instance_map:HashMap<TerrainKey,Ref<MultiMeshInstance>>,
    all_mesh:Vec<Instance<NavigationPoints>>,
    locations:HashMap<TerrainId,Vector3>,
    hidden:bool,
    movement_lock:bool,
    center:Vector3,
    render_distance:f32,
    tick:i64,
}
impl Instanced<Spatial> for ChunkNavMesh{
    fn make() -> Self{
        ChunkNavMesh{
            id:None,
            mesh_map:HashMap::with_capacity(64),
            instance_map:HashMap::with_capacity(64),
            all_mesh:Vec::with_capacity(64),
            locations:HashMap::with_capacity(2048 * 8),
            hidden:false,
            movement_lock:false,
            center:Vector3::ZERO,
            render_distance:0.0,
            tick:0,
        }
    }
}
fn size_vector(location:&Vector3) -> Vector3{
    let length = location.length();
    let is_too_long:f32 = (length > MAX_DISTANCE ).into();
    let is_not_too_long:f32 = (length <= MAX_DISTANCE ).into();
    let distance = MAX_DISTANCE * is_too_long + length * is_not_too_long;
    let location = location.normalized() * distance;
    location
}
const CHUNK_NAV_DISTANCE:f32 = 1024.0;
#[methods]
impl ChunkNavMesh{
    #[method]
    fn _physics_process(&mut self,#[base] owner:TRef<Spatial>,delta:f32){
        if self.hidden || self.movement_lock{return ;}
        let parent = owner
            .get_parent().expect("could not retrieve parent");
        let parent = unsafe{parent.assume_safe()};
        let parent_location = parent
            .cast::<Spatial>().expect("Could not cast parent to Spatial")
            .global_transform().origin;

        assert!(parent_location != Vector3::ZERO,"Zero vector set for location");

        let diff_vec = (self.center - parent_location).normalized() * CHUNK_NAV_DISTANCE;

        let mut transform = owner.global_transform();
        transform.origin = diff_vec;
        owner.set_transform(transform);

        let render_distance = self.render_distance;
        let diff_length_sq = diff_vec.length_squared();
        if  diff_length_sq > render_distance * render_distance{
            self.set_hidden(owner,false);
        }
        if diff_length_sq > 4.0 * render_distance * render_distance{
            self.set_hidden(owner,true);
        }
        if self.tick == 3000{
            self.adjust_terrain_to_locations(owner);
            self.tick = 0;
        }
        self.tick += 1;
    }
    #[method]
    fn adjust_terrain_to_locations(&self,#[base] owner:TRef<Spatial>){
        for mesh in &self.all_mesh{
            let mesh = unsafe{mesh.assume_safe()};
            let terrain_ids = mesh.map(|obj,_|obj.terrain_ids.clone()).expect("Could not find terrain ids in chunk");
            let locations = terrain_ids
                .into_iter()
                .map(|id| 
                    size_vector(
                        &owner.to_local(self.locations.get(&id).expect("id not ofund").clone())
                        )
                    )
                .collect::<Vec<Vector3>>();
            let _ = mesh.map(|obj,msh| obj.set_mesh_locations(msh,&locations) );
        }
    }
    #[method]
    fn set_id(&mut self,id:String){
        self.id = Some(id);
    }
    #[method]
    fn set_center(&mut self,id:String,location:Vector3){
        self.center = location;
    }
    
    #[method]
    fn set_render_distance(&mut self,value:f32){
        self.render_distance = value;
    }
    #[method]
    fn add_terrain_mesh(&mut self,#[base] owner:TRef<Spatial>,terrain_type:TerrainKey) -> Option<Ref<MultiMeshInstance>>{
        let instance_count = self.all_mesh.len();
        godot_print!("{}",format!("Adding terrain mesh with existing instances:{instance_count:?}"));
        if self.mesh_map.contains_key(&terrain_type){return None;}
        let mesh = NavigationPoints::make_instance().into_shared();
        let mesh = unsafe{mesh.assume_safe()};
        let _ = mesh.map_mut(|obj,_| obj.set_terrain_type(terrain_type));

        let mesh_instance = MultiMeshInstance::new().into_shared();
        let mesh_instance = unsafe{mesh_instance.assume_safe()};
        mesh_instance.set_multimesh(mesh.clone());

        let mesh = mesh.claim();
        self.all_mesh.push(mesh.clone());
        self.mesh_map.insert(terrain_type,mesh);
        owner.add_child(mesh_instance,true);
        self.instance_map.insert(terrain_type,mesh_instance.claim());
        None
    }
    #[method]
    fn remove_terrain_mesh(&mut self,#[base] owner:TRef<Spatial>,terrain_type:TerrainKey) -> Option<Ref<MultiMeshInstance>>{
        let mesh = self.instance_map.get(&terrain_type).expect("ChunkNavMeshErr: Instance not found during remove");
        let mesh = unsafe{mesh.assume_safe()};
        self.instance_map.remove(&terrain_type);
        owner.remove_child(mesh);
        mesh.queue_free();
        None
    }
    #[method]
    fn add_location_to_mesh(&mut self,terrain_id:TerrainId,terrain_type:TerrainKey,location:Vector3){
        let mesh = self.mesh_map.get(&terrain_type).expect("ChunkNavMeshErr:Instance for terrain type not found during add to loc");
        let mesh = unsafe{mesh.assume_safe()};
        self.locations.insert(terrain_id.clone(),location.clone());
        let _ = mesh.map_mut(|obj,_|obj.add_terrain(terrain_id,size_vector(&location)));
    }
    #[method]
    fn bake(&self){
        for mesh in &self.all_mesh{
            let mesh = unsafe{mesh.assume_safe()};
            let _ = mesh.map(|obj,msh| obj.bake_at(msh));
        }
    }
    #[method]
    fn set_hidden(&mut self,#[base] owner:TRef<Spatial>, value:bool){
        self.hidden = value;
        for mesh_instance in self.instance_map.values(){
            let mesh_instance = unsafe{mesh_instance.assume_safe()};
            mesh_instance.set_visible(!self.hidden);
        }
    }
    #[method]
    fn set_movement_locked(&mut self,value:bool){
        self.movement_lock = value;
    }
}
type TerrainId = String;
#[derive(NativeClass)]
#[inherit(Spatial)]
pub struct Waypoints{
    mesh_map:HashMap<ChunkId,Instance<ChunkNavMesh>>,
    terrain_id_to_chunk:HashMap<TerrainId,ChunkId>,
    chunk_to_location:HashMap<ChunkId,Vector3>,
}
impl Instanced<Spatial> for Waypoints{
    fn make() -> Self{
        Waypoints{
            mesh_map:HashMap::with_capacity(128),
            terrain_id_to_chunk:HashMap::with_capacity(2048 * 8),
            chunk_to_location:HashMap::with_capacity(128),
        }
    }
}
const MAX_DISTANCE_sqrt:f32 = 32.0;
const MAX_DISTANCE:f32 = MAX_DISTANCE_sqrt * MAX_DISTANCE_sqrt;
#[methods]
impl Waypoints{
    #[method]
    fn _ready(&self,#[base] owner:TRef<Spatial>){ }

    #[method]
    fn add_region_point(&mut self,#[base] owner:TRef<Spatial>,region_id:ChunkId,terrain_id:TerrainId,terrain_type:TerrainKey,global_location:Vector3){
        if self.terrain_id_to_chunk.contains_key(&terrain_id){return ;}
        self.terrain_id_to_chunk.insert(terrain_id.clone(),region_id.clone());

        match self.mesh_map.get(&region_id){
            Some(cm) => {}
            None => {
                let location = self.chunk_to_location.get(&region_id).expect(&format!("WaypointsErr: no location set for region_id {region_id:?}").to_string());
                let mesh = ChunkNavMesh::make_instance().into_shared();
                let mesh = unsafe{mesh.assume_safe()};
                let _ = mesh.map_mut(|obj,_| obj.set_id(region_id.clone()));
                let _ = mesh.map_mut(|obj,_| obj.set_center(region_id.clone(),location.clone()));
                let mesh = mesh.claim();
                self.mesh_map.insert(region_id.clone(),mesh.clone());
                owner.add_child(mesh,true);
            }
        }
        let chunk_mesh = self.mesh_map.get(&region_id).expect("WaypointsErr:Could not find chunkmesh, should be set");
        let chunk_mesh = unsafe{chunk_mesh.assume_safe()};
        let _ = chunk_mesh.map_mut(|obj,spatial|{
            //early returns if present
            obj.add_terrain_mesh(spatial,terrain_type);
            obj.add_location_to_mesh(terrain_id,terrain_type,global_location);
        });
    }
    #[method]
    fn bake(&self,region_id:ChunkId){
        let chunk_mesh = self.mesh_map.get(&region_id).expect("WaypointsErr:Did not find map for region_id during bake call");
        let chunk_mesh = unsafe{chunk_mesh.assume_safe()};
        let _ = chunk_mesh.map(|obj,_| obj.bake());
    }
    #[method]
    fn set_render_distance(&self,value:f32){
        for mesh in self.mesh_map.values(){
            let mesh = unsafe{mesh.assume_safe()};
            let _ = mesh.map_mut(|obj,_| obj.set_render_distance(value));
        }
    }
    #[method]
    fn set_chunk_visible(&self,chunk_id:ChunkId,visible:bool){
        self.mesh_map.get(&chunk_id).map(|chunk_mesh|{
            let chunk_mesh = unsafe{chunk_mesh.assume_safe()};
            let _ = chunk_mesh.map_mut(|obj,chunk| obj.set_hidden(chunk,!visible));
        });
    }
    #[method]
    fn set_movement_locked(&self,chunk_id:ChunkId,locked:bool){
        self.mesh_map.get(&chunk_id).map(|chunk_mesh|{
            let chunk_mesh = unsafe{chunk_mesh.assume_safe()};
            let _ = chunk_mesh.map_mut(|obj,chunk| obj.set_movement_locked(locked));
        });
    }
    #[method]
    fn add_chunk_location(&mut self,chunk_id:ChunkId,location:Vector3){
        self.chunk_to_location.insert(chunk_id,location);
    }
}
