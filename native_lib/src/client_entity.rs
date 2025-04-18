
use gdnative::prelude::*;
use gdnative::api::*;
use crate::traits::{Instanced};
use crate::socket_mode::{SocketMode};
use crate::network::gd_client_web_socket::{ClientWebSocket};


macro_rules! mystr {
    [$kind: ident, $x: expr, $y:expr] => {
        match $kind{
            _ => {
                return MyStruct{x:$x,y:$y}
            }
        }
    };
}

pub struct MyStruct{
    x:f32,
    y:f32
}


type Id = String;

trait Entity<T> where Self:Instanced<T>{
    fn id(&self) -> Id;
    fn socket_mode(&self) -> &SocketMode;
    fn set_socket_mode(&mut self, socket_mode:SocketMode);
    fn tick(&self) -> i64;
    fn set_tick(&mut self,value:i64);
}
trait Socketed{
    fn client_id(&self) -> Id;
    fn set_client_id(&mut self);
    fn client_socket(&self) -> Instance<ClientWebSocket>;
}
trait ClientEntity<T> where Self:Entity<T> {
    fn mesh(&self) -> &Option<Ref<Spatial>>;
    fn highlight_mesh(&self) -> &Option<Ref<MeshInstance>>;
    fn health(&self) -> f32;
    fn set_health(&mut self,value:f32);

    fn register_signals(builder:&ClassBuilder<Self>) where Self:Sized, Self:NativeClass{
        builder
            .signal("register")
            .with_param("id",VariantType::GodotString)
            .done();
        builder
            .signal("get_location")
            .with_param("id",VariantType::GodotString)
            .done();
    }
    fn ready(&self,owner:TRef<Spatial>){
        owner.emit_signal("register",&[Variant::new(self.id())]);
    }
    fn physics_process(&mut self,sender:TRef<Object>,delta:f32,trigger_tick:i64){
        let tick = self.tick();
        if tick == trigger_tick{
            self.set_tick(0);
            sender.emit_signal("get_location",&[Variant::new(self.id())]);
        }
        if tick == trigger_tick/2{
            sender.emit_signal("get_location",&[Variant::new(self.id())]);
        }
        self.set_tick(tick + 1);
    }
}

trait ServerEntity<T> where Self:Entity<T>{
    fn gravity_active(&self) -> bool;
    fn set_gravity_active(&mut self,value:bool);
    fn destinations_active(&self) -> bool;
    fn set_destinations_active(&mut self,value:bool);
    //fn physics_socket(&self) -> Ref<SharedRuntimeBytes>;

    fn register_signals(builder:&ClassBuilder<Self>) where Self:Sized, Self:NativeClass{
        builder
            .signal("send_direction")
            .with_param("id",VariantType::GodotString)
            .with_param("vec",VariantType::Vector3)
            .done();
        builder
            .signal("get_direction")
            .with_param("id",VariantType::GodotString)
            .done();
    }
    fn apply_vec_to_direction(&mut self,owner:TRef<Object>,vec:[f32;3]) where Self:ServerKinematicMovement{
        let mut dir = Vector3::new(0.0,0.0,0.0);
        dir.x = vec[0];
        dir.y = vec[1];
        dir.z = vec[2];
        let max_speed = self.speed();
        //proc is needed to ensure network doesn't get clogged in a loop when trying to reset speed
        //todo remove this entirely in favor of physics server managing this
        let dir = dir.normalized() * f32::min(dir.length(),max_speed);
        let tick = self.tick();
        let id = self.id();
        if tick == 2{
            self.set_tick(0);
            owner.emit_signal("send_direction",&[Variant::new(id),Variant::new(dir)]);
        }
        self.set_tick(tick + 1);
        if !self.destinations_active() || self.gravity_active(){
                self.set_direction(dir)
        }
    }
    fn physics_process_native(&self,owner:TRef<KinematicBody>,delta:f32){

    }
}

#[derive(NativeClass)]
#[inherit(KinematicBody)]
pub struct Prowler{
    id:Option<Id>,
    mesh:Option<Ref<Spatial>>,
    highlight_mesh:Option<Ref<MeshInstance>>,
    socket_mode:SocketMode,
    health:f32,
    tick:i64,
}
impl Instanced<KinematicBody> for Prowler{
    fn make() -> Self{
        Prowler{
            id:None,
            mesh:None,
            highlight_mesh:None,
            socket_mode:SocketMode::NativeLocOnlyDelta,
            health:1000.0,
            tick:0,
        }
    }
}
impl Entity<KinematicBody> for Prowler{
    fn id(&self) -> Id{self.id.clone().expect("no id found for Prowler")}
    fn socket_mode(&self) -> &SocketMode{&self.socket_mode}
    fn set_socket_mode(&mut self, socket_mode:SocketMode){self.socket_mode = socket_mode;}
    fn tick(&self) -> i64 {self.tick}
    fn set_tick(&mut self,value:i64){self.tick = value;}
}
impl ClientEntity<KinematicBody> for Prowler{
    fn mesh(&self) -> &Option<Ref<Spatial>>{&self.mesh}
    fn highlight_mesh(&self) -> &Option<Ref<MeshInstance>>{&self.highlight_mesh}
    fn health(&self) -> f32{self.health}
    fn set_health(&mut self,value:f32){self.health = value;}
}
#[methods]
impl Prowler{
    #[method]
    fn _ready(&self,#[base] owner:TRef<KinematicBody>){

    }
}

trait ClientKinematicMovement{
    fn set_direction(&mut self);
    fn direction(&self) -> Vector3;
    fn move_with_delta(&self,body:TRef<Spatial>,location:Vector3,delta:f32){
        let mut transform = body.global_transform();
        let origin = transform.origin;
        transform.origin = delta * (location - origin);
        body.set_transform(transform);
    }
    fn move_to(delta:f32,location:Vector3,body:TRef<Spatial>){
        let mut transform = body.global_transform();
        transform.origin = location;
        body.set_transform(transform);
    }

}
trait ServerKinematicMovement{
    fn set_direction(&mut self,value:Vector3);
    fn direction(&self) -> Vector3;
    fn speed(&self) -> f32;
    fn set_speed(&mut self,value:f32);

    fn teleport(&self,body:TRef<Spatial>,location:Vector3){
	body.translate(location - body.global_transform().origin);
    }
    fn move_by_gravity(&self,body:TRef<Spatial>,id:Id,location:Vector3,delta:f32){
	let diff:Vector3 = (body.global_transform().origin - location).normalized() * self.speed() * delta;
	let diff = -1.0 * diff * 0.05 * 10.0;
        body.emit_signal("apply_vec",&[Variant::new(id),Variant::new(diff)]);
    }
    fn move_to(&self,body:TRef<KinematicBody>,location:Vector3,delta:f32){
        let speed = self.speed();
	let diff:Vector3 = (body.global_transform().origin - location).normalized() * speed * delta;
	let dir = -1.0 * diff* 0.0005 * 10000.0;
	body.move_and_slide(dir,Vector3::UP,false,4,0.785398,true);
    }
    fn move_by_direction(&self,body:TRef<KinematicBody>){
        let speed = self.speed();
        let dir = self.direction();
	body.move_and_slide(dir * speed * 0.005,Vector3::UP,false,4,0.785398,true);
    }
    fn apply_direction(&mut self,direction:Vector3){
        let mut dir = self.direction();
	dir += direction.normalized();
        self.set_direction(dir);
    }
}
