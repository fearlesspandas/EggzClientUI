
use std::collections::HashMap;
use gdnative::prelude::*;
use gdnative::api::*;
use crate::traits::{CreateSignal,Instanced,InstancedDefault,Defaulted};
use crate::field_ability_mesh::{FieldAbilityMesh};
use crate::field_ability_actions::ToAction;
use crate::field_abilities::{AbilityType,SubAbilityType};
use tokio::sync::mpsc;

type Sender<T> = mpsc::UnboundedSender<T>;
type Receiver<T> = mpsc::UnboundedReceiver<T>;

pub const ZONE_WIDTH:f32 = 20.0;
pub const ZONE_HEIGHT:f32 = 1.0;

const COLLISION_LAYER:i64 = 20;

pub enum FieldZoneCommand{
    Selected(AbilityType),
    Proc(bool),

}
#[derive(Copy,Clone,Eq,Hash,PartialEq,Debug)]
pub struct Location{
    pub x:i64,
    pub y:i64,
}
impl Defaulted for Location{
    fn default() -> Self{
        Location{x:0,y:0}
    }
}
#[derive(NativeClass)]
#[inherit(StaticBody)]
pub struct FieldZone{
    location:Location,
    mesh:Ref<MeshInstance>,
    proc_mesh:Ref<MeshInstance>,
    op_menu:Instance<FieldOps3D>,
    pub abilities:HashMap<AbilityType,Instance<FieldAbilityMesh>>,
    zone_tx:Sender<FieldZoneCommand>,
    zone_rx:Receiver<FieldZoneCommand>,
    field_tx:Option<Sender<FieldCommand>>,
    pub proc:bool,
}
impl InstancedDefault<StaticBody,Location> for FieldZone{
    fn make(args:&Location) -> Self{
        let (tx,rx) = mpsc::unbounded_channel::<FieldZoneCommand>();
        let op_menu = FieldOps3D::make_instance().into_shared();
        let op_menu_obj = unsafe{op_menu.assume_safe()};
        let _ = op_menu_obj.map_mut(|obj,_| obj.set_tx(tx.clone()));
        FieldZone{
            location:*args,
            mesh:MeshInstance::new().into_shared(),
            proc_mesh:MeshInstance::new().into_shared(),
            op_menu:op_menu,
            abilities:HashMap::new(),
            zone_tx: tx,
            zone_rx: rx,
            field_tx:None,
            proc:false,
        }
    }
}
#[methods]
impl FieldZone{
    #[method]
    fn _ready(&self,#[base] owner:TRef<StaticBody>){
        let op_menu = unsafe{self.op_menu.assume_safe()};
        let mesh = unsafe{self.mesh.assume_safe()};
        let proc_mesh = unsafe{self.proc_mesh.assume_safe()};
        let cube = CubeMesh::new().into_shared();
        let cube = unsafe{cube.assume_safe()};
        let proc_cube = CubeMesh::new().into_shared();
        let proc_cube = unsafe{proc_cube.assume_safe()};
        let cube_size = Vector3{x:ZONE_WIDTH,y:ZONE_HEIGHT,z:ZONE_WIDTH}; 
        cube.set_size(cube_size);
        mesh.set_mesh(cube);
        let proc_mesh_material = SpatialMaterial::new().into_shared();
        let proc_mesh_material = unsafe{proc_mesh_material.assume_safe()};
        proc_mesh_material.set_albedo(Color{r:0.0,g:255.0,b:0.0,a:1.0});
        proc_cube.set_size(cube_size + Vector3{x:1.0,y:1.0,z:1.0});
        proc_cube.set_material(proc_mesh_material);
        proc_mesh.set_mesh(proc_cube);

        //initialize
        //op_menu.map(|_ , control| control.set_size(Vector2{x:200.0,y:200.0},false));
        let _ = op_menu.map(|_,spatial|{
            let mut transform = spatial.transform();
            transform.origin = cube_size/2.0;
        });

        //technically unneeded
        let _ = op_menu.map_mut(|obj,_| obj.set_tx(self.zone_tx.clone()));
            
        let _ = op_menu.map_mut(|obj,control| obj.add_op(control,255,1));
        let _ = op_menu.map_mut(|obj,_| obj.width = ZONE_WIDTH/2.0);
        let _ = op_menu.map(|obj , spatial| obj.hide(spatial));


        let collision_shape = BoxShape::new().into_shared();
        let collision_shape = unsafe{collision_shape.assume_safe()};
        collision_shape.set_extents(Vector3{x:ZONE_WIDTH/2.0,y:ZONE_HEIGHT/2.0,z:ZONE_WIDTH/2.0});

        let collider = CollisionShape::new().into_shared();
        let collider = unsafe{collider.assume_safe()};
        collider.set_shape(collision_shape);

        mesh.set_visible(false);
        proc_mesh.set_visible(false);
        //add children
        owner.add_child(collider,true);
        owner.set_collision_layer_bit(0,false);
        owner.set_collision_mask_bit(0,false);
        owner.set_collision_layer_bit(COLLISION_LAYER,true);
        owner.set_collision_mask_bit(COLLISION_LAYER,true);
        owner.add_child(mesh,true);
        owner.add_child(proc_mesh,true);
        owner.add_child(op_menu,true);
        //add signals
        
    }
    #[method]
    fn _process(&mut self,#[base] _owner:TRef<StaticBody>,_delta:f64){
        match self.zone_rx.try_recv() {
            Ok(FieldZoneCommand::Selected(typ)) => {
                let _ = self.field_tx
                    .as_ref()
                    .expect("field_tx not set for zone")
                    .send(FieldCommand::AddAbility(self.location,typ));
            }
            Ok(FieldZoneCommand::Proc(value)) => {
                let mesh = unsafe{ self.proc_mesh.assume_safe()};
                if value{
                    mesh.set_visible(true);
                }
                else{
                    mesh.set_visible(false);
                }
            }
            Err(_) => {}
        }
    }
    #[method]
    fn clicked(&self,#[base] _owner:TRef<StaticBody>,_event_position:Vector2,_intersect_position:Vector3){
        godot_print!("Field Area Clicked!");
        if self.abilities.len() == 0{
            let op_menu = unsafe{self.op_menu.assume_safe()};
            let _ = op_menu.map(|obj,spatial| obj.toggle(spatial));
        }else{
            let field_tx = self.field_tx.clone().unwrap();
            for typ in self.abilities.keys(){
                let _ = field_tx.send(FieldCommand::Trigger(self.location,*typ));
            }
        }
    }
    #[method]
    fn add_op_to_menu(&self,ability_id:u8,amount:i64){
        let op_menu = unsafe{self.op_menu.assume_safe()};
        let _ = op_menu.map_mut(|obj,control| obj.add_op(control,ability_id,amount));
    }
    #[method]
    fn remove_op_from_menu(&self,ability_id:u8,amount:i64){
        let op_menu = unsafe{self.op_menu.assume_safe()};
        let _ = op_menu.map_mut(|obj,control| obj.remove_op(control,ability_id,amount));
    }
    #[method]
    fn clear_operations(&self){
        let op_menu = unsafe{self.op_menu.assume_safe()};
        let _ = op_menu.map_mut(|obj,control| obj.clear(control));
    }
    #[method]
    fn place_ability(&mut self,#[base] owner:TRef<StaticBody>,typ:u8){
        let typ = AbilityType::from(typ);
        if self.abilities.contains_key(&typ){return ;}
        let mesh = FieldAbilityMesh::make_instance(&typ).into_shared();
        self.abilities.insert(typ,mesh.clone());
        let mesh = unsafe{mesh.assume_safe()};
        owner.add_child(mesh,true);
        let op_menu = unsafe{self.op_menu.assume_safe()};
        let _ = op_menu.map(|obj,spatial| obj.hide(spatial));
    }
    #[method]
    pub fn remove_ability(&mut self,#[base] owner:TRef<StaticBody>, typ:u8){
        let typ = AbilityType::from(typ);
        if self.abilities.contains_key(&typ){
            let ability = self.abilities.get(&typ).unwrap();
            owner.remove_child(ability);
            let ability = unsafe{ability.assume_safe()};
            let _ = ability.map(|_,mesh| mesh.queue_free());
            self.abilities.remove(&typ);
        }
    }
    #[method]
    fn entered(&self,#[base] _owner:TRef<StaticBody>){
        let mesh = unsafe{self.mesh.assume_safe()};
        mesh.set_visible(true);
        //godot_print!("Body Entered!");
    }
    #[method]
    fn exited(&self,#[base] _owner:TRef<StaticBody>){
        let mesh = unsafe{self.mesh.assume_safe()};
        mesh.set_visible(false);
        //godot_print!("Body Exited!");
    }
    #[method]
    fn hide(&self,#[base] owner:TRef<StaticBody>){
        if self.abilities.len() == 0{
            let op_menu = unsafe{self.op_menu.assume_safe()};
            let _ = op_menu.map(|obj,spatial| obj.hide(spatial));
        }        
        owner.set_visible(false);
        owner.set_collision_layer_bit(COLLISION_LAYER,false);
        owner.set_collision_mask_bit(COLLISION_LAYER,false);
    }
    #[method]
    fn show(&self,#[base] owner:TRef<StaticBody>){
        owner.set_visible(true);
        owner.set_collision_layer_bit(COLLISION_LAYER,true);
        owner.set_collision_mask_bit(COLLISION_LAYER,true);
    }
    #[method]
    fn toggle(&self,#[base] owner:TRef<StaticBody>){
        if owner.is_visible(){
            self.hide(owner);
        }else{
            self.show(owner);
        }
    }
    fn set_tx(&mut self,tx:Sender<FieldCommand>){
        self.field_tx = Some(tx);
    }
    pub fn proc(&mut self){
        self.proc = true;
        let _ = self.zone_tx.send(FieldZoneCommand::Proc(true));
    }
    pub fn unproc(&mut self){
        self.proc = false;
        let _ = self.zone_tx.send(FieldZoneCommand::Proc(false));
    }
}
pub enum FieldCommand{
    AddAbility(Location,AbilityType),
    DoAbility(Location,AbilityType),
    ModifyAbility(Location,SubAbilityType),
    Trigger(Location,AbilityType),
}
impl ToString for FieldCommand{
    fn to_string(&self) -> String{
        match self{
            FieldCommand::AddAbility(_,_) => "add_ability".to_string(),
            FieldCommand::DoAbility(_,_) => "do_ability".to_string(),
            FieldCommand::ModifyAbility(_,_) => "modify_ability".to_string(),
            FieldCommand::Trigger(_,_) => "trigger".to_string(),
        }
    }
}
impl CreateSignal<Field> for FieldCommand{
    fn register(builder:&ClassBuilder<Field>){
        builder
            .signal(&FieldCommand::AddAbility(Location::default(),AbilityType::empty).to_string())
            .with_param("location",VariantType::VariantArray)
            .with_param("ability_id",VariantType::I64)
            .done();
        builder
            .signal(&FieldCommand::DoAbility(Location::default(),AbilityType::empty).to_string())
            .with_param("location",VariantType::VariantArray)
            .with_param("type",VariantType::I64)
            .done();
        builder
            .signal(&FieldCommand::ModifyAbility(Location::default(),SubAbilityType::empty).to_string())
            .with_param("location",VariantType::VariantArray)
            .with_param("type",VariantType::I64)
            .done();
    }
}

