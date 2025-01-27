
use gdnative::prelude::*;
use gdnative::api::*;
use crate::traits::{CreateSignal,Instanced,InstancedDefault,Defaulted};
use crate::field_ability_mesh::{FieldAbilityMesh};
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
    op_menu:Instance<FieldOps3D>,
}
impl InstancedDefault<KinematicBody,Location> for FieldZone{
    fn make(args:&Location) -> Self{
        FieldZone{
            location:*args,
            mesh:MeshInstance::new().into_shared(),
            op_menu:FieldOps3D::make_instance().into_shared(),
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
        let cube_size = Vector3{x:zone_width,y:zone_height,z:zone_width}; 
        cube.set_size(cube_size);
        mesh.set_mesh(cube);
        let owner_transform = owner.transform();

        //initialize
        //op_menu.map(|_ , control| control.set_size(Vector2{x:200.0,y:200.0},false));
        op_menu.map(|_,spatial|{
            let mut transform = spatial.transform();
            transform.origin = cube_size/2.0;
        });
        op_menu.map(|obj , spatial| obj.hide(spatial));

        op_menu.map_mut(|obj,control| obj.add_op(control,255));
        op_menu.map_mut(|obj,control| obj.add_op(control,0));


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
        op_menu.map(|obj,spatial| obj.toggle(spatial));
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
        owner.add_child(zone.clone(),true);
        zone.map(|obj,spatial| {
            let mut transform = spatial.transform();
            transform.origin = Vector3{x:zone_width * (location.x as f32),y:0.0,z:zone_width * (location.y as f32)};
            spatial.set_transform(transform);
        });
    }

}

#[derive(Copy,Clone)]
pub enum OpType{
    empty,
    smack,
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
            0 => OpType::smack,
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
            OpType::smack => "Smack".to_string(),
        }
    }
}

#[derive(NativeClass)]
#[inherit(KinematicBody)]
pub struct FieldOp3D{
    mesh:Instance<FieldAbilityMesh>,
    highlight_left:Ref<MeshInstance>,
    highlight_right:Ref<MeshInstance>,
    radius:f64,
}
impl InstancedDefault<KinematicBody,OpType> for FieldOp3D{
    fn make(args:&OpType) -> Self{
        FieldOp3D{
            mesh:FieldAbilityMesh::make_instance(args).into_shared(),
            highlight_left:MeshInstance::new().into_shared(),
            highlight_right:MeshInstance::new().into_shared(),
            radius:5.0,
        }
    }
}
#[methods]
impl FieldOp3D{
    #[method]
    fn _ready(&self,#[base] owner:TRef<KinematicBody>){
        let mesh = unsafe{self.mesh.assume_safe()};
        mesh.map(|_,spatial| owner.add_child(spatial,true));

        let highlight_left = unsafe{self.highlight_left.assume_safe()};
        let highlight_right = unsafe{self.highlight_right.assume_safe()};

        let highlight_left_mesh = CubeMesh::new().into_shared();
        let highlight_left_mesh = unsafe{highlight_left_mesh.assume_safe()};
        let highlight_right_mesh = CubeMesh::new().into_shared();
        let highlight_right_mesh = unsafe{highlight_right_mesh.assume_safe()};
        
        highlight_left_mesh.set_size(Vector3{x:5.0,y:(self.radius + 1.0) as f32,z:(self.radius + 1.0) as f32});
        highlight_right_mesh.set_size(Vector3{x:5.0,y:(self.radius + 1.0) as f32,z:(self.radius + 1.0) as f32});

        let highlight_material = SpatialMaterial::new().into_shared() ;
        let highlight_material = unsafe{highlight_material.assume_safe()};

        highlight_material.set_albedo(Color{r:255.0,g:255.0,b:255.0,a:1.0});

        highlight_left_mesh.set_material(highlight_material);
        highlight_right_mesh.set_material(highlight_material);

        highlight_left.set_mesh(highlight_left_mesh);
        highlight_right.set_mesh(highlight_right_mesh);

        let mut left_transform = highlight_left.transform();
        let mut right_transform = highlight_right.transform();
        left_transform.origin = Vector3{x:-12.5,y:0.0,z:0.0};
        right_transform.origin = Vector3{x:12.5,y:0.0,z:0.0};
        highlight_left.set_transform(left_transform);
        highlight_right.set_transform(right_transform);
        highlight_left.set_visible(false);
        highlight_right.set_visible(false);

        let collision_shape = BoxShape::new().into_shared();
        let collision_shape = unsafe{collision_shape.assume_safe()};
        let collision_object = CollisionShape::new().into_shared();
        let collision_object = unsafe{collision_object.assume_safe()};
        collision_shape.set_extents(Vector3{x:12.5,y:(self.radius/2.0) as f32,z:(self.radius/2.0) as f32});
        collision_object.set_shape(collision_shape);

        owner.add_child(mesh,true);
        owner.add_child(collision_object,true);
        owner.add_child(highlight_left,true);
        owner.add_child(highlight_right,true);
    }
    #[method]
    fn entered(&self,#[base] owner:TRef<KinematicBody>){
        let highlight_left = unsafe{self.highlight_left.assume_safe()};
        let highlight_right = unsafe{self.highlight_right.assume_safe()};
        highlight_left.set_visible(true);
        highlight_right.set_visible(true);
    }
    #[method]
    fn exited(&self,#[base] owner:TRef<KinematicBody>){
        let highlight_left = unsafe{self.highlight_left.assume_safe()};
        let highlight_right = unsafe{self.highlight_right.assume_safe()};
        highlight_left.set_visible(false);
        highlight_right.set_visible(false);
    }
    
}
#[derive(NativeClass)]
#[inherit(Spatial)]
pub struct FieldOps3D{
    operations:Vec<Instance<FieldOp3D>>,
}
impl Instanced<Spatial> for FieldOps3D{
    fn make() -> Self{
        FieldOps3D{
            operations:Vec::new(),
        }
    }
}
#[methods]
impl FieldOps3D{
    #[method]
    fn _ready(&self,#[base] owner:TRef<Spatial>){
    }
    
