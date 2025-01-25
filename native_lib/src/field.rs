
use gdnative::prelude::*;
use gdnative::api::*;
use crate::traits::{CreateSignal,Instanced,InstancedDefault,Defaulted};
use tokio::sync::mpsc;

const zone_width:f32 = 20.0;
const zone_height:f32 = 1.0;
#[derive(Copy,Clone)]
pub struct Location{
    x:i64,
    y:i64,
}
impl Defaulted for Location{
    fn default() -> Self{
        Location{x:0,y:0}
    }
}
#[derive(NativeClass)]
#[inherit(KinematicBody)]
pub struct FieldZone{
    location:Location,
    mesh:Ref<MeshInstance>,
    op_menu:Instance<FieldOps>,
}
impl InstancedDefault<KinematicBody,Location> for FieldZone{
    fn make(args:&Location) -> Self{
        FieldZone{
            location:*args,
            mesh:MeshInstance::new().into_shared(),
            op_menu:FieldOps::make_instance().into_shared(),
        }
    }
}
#[methods]
impl FieldZone{
    #[method]
    fn _ready(&self,#[base] owner:TRef<KinematicBody>){
        let op_menu = unsafe{self.op_menu.assume_safe()};
        let mesh = unsafe{self.mesh.assume_safe()};
        let cube = CubeMesh::new().into_shared();
        let cube = unsafe{cube.assume_safe()};
        cube.set_size(Vector3{x:zone_width,y:zone_height,z:zone_width});
        mesh.set_mesh(cube);

        //initialize
        op_menu.map(|_ , control| control.set_size(Vector2{x:200.0,y:200.0},false));
        op_menu.map_mut(|obj,control| obj.add_op(control,255));

        let collision_shape = BoxShape::new().into_shared();
        let collision_shape = unsafe{collision_shape.assume_safe()};
        collision_shape.set_extents(Vector3{x:zone_width/2.0,y:zone_height/2.0,z:zone_width/2.0});

        let collider = CollisionShape::new().into_shared();
        let collider = unsafe{collider.assume_safe()};
        collider.set_shape(collision_shape);

        mesh.set_visible(false);
        //add children
        owner.add_child(collider,true);
        owner.add_child(mesh,true);
        owner.add_child(op_menu,true);
        //add signals
        owner.connect("mouse_entered",owner,"clicked",VariantArray::new_shared(),0);
        
    }
    #[method]
    fn clicked(&self,#[base] owner:TRef<KinematicBody>,event_position:Vector2,intersect_position:Vector3){
        godot_print!("Field Area Clicked!");
        let op_menu = unsafe{self.op_menu.assume_safe()};
        op_menu.map(|_,control| control.set_visible(!control.is_visible()));
        op_menu.map(|_,control| control.set_position(event_position,false));
    }
    #[method]
    fn entered(&self,#[base] owner:TRef<KinematicBody>){
        let mesh = unsafe{self.mesh.assume_safe()};
        mesh.set_visible(true);
        godot_print!("Body Entered!");
    }
    #[method]
    fn exited(&self,#[base] owner:TRef<KinematicBody>){
        let mesh = unsafe{self.mesh.assume_safe()};
        mesh.set_visible(false);
        godot_print!("Body Exited!");
    }
}
#[derive(NativeClass)]
#[inherit(Spatial)]
pub struct Field{
    zones:Vec<Instance<FieldZone>>,
}
impl Instanced<Spatial> for Field{
    fn make() -> Self{
        Field{
            zones:Vec::new(),
        }
    }
}
#[methods]
impl Field{
    #[method]
    fn _ready(&self, #[base] owner:TRef<Spatial>){
    }

    #[method]
    fn _process(&self,#[base] owner:TRef<Spatial>,delta:f64){

    }
    #[method]
    fn add_zone(&self,#[base] owner:TRef<Spatial>,location:(i64,i64)){
        let (x,y) = location;
        let location = Location{x:x,y:y};
        let zone = FieldZone::make_instance(&location).into_shared();
        let zone = unsafe{zone.assume_safe()};
        let owner_transform = owner.transform();
        owner.add_child(zone.clone(),true);
        zone.map(|obj,spatial| {
            let mut transform = spatial.transform();
            transform.origin = Vector3{x:zone_width * (location.x as f32),y:0.0,z:zone_width * (location.y as f32)};
            spatial.set_transform(transform);
        });
    }

}