#[derive(NativeClass)]
#[inherit(Spatial)]
#[register_with(Self::register_signals)]
pub struct Field{
    zones:HashMap<Location,Instance<FieldZone>>,
    tx:Sender<FieldCommand>,
    rx:Receiver<FieldCommand>
}
impl Instanced<Spatial> for Field{
    fn make() -> Self{
        let (tx,rx) = mpsc::unbounded_channel::<FieldCommand>();
        Field{
            zones:HashMap::new(),
            tx:tx,
            rx:rx,
        }
    }
}
#[methods]
impl Field{
    fn register_signals(builder:&ClassBuilder<Self>){
        FieldCommand::register(builder);
    }
    #[method]
    fn _ready(&self, #[base] _owner:TRef<Spatial>){
    }
    #[method]
    fn _process(&mut self,#[base] owner:TRef<Spatial>,_delta:f64){
        match self.rx.try_recv(){
            Ok(cmd) => {
                match cmd{
                    FieldCommand::AddAbility(location,typ) => {
                        //godot_print!("{}",format!("Ability Selected! {location:?} {typ:?}"));
                        let mut loc:Vec<i64> = Vec::new();
                        loc.push(location.x);
                        loc.push(location.y);
                        godot_print!("{}",format!("location : {loc:?}"));
                        owner.emit_signal(cmd.to_string(),&[Variant::new(loc),Variant::new(Into::<u8>::into(typ))]);
                    }
                    FieldCommand::Trigger(location,typ) => {
                        typ.to_action(self.tx.clone(),&location,&self.zones);
                    }
                    FieldCommand::ModifyAbility(location,typ) => {
                        let mut loc:Vec<i64> = Vec::new();
                        loc.push(location.x);
                        loc.push(location.y);
                        owner.emit_signal(cmd.to_string(),&[Variant::new(loc),Variant::new(Into::<u8>::into(typ))]);
                    }
                    FieldCommand::DoAbility(location,typ) => {
                        let mut loc:Vec<i64> = Vec::new();
                        loc.push(location.x);
                        loc.push(location.y);
                        owner.emit_signal(cmd.to_string(),&[Variant::new(loc),Variant::new(Into::<u8>::into(typ))]);
                    }
                }
            }
            Err(_) => {}
        }
    }
    #[method]
    fn _input(&self,#[base] owner:TRef<Spatial>, event:Ref<InputEvent>){
        let event = unsafe{event.assume_safe()};
        if event.is_action_released("toggle_field",true){
            self.toggle(owner);
        }
    }
    #[method]
    fn get_point_from_location(&self,x:i64,y:i64) -> Vector3{
        Vector3{x:ZONE_WIDTH * (x as f32),y:0.0,z:ZONE_WIDTH * (y as f32)}
    }
    #[method]
    fn add_zone(&mut self,#[base] owner:TRef<Spatial>,location:(i64,i64)){
        let (x,y) = location;
        let location = Location{x:x,y:y};
        if self.zones.contains_key(&location){return ;}
        let zone = FieldZone::make_instance(&location).into_shared();
        self.zones.insert(location,zone.clone());
        let zone = unsafe{zone.assume_safe()};
        let _ = zone.map_mut(|obj,_| obj.set_tx(self.tx.clone()));
        owner.add_child(zone.clone(),false);
        let _ = zone.map(|_,spatial| {
            let mut transform = spatial.transform();
            transform.origin = Vector3{x:ZONE_WIDTH * (location.x as f32),y:0.0,z:ZONE_WIDTH * (location.y as f32)};
            spatial.set_transform(transform);
        });
        
    }
    #[method]
    fn add_op_to_menus(&self,ability_id:u8,amount:i64){
        for zone in self.zones.values(){
            let zone = unsafe{zone.assume_safe()};
            let _ = zone.map_mut(|obj,_| obj.add_op_to_menu(ability_id,amount));
        }
    }
    #[method]
    fn remove_op_from_menus(&self,ability_id:u8,amount:i64){
        for zone in self.zones.values(){
            let zone = unsafe{zone.assume_safe()};
            let _ = zone.map_mut(|obj,_| obj.remove_op_from_menu(ability_id,amount));
        }
    }