    #[method]
    fn add_op(&mut self,#[base] owner:TRef<Spatial>,typ:u8){
        let op = FieldOp3D::make_instance(&OpType::from(typ)).into_shared();
        owner.add_child(op.clone(),true);
        let num_ops = self.operations.len() as f32;
        self.operations.push(op.clone());
        let op = unsafe{op.assume_safe()};
        op.map(|obj,spatial| {
            let mut transform = spatial.transform();
            let radius = obj.radius as f32;
            transform.origin = Vector3{x:0.0,y: radius + (num_ops * 2.0*radius),z:0.0};
            spatial.set_transform(transform);
        });
    }
    #[method]
    fn show(&self, #[base] owner:TRef<Spatial>){
        for op in &self.operations{
            let op = unsafe{op.assume_safe()};
            op.map(|_,body| {
                body.set_collision_layer_bit(0,true);
                body.set_collision_mask_bit(0,true);
            });
        }
        owner.set_visible(true);
    }
    #[method]
    fn hide(&self, #[base] owner:TRef<Spatial>){
        for op in &self.operations{
            let op = unsafe{op.assume_safe()};
            op.map(|_,body| {
                body.set_collision_layer_bit(0,false);
                body.set_collision_mask_bit(0,false);
            });
        }
        owner.set_visible(false);
    }
    #[method]
    fn toggle(&self, #[base] owner:TRef<Spatial>){
        owner.set_visible(!owner.is_visible());
        for op in &self.operations{
            let op = unsafe{op.assume_safe()};
            op.map(|_,body| {
                body.set_collision_layer_bit(0,owner.is_visible());
                body.set_collision_mask_bit(0,owner.is_visible());
            });
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
            bg_color:Color{r:255.0,g:0.0,b:0.0,a:1.0},
            color:Color{r:0.0,g:255.0,b:0.0,a:1.0},
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
        
        bg_rect.set_frame_color(self.bg_color);
        main_rect.set_frame_color(self.color);
        label.set_text(self.typ.to_label());

        //owner.add_child(bg_rect,true);
        owner.add_child(main_rect,true);
        owner.add_child(label,true);
    }

    #[method]
    fn _process(&self,#[base] owner:TRef<Control>,delta:f64){
        let label = unsafe{self.label.assume_safe()};
        let bg_rect = unsafe{self.bg_rect.assume_safe()};
        let main_rect = unsafe{self.bg_rect.assume_safe()};

        let owner_size = owner.size();
        let offset = 20.0;
        let vector_offset = Vector2{x:offset,y:offset};

        bg_rect.set_size(owner_size,false);
        main_rect.set_size(owner_size - vector_offset,false);
        label.set_size(owner_size/2.0,false);

        bg_rect.set_position(Vector2{x:vector_offset.x/-2.0,y:0.0},false);
        main_rect.set_position(bg_rect.position() + vector_offset/2.0,false);
        label.set_position(owner_size/2.0 - label.size()/2.0,false);
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

        //let op_size = owner_size / (num_ops * ((num_ops > 0.0) as i32 as f32) + 1.0 * ((num_ops == 0.0) as i32 as f32));
        let op_size = owner_size/num_ops;
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