#[derive(Copy,Clone)]
enum OpType{
    empty,
}
impl Defaulted for OpType{
    fn default() -> Self{
        OpType::empty
    }
}
impl From<u8> for OpType{
    fn from(value:u8) -> Self{
        match value{
            255 => OpType::empty,
            _ => todo!(),
        }
    }
}
trait ToLabel{
    fn to_label(&self) -> String;
}
impl ToLabel for OpType{
    fn to_label(&self) -> String{
        match self{
            OpType::empty => "Empty".to_string(),
        }
    }
}

#[derive(NativeClass)]
#[inherit(Control)]
pub struct FieldOp{
    typ:OpType,
    label:Ref<Label>,
    bg_rect:Ref<ColorRect>,
    main_rect:Ref<ColorRect>,
    bg_color:Color,
    color:Color,
}
impl InstancedDefault<Control,OpType> for FieldOp{
    fn make(args:&OpType) -> Self{
        FieldOp{
            typ:*args,
            label:Label::new().into_shared(),
            bg_rect:ColorRect::new().into_shared(),
            main_rect:ColorRect::new().into_shared(),
            bg_color:Color{r:0.0,g:0.0,b:0.0,a:1.0},
            color:Color{r:0.0,g:0.0,b:0.0,a:0.6},
        }
    }
}
#[methods]
impl FieldOp{
    #[method]
    fn _ready(&self,#[base] owner:TRef<Control>){
        let label = unsafe{self.label.assume_safe()};
        let bg_rect = unsafe{self.bg_rect.assume_safe()};
        let main_rect = unsafe{self.main_rect.assume_safe()};
        
        label.set_text(self.typ.to_label());
        bg_rect.set_frame_color(self.bg_color);
        main_rect.set_frame_color(self.color);

        owner.add_child(label,true);
        owner.add_child(bg_rect,true);
        owner.add_child(main_rect,true);
    }

    #[method]
    fn _process(&self,#[base] owner:TRef<Control>,delta:f64){
        let label = unsafe{self.label.assume_safe()};
        let bg_rect = unsafe{self.bg_rect.assume_safe()};
        let main_rect = unsafe{self.bg_rect.assume_safe()};
        let owner_size = owner.size();
        let offset = 10.0;
        let vector_offset = Vector2{x:offset,y:offset};

        bg_rect.set_size(owner_size,false);
        main_rect.set_size(owner_size - vector_offset/2.0,false);
        label.set_size(main_rect.size(),false);

        main_rect.set_position(vector_offset,false);
    }
}
#[derive(NativeClass)]
#[inherit(Control)]
pub struct FieldOps{
    operations:Vec<Instance<FieldOp>>,

}
impl Instanced<Control> for FieldOps{
    fn make() -> Self{
        FieldOps{
            operations:Vec::new(),
        }

    }
}
#[methods]
impl FieldOps{
    #[method]
    fn _ready(&self,#[base] owner:TRef<Control>){

    }
    #[method]
    fn add_op(&mut self,#[base] owner:TRef<Control>,typ:u8){
        let op = FieldOp::make_instance(&OpType::from(typ)).into_shared();
        owner.add_child(op.clone(),true);
        self.operations.push(op);
        
    }
    #[method]
    fn _process(&self,#[base] owner:TRef<Control> , delta:f64){
        let owner_size = owner.size(); 
        let num_ops = self.operations.len() as f32;

        let op_size = owner_size / (num_ops * ((num_ops > 0.0) as i32 as f32) + 1.0 * ((num_ops == 0.0) as i32 as f32));
        for op in &self.operations{
            let op = unsafe{op.assume_safe()};
            let mut idx = 0.0;
            op.map(|obj,control| {
                control.set_size(op_size,false);
                control.set_position(op_size * idx ,false);
            });
            idx += 1.0;
        }

    }
}
