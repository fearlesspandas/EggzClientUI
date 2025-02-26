use gdnative::prelude::*;
use gdnative::api::*;
use rand::prelude::*;
use tokio::sync::mpsc;
use crate::traits::{Instanced};


type Sender<T> = mpsc::UnboundedSender<T>;
type Receiver<T> = mpsc::UnboundedReceiver<T>;
enum OrbitNodeType{
    SphereOrbital(Color)
}
enum OrbitNodeCommand{
    SetNodeType(OrbitNodeType),
}
impl OrbitNodeType{
    fn to_mesh(&self) -> Ref<Mesh>{
        match self{
            OrbitNodeType::SphereOrbital(color) => {
                let mesh = SphereMesh::new().into_shared();
                let meshi = unsafe{mesh.assume_safe()};
                let material = SpatialMaterial::new().into_shared();
                let material = unsafe{material.assume_safe()};
                meshi.set_material(material);
                material.set_albedo(Color{r:227.0,g:0.0,b:225.0,a:1.0});
                meshi.set_radius(0.5);
                meshi.set_height(1.0);
                mesh.upcast::<Mesh>()
            }
        }
    }
    fn orbit(&self,body:TRef<Spatial>,delta:f64,axis:Vector3){
        match self{
            OrbitNodeType::SphereOrbital(_) => {
                let delta = delta as f32;
                let distance = 5.0;
                let mut transform = body.transform();
                transform.origin = transform.origin.rotated(axis.normalized(),delta *5.0);
                transform.origin = transform.origin.normalized() * distance;
                body.set_transform(transform);
            }
        }
    }
}
#[derive(NativeClass)]
#[inherit(MeshInstance)]
pub struct OrbitNode{
    mesh:Option<Ref<Mesh>>,
    node_type:Option<OrbitNodeType>,
    tx:Sender<OrbitNodeCommand>,
    rx:Receiver<OrbitNodeCommand>,
    axis:Vector3,
}
impl Instanced<MeshInstance> for OrbitNode{
    fn make() -> Self{
        let (tx,rx) = mpsc::unbounded_channel::<OrbitNodeCommand>();
        OrbitNode{
            mesh:None,
            node_type: None,
            tx:tx,
            rx:rx,
            axis: Vector3::UP,
        }
    }
}
#[methods]
impl OrbitNode{
    fn set_type(&self,node_type:OrbitNodeType){
        let _ = self.tx.send(OrbitNodeCommand::SetNodeType(node_type));
    }
    #[method]
    fn _process(&mut self,#[base] owner:TRef<MeshInstance>,delta:f64){
        let location = owner.transform().origin;
        match self.rx.try_recv(){
            Ok(OrbitNodeCommand::SetNodeType(node_type)) => {
                let mesh = node_type.to_mesh();
                owner.set_mesh(mesh.clone());
                self.mesh = Some(mesh);
                self.node_type = Some(node_type);
            }
            Err(_) => {}
        }
        self.orbit(owner,delta);
    }
    #[method]
    fn orbit(&mut self,#[base] owner:TRef<MeshInstance>,delta:f64){
        if rand::random::<f32>() < 0.01{
            let x = rand::random::<f32>()-0.5;
            let y = rand::random::<f32>()-0.5;
            let z = rand::random::<f32>()-0.5;
            self.axis.x = x;
            self.axis.y = y;
            self.axis.z = z;
        }
        let _ = self.node_type.as_ref().map(|node| {
            node.orbit(owner.upcast::<Spatial>(),delta,self.axis);
        });
    }
}
#[derive(NativeClass)]
#[inherit(Spatial)]
pub struct BeltOrbital{
    nodes:Vec<Instance<OrbitNode>>,
}
impl Instanced<Spatial> for BeltOrbital{
    fn make() -> Self{
        BeltOrbital{
            nodes:Vec::new(),
        }
    }
}

#[methods]
impl BeltOrbital{
    #[method]
    fn add_sphere_node(&mut self,#[base] owner:TRef<Spatial>){
        let node = OrbitNode::make_instance().into_shared();
        let node_r = unsafe{node.assume_safe()};
        let _ = node_r.map(|obj,_| obj.set_type(OrbitNodeType::SphereOrbital(Color{r:100.0,g:0.0,b:0.0,a:1.0})));
        let _ = node_r.map(|_,spatial| spatial.translate(Vector3{x:0.0,y:0.0,z:5.0}));
        let _ = node_r.map(|_,control| owner.add_child(control,true));
        self.nodes.push(node);
    }
    #[method]
    fn _ready(&mut self,#[base] owner:TRef<Spatial>){
        self.add_sphere_node(owner);
        self.add_sphere_node(owner);
        self.add_sphere_node(owner);
        self.add_sphere_node(owner);
        self.add_sphere_node(owner);
        self.add_sphere_node(owner);
        self.add_sphere_node(owner);
    }
    #[method]
    fn _process(&self,#[base] owner:TRef<Spatial>,delta:f64){

    }
}