    #[method]
    fn clear_all_operations(&self){
        for zone in self.zones.values(){
            let zone = unsafe{zone.assume_safe()};
            let _ = zone.map_mut(|obj,_| obj.clear_operations());
        }
    }
    #[method]
    fn add_field_ability(&self,#[base] _owner:TRef<Spatial>,ability_id:u8,location:(i64,i64)){
        let (x,y) = location;
        let location = Location{x:x,y:y};
        if !self.zones.contains_key(&location){return ;}
        let zone = self.zones.get(&location).unwrap();
        let zone = unsafe{zone.assume_safe()};
        let _ = zone.map_mut(|obj,body|{
            obj.place_ability(body,ability_id)
        });
    }
    #[method]
    fn hide(&self,#[base] _owner:TRef<Spatial>){
        for zone in self.zones.values(){
            let zone = unsafe{zone.assume_safe()};
            let _ = zone.map(|obj,body| obj.hide(body));
        }
    }
    #[method]
    fn show(&self,#[base] _owner:TRef<Spatial>){
        for zone in self.zones.values(){
            let zone = unsafe{zone.assume_safe()};
            let _ = zone.map(|obj,body| obj.show(body));
        }
    }
    #[method]
    fn toggle(&self,#[base] _owner:TRef<Spatial>){
        for zone in self.zones.values(){
            let zone = unsafe{zone.assume_safe()};
            let _ = zone.map(|obj,body| obj.toggle(body));
        }
    }
}

