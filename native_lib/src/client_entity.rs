
use gdnative::prelude::*;
use gdnative::api::*;
use crate::traits::{Instanced};



trait Entity where Self:Instanced<KinematicBody>{
    fn id(&self) -> String;
    fn mesh(&self) -> &Option<Ref<MeshInstance>>;
    fn move_body(&self,location:Vector3);
}

trait ClientPlayerEntity where Self:Entity{
    
}