#[derive(NativeClass)]
#[inherit(StaticBody)]
pub struct FieldOp3D{
    typ:AbilityType,
    mesh:Instance<FieldAbilityMesh>,
    highlight_left:Ref<MeshInstance>,
    highlight_right:Ref<MeshInstance>,
    radius:f64,
    width:f32,
    field_tx:Option<Sender<FieldZoneCommand>>,

}
impl InstancedDefault<StaticBody,AbilityType> for FieldOp3D{
    fn make(args:&AbilityType) -> Self{
        FieldOp3D{
            typ:*args,
            mesh:FieldAbilityMesh::make_instance(args).into_shared(),
            highlight_left:MeshInstance::new().into_shared(),
            highlight_right:MeshInstance::new().into_shared(),
            radius:5.0,
            width:12.5,
            field_tx:None
        }
    }
}
#[methods]
impl FieldOp3D{
    #[method]
    fn _ready(&self,#[base] owner:TRef<StaticBody>){
        let mesh = unsafe{self.mesh.assume_safe()};
        let _ = mesh.map(|_,spatial| owner.add_child(spatial,true));

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
        left_transform.origin = Vector3{x:-self.width,y:0.0,z:0.0};
        right_transform.origin = Vector3{x:self.width,y:0.0,z:0.0};
        highlight_left.set_transform(left_transform);
        highlight_right.set_transform(right_transform);
        highlight_left.set_visible(false);
        highlight_right.set_visible(false);

        let collision_shape = BoxShape::new().into_shared();
        let collision_shape = unsafe{collision_shape.assume_safe()};
        let collision_object = CollisionShape::new().into_shared();
        let collision_object = unsafe{collision_object.assume_safe()};
        collision_shape.set_extents(Vector3{x:self.width,y:(self.radius/2.0)as f32,z:(self.radius/2.0) as f32});
        collision_object.set_shape(collision_shape);

        owner.set_collision_layer_bit(0,false);
        owner.set_collision_mask_bit(0,false);

        owner.add_child(mesh,true);
        owner.add_child(collision_object,true);
        owner.add_child(highlight_left,true);
        owner.add_child(highlight_right,true);
    }
    #[method]
    fn clicked(&self,#[base] _owner:TRef<StaticBody>,_event_position:Vector2,_intersect_position:Vector3){
        let _ = self.field_tx
            .as_ref()
            .expect("field_tx not set")
            .send(FieldZoneCommand::Selected(self.typ));
    }
    #[method]
    fn entered(&self,#[base] _owner:TRef<StaticBody>){
        let highlight_left = unsafe{self.highlight_left.assume_safe()};
        let highlight_right = unsafe{self.highlight_right.assume_safe()};
        highlight_left.set_visible(true);
        highlight_right.set_visible(true);
    }
    #[method]
    fn exited(&self,#[base] _owner:TRef<StaticBody>){
        let highlight_left = unsafe{self.highlight_left.assume_safe()};
        let highlight_right = unsafe{self.highlight_right.assume_safe()};
        highlight_left.set_visible(false);
        highlight_right.set_visible(false);
    }
    fn set_tx(&mut self,tx:Sender<FieldZoneCommand>){
        self.field_tx = Some(tx);
    }
    #[method]
    fn set_height_idx(&self,#[base] owner:TRef<StaticBody>,idx:u64,radius:f32){
        let mut transform = owner.transform();
        transform.origin = Vector3{x:0.0,y: radius + (idx as f32 * radius),z:0.0};
        owner.set_transform(transform);
    }
}
#[derive(NativeClass)]
#[inherit(Spatial)]
pub struct FieldOps3D{
    operations:HashMap<AbilityType,Instance<FieldOp3D>>,
    op_count:HashMap<AbilityType,i64>,
    field_tx:Option<Sender<FieldZoneCommand>>,
    width:f32,
}
impl Instanced<Spatial> for FieldOps3D{
    fn make() -> Self{
        FieldOps3D{
            operations:HashMap::new(),
            op_count:HashMap::new(),
            field_tx:None,
            width:12.5,
        }
    }
}
#[methods]
impl FieldOps3D{
    #[method]
    fn _ready(&self,#[base] _owner:TRef<Spatial>){ }
    
    #[method]
    fn add_op(&mut self,#[base] owner:TRef<Spatial>,typ:u8,amount:i64){
        let typ = AbilityType::from(typ);

        self.op_count.insert(typ, self.op_count.get(&typ).map(|x|*x).unwrap_or(0) + amount);
        if self.operations.contains_key(&typ){
            return ;
        }

        let op = FieldOp3D::make_instance(&typ).into_shared();
        let num_ops = self.operations.len();
        self.operations.insert(typ,op.clone());
        let op = unsafe{op.assume_safe()};
        let _ = op.map(|obj,body| {
           obj.set_height_idx(body,num_ops as u64,obj.radius as f32); 
        });
        let _ = op.map_mut(|obj,_| self.field_tx.as_ref().map(|tx|obj.set_tx(tx.clone())));
        let _ = op.map_mut(|obj,_| obj.width = self.width);
        owner.add_child(op.clone(),true);
    }
    #[method]
    fn remove_op(&mut self,#[base] owner:TRef<Spatial>,typ:u8,amount:i64){
        let typ = AbilityType::from(typ);
        self.op_count.insert(typ,std::cmp::max(self.op_count.get(&typ).map(|x| *x).unwrap_or(0) - amount,0));
        if !self.operations.contains_key(&typ){return ;}
        if self.op_count.get(&typ).map(|x|*x).unwrap_or(0) > 0 { return ;}

        let op = self.operations.get(&typ).unwrap();
        let op = unsafe{op.assume_safe()};
        owner.remove_child(op.clone());
        let _ = op.map(|_,control| control.queue_free());
        self.operations.remove(&typ);
        let mut idx:u64 = 0;
        for r_op in self.operations.values(){
            let r_op = unsafe{r_op.assume_safe()};
            let _ = r_op.map(|obj,body| obj.set_height_idx(body,idx,obj.radius as f32));
            idx += 1;
        }
    }
    #[method]
    fn show(&self, #[base] owner:TRef<Spatial>){
        for op in self.operations.values(){
            let op = unsafe{op.assume_safe()};
            let _ = op.map(|_,body| {
                body.set_collision_layer_bit(COLLISION_LAYER,true);
                body.set_collision_mask_bit(COLLISION_LAYER,true);
            });
        }
        owner.set_visible(true);
    }
    #[method]
    fn clear(&mut self,#[base] owner:TRef<Spatial>){
        for op in self.operations.clone().values(){
            let op = unsafe{op.assume_safe()};
            let _ = op.map(|_,control| {
                owner.remove_child(control.clone());
                control.queue_free();
            });
            self.operations.clear();
        }
    }
    #[method]
    fn hide(&self, #[base] owner:TRef<Spatial>){
        for op in self.operations.values(){
            let op = unsafe{op.assume_safe()};
            let _ = op.map(|_,body| {
                body.set_collision_layer_bit(COLLISION_LAYER,false);
                body.set_collision_mask_bit(COLLISION_LAYER,false);
            });
        }
        owner.set_visible(false);
    }
    #[method]
    fn toggle(&self, #[base] owner:TRef<Spatial>){
        owner.set_visible(!owner.is_visible());
        for op in self.operations.values(){
            let op = unsafe{op.assume_safe()};
            let _ = op.map(|_,body| {
                body.set_collision_layer_bit(COLLISION_LAYER,owner.is_visible());
                body.set_collision_mask_bit(COLLISION_LAYER,owner.is_visible());
            });
        }
    }
    fn set_tx(&mut self,tx:Sender<FieldZoneCommand>){
        self.field_tx = Some(tx.clone());
        for op in self.operations.values(){
            let op = unsafe{op.assume_safe()};
            let _ = op.map_mut(|obj,_|obj.set_tx(tx.clone()));
        }
    }
}
#[derive(NativeClass)]
#[inherit(Control)]
pub struct FieldOp{
    typ:AbilityType,
    label:Ref<Label>,
    bg_rect:Ref<ColorRect>,
    main_rect:Ref<ColorRect>,
    bg_color:Color,
    color:Color,
}
impl InstancedDefault<Control,AbilityType> for FieldOp{
    fn make(args:&AbilityType) -> Self{
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
        label.set_text(self.typ.to_string());

        //owner.add_child(bg_rect,true);
        owner.add_child(main_rect,true);
        owner.add_child(label,true);
    }

    #[method]
    fn _process(&self,#[base] owner:TRef<Control>,_delta:f64){
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
    fn _ready(&self,#[base] _owner:TRef<Control>){

    }
    #[method]
    fn add_op(&mut self,#[base] owner:TRef<Control>,typ:u8){
        let op = FieldOp::make_instance(&AbilityType::from(typ)).into_shared();
        owner.add_child(op.clone(),true);
        self.operations.push(op);
    }
    #[method]
    fn _process(&self,#[base] owner:TRef<Control> , _delta:f64){
        let owner_size = owner.size(); 
        let num_ops = self.operations.len() as f32;

        //let op_size = owner_size / (num_ops * ((num_ops > 0.0) as i32 as f32) + 1.0 * ((num_ops == 0.0) as i32 as f32));
        let op_size = owner_size/num_ops;
        for op in &self.operations{
            let op = unsafe{op.assume_safe()};
            let mut idx = 0.0;
            let _ = op.map(|_,control| {
                control.set_size(op_size,false);
                control.set_position(op_size * idx ,false);
            });
            idx += 1.0;
        }

    }
}
