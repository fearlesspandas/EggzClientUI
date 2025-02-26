
use std::collections::HashMap;
use gdnative::prelude::*;
use gdnative::api::*;
use crate::traits::{CreateSignal,Instanced,InstancedDefault,Defaulted};
use crate::field_ability_mesh::{FieldAbilityMesh,ToMesh};
use crate::field_ability_actions::ToAction;
use crate::field_abilities::{AbilityType};
use crate::ui_traits::{AnimationWindow,Windowed,LabelButton,Action,Centering,TileButton};
use crate::button_tiles::{TileType,Tile};
use tokio::sync::mpsc;
use rand::Rng;

type Sender<T> = mpsc::UnboundedSender<T>;
type Receiver<T> = mpsc::UnboundedReceiver<T>;

enum InventoryAction{
    clicked(AbilityType,i32),
    pocketed(AbilityType,i32),
    unpocketed(AbilityType,i32),
    hover(AbilityType),
    unhover,
    unhandled,
}
impl From<Action> for InventoryAction{
    fn from(item:Action) -> InventoryAction{
        match item{
            _ => InventoryAction::unhandled,
        }
    }
}
////INVENTORY SLOTS///////////////
#[derive(NativeClass)]
#[inherit(Control)]
//#[register_with(Self::register_signals)]
pub struct SlotAmount{
    bg_rect:Ref<ColorRect>,
    main_rect:Ref<ColorRect>,
    label:Ref<Label>,
    tx:Sender<InventoryAction>,
    hovering:bool,
}
impl InstancedDefault<Control,Sender<InventoryAction>> for SlotAmount{
    fn make(args:&Sender<InventoryAction>) -> Self{
        SlotAmount{
            bg_rect:ColorRect::new().into_shared(),
            main_rect:ColorRect::new().into_shared(),
            label:Label::new().into_shared(),
            tx:args.clone(),
            hovering:false,
        }
    }
}
impl Windowed<InventoryAction> for SlotAmount{
    const bg_highlight_color:Color = Color{r:255.0,g:255.0,b:255.0,a:1.0};
    const bg_color:Color = Color{r:255.0,g:255.0,b:255.0,a:1.0};
    const main_color:Color = Color{r:255.0,g:0.0,b:0.0,a:1.0};
    const margin_size:f32 = 5.0;
    fn hovering(&self) -> bool {self.hovering}
    fn set_hovering(&mut self,value:bool){self.hovering = value}
    fn centering(&self) -> Centering {Centering::center}
    fn tx(&self) -> &Sender<InventoryAction> {&self.tx}
    fn bg_rect(&self) -> &Ref<ColorRect>{&self.bg_rect}
    fn main_rect(&self) -> &Ref<ColorRect>{&self.main_rect}
}
impl LabelButton<InventoryAction> for SlotAmount{
    fn label(&self) -> &Ref<Label>{&self.label}
}
#[methods]
impl SlotAmount{
    #[method]
    fn _ready(&self,#[base] owner:TRef<Control>){
        <Self as LabelButton<InventoryAction>>::ready(self,owner);
    }
    #[method]
    fn _process(&self,#[base] owner:TRef<Control>,delta:f64){
        <Self as LabelButton<InventoryAction>>::process(self,owner,delta);
    }
    #[method]
    fn _input(&self,#[base] owner:TRef<Control>,event:Ref<InputEvent>){
        <Self as Windowed<InventoryAction>>::input(self,owner,event.clone());
    }
    #[method]
    fn set_amount(&self,amount:i32){
        self.set_text(amount.to_string());
    }
}
#[derive(NativeClass)]
#[inherit(Control)]
#[register_with(Self::register_signals)]
pub struct InventorySlot{
    typ:AbilityType,
    amount:i32,
    bg_rect:Ref<ColorRect>,
    main_rect:Ref<ColorRect>,
    label:Ref<Label>,
    tx:Sender<InventoryAction>,
    hovering:bool,
    id:i32,
    amount_display:Instance<SlotAmount>,
    color_mode:u8,
    dynamic_color:Color,
    tick:u64,
}
impl InstancedDefault<Control,Sender<InventoryAction>> for InventorySlot{
    fn make(args:&Sender<InventoryAction>) -> Self{
        InventorySlot{
            typ:AbilityType::empty,
            amount:0,
            bg_rect:ColorRect::new().into_shared(),
            main_rect:ColorRect::new().into_shared(),
            label:Label::new().into_shared(),
            tx:args.clone(),
            hovering:false,
            id:0,
            amount_display:SlotAmount::make_instance(args).into_shared(),
            color_mode:0,
            dynamic_color:Color{r:0.0,g:0.0,b:0.0,a:1.0},
            tick:0,
        }
    }
}
impl Windowed<InventoryAction> for InventorySlot{
    const bg_highlight_color:Color = Color{r:255.0,g:255.0,b:255.0,a:1.0};
    const bg_color:Color = Color{r:0.0,g:255.0,b:255.0,a:1.0};
    const main_color:Color = Color{r:0.0,g:0.0,b:0.0,a:1.0};
    const margin_size:f32 = 5.0;
    fn from_command(&self,cmd:Action) -> InventoryAction{
        match cmd{
            Action::clicked => InventoryAction::clicked(self.typ,self.id),
            _ => InventoryAction::unhandled,
        }
    }
    fn hovering(&self) -> bool {self.hovering}
    fn set_hovering(&mut self,value:bool){self.hovering = value}
    fn centering(&self) -> Centering {Centering::center}
    fn tx(&self) -> &Sender<InventoryAction> {&self.tx}
    fn bg_rect(&self) -> &Ref<ColorRect>{&self.bg_rect}
    fn main_rect(&self) -> &Ref<ColorRect>{&self.main_rect}
}
impl LabelButton<InventoryAction> for InventorySlot{
    fn label(&self) -> &Ref<Label>{&self.label}
}
#[methods]
impl InventorySlot{
    fn register_signals(builder:&ClassBuilder<Self>){
        builder.signal("clicked").done();
    }
    #[method]
    fn _ready(&self,#[base] owner:TRef<Control>){
        <Self as LabelButton<InventoryAction>>::ready(self,owner);
        let _ = owner.connect("mouse_entered",owner,"entered",VariantArray::new_shared(),0);
        let _ = owner.connect("mouse_exited",owner,"exited",VariantArray::new_shared(),0);
        let _ = owner.connect("clicked",owner,"clicked",VariantArray::new_shared(),0);
        let amount_disp = unsafe{self.amount_display.assume_safe()};
        let _ = amount_disp.map(|_,control|control.set_mouse_filter(control::MouseFilter::IGNORE.into()));
        owner.add_child(amount_disp,true);
    }
    #[method]
    fn _process(&mut self,#[base] owner:TRef<Control>,delta:f64){
        <Self as LabelButton<InventoryAction>>::process(self,owner,delta);
        let amount_disp = unsafe{self.amount_display.assume_safe()};
        let _ = amount_disp.map(|_,control| control.set_size(owner.size()/3.0,false));
        let _ = amount_disp.map(|_,control| control.set_position(owner.size() - control.size(),false));
    }
    #[method]
    fn _input(&self,#[base] owner:TRef<Control>,event:Ref<InputEvent>){
        <Self as Windowed<InventoryAction>>::input(self,owner,event.clone());
    }
    #[method]
    fn entered(&mut self){
        self.hover();
    }
    #[method]
    fn exited(&mut self){
           self.unhover();
    }
    #[method]
    fn clicked(&self) {
        godot_print!("Inventory Menu Clicked!");
    }
    #[method]
    fn set_id(&mut self,id:i32){
        self.id = id;
    }
    #[method]
    fn set_type(&mut self,typ:u8){
        let label = unsafe{self.label.assume_safe()};
        let typ = AbilityType::from(typ);
        self.set_text(typ.to_string());
        self.typ = typ;
    }
    #[method]
    fn set_amount(&mut self,amount:i32){
        self.amount = amount;
        let amount_disp = unsafe{self.amount_display.assume_safe()};
        let _ = amount_disp.map_mut(|obj,_| obj.set_amount(self.amount));
    }
    #[method]
    fn add_amount(&mut self,amount:i32){
        self.amount += amount;
        let amount_disp = unsafe{self.amount_display.assume_safe()};
        let _ = amount_disp.map_mut(|obj,_| obj.set_amount(self.amount));
        
    }
    #[method]
    fn is_empty(&self) -> bool{
        self.typ == AbilityType::empty
    }
    #[method]
    fn is_type(&self,typ:u8) -> bool{
        self.typ == AbilityType::from(typ)
    }
}
/////////OPERATIONS//////////////////////////////////
#[derive(Copy,Clone)]
pub enum OperationType{
    empty,
    place,
    remove,
}
impl ToString for OperationType{
    fn to_string(&self) -> String{
        match self {
            OperationType::empty => "".to_string(),
            OperationType::place => "+".to_string(),
            OperationType::remove => "-".to_string(),
        }
    }
}
impl From<Action> for OperationType{
    fn from(item:Action) -> Self{
        OperationType::empty
    }
}
impl From<u8> for OperationType {
    fn from(item:u8) -> OperationType{
        match item{
            255 => OperationType::empty,
            0 => OperationType::place,
            1 => OperationType::remove,
            _ => todo!(),
        }
    }
}
impl Into<TileType> for OperationType{
    fn into(self) -> TileType{
        match self{
            OperationType::empty => TileType::empty,
            OperationType::place => TileType::down_arrow,
            OperationType::remove => TileType::up_arrow,
            _ => TileType::empty,
        }
    }
}
impl Into<u8> for OperationType{
    fn into(self) -> u8{
        match self{
            OperationType::place => 0,
            OperationType::remove => 1,
            OperationType::empty => 255,
        }
    }
}
type OP_BUTTON = OperationType;
#[derive(NativeClass)]
#[inherit(Control)]
#[register_with(Self::register_signals)]
pub struct OperationButton{
    typ:OperationType,
    bg_rect:Ref<ColorRect>,
    main_rect:Ref<ColorRect>,
    label:Ref<Label>,
    tile:Instance<Tile>,
    tx:Sender<OP_BUTTON>,
    hovering:bool,
    centering:Centering,
}
impl InstancedDefault<Control,Sender<OP_BUTTON>> for OperationButton{
    fn make(args:&Sender<OP_BUTTON>) -> Self{
        OperationButton{
            typ:OperationType::empty,
            bg_rect:ColorRect::new().into_shared(),
            main_rect:ColorRect::new().into_shared(),
            label:Label::new().into_shared(),
            tile:Tile::make_instance().into_shared(),
            tx:args.clone(),
            hovering:false,
            centering:Centering::center,
        }
    }
}
impl Windowed<OP_BUTTON> for OperationButton{
    const bg_highlight_color:Color = Color{r:255.0,g:255.0,b:255.0,a:1.0};
    const bg_color:Color = Color{r:0.0,g:50.0,b:50.0,a:1.0};
    const main_color:Color = Color{r:0.0,g:0.0,b:0.0,a:1.0};
    const margin_size:f32 = 5.0;
    fn from_command(&self,cmd:Action) -> OP_BUTTON{
        match cmd{
            Action::clicked => self.typ,
            _ => OP_BUTTON::empty,
        }
        
    }
    fn hovering(&self) -> bool {self.hovering}
    fn set_hovering(&mut self,value:bool){self.hovering = value}
    fn centering(&self) -> Centering {self.centering.clone()}
    fn tx(&self) -> &Sender<OP_BUTTON> {&self.tx}
    fn bg_rect(&self) -> &Ref<ColorRect>{&self.bg_rect}
    fn main_rect(&self) -> &Ref<ColorRect>{&self.main_rect}
}
impl LabelButton<OP_BUTTON> for OperationButton{
    fn label(&self) -> &Ref<Label>{&self.label}
}
impl TileButton<OP_BUTTON> for OperationButton{
    fn tile(&self) -> &Instance<Tile>{&self.tile}
}
#[methods]
impl OperationButton{
    fn register_signals(builder:&ClassBuilder<Self>){
        builder.signal("clicked").done();
    }
    #[method]
    fn _ready(&self,#[base] owner:TRef<Control>){
        <Self as TileButton<OP_BUTTON>>::ready(self,owner);
        let _ = owner.connect("mouse_entered",owner,"entered",VariantArray::new_shared(),0);
        let _ = owner.connect("mouse_exited",owner,"exited",VariantArray::new_shared(),0);
        let _ = owner.connect("clicked",owner,"clicked",VariantArray::new_shared(),0);
        self.set_tile(self.typ.into());

    }
    #[method]
    fn _process(&self,#[base] owner:TRef<Control>,delta:f64){
        <Self as TileButton<OP_BUTTON>>::process(self,owner,delta);
    }
    #[method]
    fn _input(&self,#[base] owner:TRef<Control>,event:Ref<InputEvent>){
        <Self as Windowed<OP_BUTTON>>::input(self,owner,event.clone());
    }
    #[method]
    fn entered(&mut self){
        self.hover();
        self.hover_symbol();
    }
    #[method]
    fn exited(&mut self){
        self.unhover();
        self.unhover_symbol();
    }
    #[method]
    fn clicked(&self) {
    }
    #[method]
    fn set_centering(&mut self,centering:Centering){
        self.centering = centering;
    }
    fn set_type(&mut self,typ:u8){
        let typ = OperationType::from(typ);
        self.set_text(typ.to_string());
        self.set_tile(typ.into());
        self.typ = typ;
    }
}
////OPERATIONS MENU///////////////////////
#[derive(NativeClass)]
#[inherit(Control)]
#[register_with(Self::register_signals)]
pub struct InventoryOperations {
    typ:AbilityType,
    position:i32,
    bg_rect:Ref<ColorRect>,
    main_rect:Ref<ColorRect>,
    place_button:Instance<OperationButton>,
    remove_button:Instance<OperationButton>,
    tx:Sender<InventoryAction>,
    buttons_tx:Sender<OP_BUTTON>,
    buttons_rx:Receiver<OP_BUTTON>,
    hovering:bool,
}
impl InstancedDefault<Control,Sender<InventoryAction>> for InventoryOperations{
    fn make(args:&Sender<InventoryAction>) -> Self{
        let (tx,rx) = mpsc::unbounded_channel::<OP_BUTTON>() ;
        InventoryOperations{
            typ:AbilityType::empty,
            position:0,
            bg_rect:ColorRect::new().into_shared(),
            main_rect:ColorRect::new().into_shared(),
            place_button:OperationButton::make_instance(&tx).into_shared(),
            remove_button:OperationButton::make_instance(&tx).into_shared(),
            tx:args.clone(),
            buttons_tx:tx,
            buttons_rx:rx,
            hovering:false,
        }
    }
}
impl Windowed<InventoryAction> for InventoryOperations{
    const bg_highlight_color:Color = Color{r:255.0,g:255.0,b:255.0,a:1.0};
    const bg_color:Color = Color{r:0.0,g:0.0,b:0.0,a:0.5};
    const main_color:Color = Color{r:255.0,g:0.0,b:255.0,a:0.5};
    const margin_size:f32 = 5.0;
    fn from_command(&self,cmd:Action) -> InventoryAction{
        match cmd{
            _ => InventoryAction::unhandled,
        }
    }
    fn hovering(&self) -> bool {self.hovering}
    fn centering(&self) -> Centering {Centering::center}
    fn set_hovering(&mut self,value:bool){self.hovering = value}
    fn tx(&self) -> &Sender<InventoryAction> {&self.tx}
    fn bg_rect(&self) -> &Ref<ColorRect>{&self.bg_rect}
    fn main_rect(&self) -> &Ref<ColorRect>{&self.main_rect}
}
impl AnimationWindow<InventoryAction> for InventoryOperations{
    fn animation(start:Vector2,delta:f64) -> Vector2{
        todo!()
    }
    fn shapes(&self) -> Vec<Ref<Control>>{
        let num_shapes = 10;
        let mut vec = Vec::new();
        let rect = Control::new().into_shared();
        vec.push(rect.clone());
        let rect = unsafe{rect.assume_safe()};
        //rect.translate(todo!());
        vec
    }
}
#[methods]
impl InventoryOperations{
    fn register_signals(builder:&ClassBuilder<Self>){
        builder.signal("clicked").done();
    }
    #[method]
    fn _ready(&self,#[base] owner:TRef<Control>){
        <Self as Windowed<InventoryAction>>::ready(self,owner);
        let _ = owner.connect("mouse_entered",owner,"entered",VariantArray::new_shared(),0);
        let _ = owner.connect("mouse_exited",owner,"exited",VariantArray::new_shared(),0);
        let _ = owner.connect("clicked",owner,"clicked",VariantArray::new_shared(),0);
        let place_button = unsafe{self.place_button.assume_safe()};
        owner.add_child(place_button.clone(),true);
        let _ = place_button.map_mut(|obj,_| obj.set_type(OperationType::place.into()));
        let _ = place_button.map_mut(|obj,_| obj.set_centering(Centering::top_left));
        let remove_button = unsafe{self.remove_button.assume_safe()};
        owner.add_child(remove_button.clone(),true);
        let _ = remove_button.map_mut(|obj,_| obj.set_type(OperationType::remove.into()));
        let _ = remove_button.map_mut(|obj,_| obj.set_centering(Centering::top_right));
    }
    #[method]
    fn _process(&mut self,#[base] owner:TRef<Control>,delta:f64){
        <Self as Windowed<InventoryAction>>::process(self,owner,delta);
        let place_button = unsafe{self.place_button.assume_safe()};
        let button_size = owner.size()/4.0;
        let _ = place_button.map(|_,control| control.set_size(button_size,false));
        let remove_button = unsafe{self.remove_button.assume_safe()};
        let _ = remove_button.map(|_,control| control.set_size(button_size,false));
        let _ = remove_button.map(|_,control| control.set_position(Vector2{x:owner.size().x - button_size.x,y:0.0},false));
        match self.buttons_rx.try_recv(){
            Ok(OperationType::place) => {
                let _ = self.tx.send(InventoryAction::pocketed(self.typ,self.position));
            }
            Ok(OperationType::remove) => {
                let _ = self.tx.send(InventoryAction::unpocketed(self.typ,self.position));
            }
            Ok(_) => {}
            Err(_) => {}
        }
    }
    #[method]
    fn _input(&self,#[base] owner:TRef<Control>,event:Ref<InputEvent>){
        <Self as Windowed<InventoryAction>>::input(self,owner,event.clone());
    }
    #[method]
    fn entered(&mut self){
        self.hover();
    }
    #[method]
    fn exited(&mut self){
        self.unhover();
    }
    #[method]
    fn clicked(&self,#[base] owner:TRef<Control>) {
        owner.set_visible(false);
    }
    #[method]
    fn set_type(&mut self,typ:u8){
        let typ = AbilityType::from(typ);
        self.typ = typ;
    }
    #[method]
    fn is_empty(&self) -> bool{
        self.typ == AbilityType::empty
    }
    #[method]
    fn is_type(&self,typ:u8) -> bool{
        self.typ == AbilityType::from(typ)
    }
    #[method]
    fn set_place_button_visible(&self,visible:bool){
        let place_button = unsafe{self.place_button.assume_safe()};
        let _ = place_button.map(|_,control| control.set_visible(visible));
    }
    #[method]
    fn set_remove_button_visible(&self,visible:bool){
        let remove_button = unsafe{self.remove_button.assume_safe()};
        let _ = remove_button.map(|_,control| control.set_visible(visible));
    }
}
////INVENTORY MENU///////////////////
#[derive(NativeClass)]
#[inherit(Control)]
#[register_with(Self::register_signals)]
pub struct InventoryMenu{
    slots:Vec<Instance<InventorySlot>>,
    operations:Instance<InventoryOperations>,
    tx:Sender<InventoryAction>,
    rx:Receiver<InventoryAction>,
}
impl Instanced<Control> for InventoryMenu{
    fn make() -> Self{
        let (tx,rx) = mpsc::unbounded_channel::<InventoryAction>();
        InventoryMenu{
            slots:Vec::new(),
            operations:InventoryOperations::make_instance(&tx).into_shared(),
            tx:tx,
            rx:rx,
        }
    }
}
#[methods]
impl InventoryMenu{
    fn register_signals(builder:&ClassBuilder<Self>){
        builder
            .signal("pocketed")
            .with_param("type",VariantType::I64)
            .with_param("amount",VariantType::I64)
            .done();
        builder
            .signal("unpocketed")
            .with_param("type",VariantType::I64)
            .with_param("amount",VariantType::I64)
            .done();
    }
    #[method]
    fn _ready(&mut self,#[base] owner:TRef<Control>){
        self.add_slot(owner,AbilityType::empty.into(),10);
        self.add_slot(owner,AbilityType::empty.into(),10);
        self.add_slot(owner,AbilityType::empty.into(),10);
        self.add_slot(owner,AbilityType::empty.into(),10);
        self.add_slot(owner,AbilityType::empty.into(),10);
        self.add_slot(owner,AbilityType::empty.into(),10);
        self.add_slot(owner,AbilityType::empty.into(),10);
        self.add_slot(owner,AbilityType::empty.into(),10);
        self.add_slot(owner,AbilityType::empty.into(),10);
        self.add_slot(owner,AbilityType::empty.into(),10);
        self.add_slot(owner,AbilityType::empty.into(),10);
        self.add_slot(owner,AbilityType::empty.into(),10);

        owner.add_child(self.operations.clone(),true);
        owner.set_visible(false);
        let operations = unsafe{self.operations.assume_safe()};
        let _ = operations.map(|_,control| control.set_visible(false));


    }
    #[method]
    fn _process(&mut self,#[base] owner:TRef<Control>,delta:f64){
        match self.rx.try_recv(){
            Ok(InventoryAction::clicked(typ,id)) => {
                let operations = unsafe{self.operations.assume_safe()};
                let slot = &self.slots[id as usize];
                let slot = unsafe{slot.assume_safe()};
                
                let _ = slot
                    .map(|_,control|control.position())
                    .map(|position| operations.map(|_,op_control| op_control.set_position(position,false)));
                let _ = operations.map(|_,control| control.set_visible(true));
                let _ = operations.map_mut(|obj,_| obj.position = id);
                let _ = operations.map_mut(|obj,_| obj.set_type(typ.into()));
            }
            Ok(InventoryAction::pocketed(typ,id)) => {
                owner.emit_signal("pocketed",&[Variant::new(typ),Variant::new(1)]);
                godot_print!("{}",format!("pocketed typ:{typ:?}, id:{id:?}"));
            }
            Ok(InventoryAction::unpocketed(typ,id)) => {
                owner.emit_signal("unpocketed",&[Variant::new(typ),Variant::new(1)]);
                godot_print!("Item unPocketed");
            }
            Ok(_) => {}
            Err(_) => {}
        }
        let vp = owner.get_viewport().unwrap();
        let vp  = unsafe{vp.assume_safe()};
        let size = vp.get_visible_rect().size;
        owner.set_size(size/2.0,false);
        let slot_size = 100.0;
        let slot_size_v = Vector2{x:slot_size,y:slot_size};
        let num_slots = self.slots.len() as f32;
        let max_per_row = (owner.size().x / slot_size).floor();
        let mut idx = 0.0;
        for slot in &self.slots{
            let slot = unsafe{slot.assume_safe()};
            let _ = slot.map(|_,control|control.set_size(slot_size_v,false));
            let _ = slot.map(|_,control|
                control.set_position(
                    Vector2{
                        x:slot_size * (idx%max_per_row),
                        y:slot_size * (idx/max_per_row).floor()
                    },
                    false
                )
            );
            idx += 1.0;
        }
        let operations = unsafe{self.operations.assume_safe()};
        let _ = operations.map(|_,control| control.set_size(slot_size_v,false));
    }
    #[method]
    fn _input(&self,#[base] owner:TRef<Control>,event:Ref<InputEvent>){
        let event = unsafe{event.assume_safe()};
        if event.is_action_released("inventory_toggle",true){
            owner.set_visible(!owner.is_visible());
        }
    }
    #[method]
    fn add_slot(&mut self ,#[base] owner:TRef<Control>,typ:u8,amount:i32){
        let slot = InventorySlot::make_instance(&self.tx).into_shared();
        let id = self.slots.len() as i32;
        self.slots.push(slot.clone());
        let slot = unsafe{slot.assume_safe()};
        let _ = slot.map_mut(|obj,_| obj.set_type(typ));
        let _ = slot.map_mut(|obj,_| obj.set_id(id));
        let _ = slot.map_mut(|obj,_| obj.set_amount(amount));
        owner.add_child(slot,true);
    }
    #[method]
    fn fill_slot(&mut self,#[base] owner:TRef<Control>, typ:u8,amount:i32){
        let mut slots = unsafe{self.slots.clone().into_iter().map(|x| x.assume_safe())};
        let empty_slot = slots.find(|x| x.map(|obj,_| obj.is_empty() || obj.typ == AbilityType::from(typ)).unwrap());
        let _ = empty_slot.map(|slot| {
            let _ = slot.map_mut(|obj,_| obj.set_type(typ));
            let _ = slot.map_mut(|obj,_| obj.add_amount(amount));
        });
    }
    #[method]
    fn fill_client_slot(&mut self,#[base] owner:TRef<Control>,id:String, typ:u8,amount:i32){
        self.fill_slot(owner,typ,amount);
    }
    #[method]
    fn remove_item(&mut self,#[base] owner:TRef<Control>, typ:u8,amount:i32){
        assert!(false,"removing item fail");
        let mut slots = unsafe{self.slots.clone().into_iter().map(|x| x.assume_safe())};
        let item_slot = slots.find(|x| x.map(|obj,_| obj.is_type(typ)).unwrap());
        item_slot.map(|slot| slot.map_mut(|obj,_| {
                    let new_amount = obj.amount - amount;
                    match new_amount{
                        x if x <= 0 => obj.set_type(AbilityType::empty.into()),
                        _ => {}
                    };
                    obj.set_amount(new_amount);
                }));
    }
    #[method]
    fn remove_client_item(&mut self,#[base] owner:TRef<Control>,id:String, typ:u8,amount:i32){
        self.remove_item(owner,typ,amount);
    }
    #[method]
    fn clear(&mut self,#[base] owner:TRef<Control>){
        for slot in &self.slots{
            let slot = unsafe{slot.assume_safe()};
            let _ = slot.map_mut(|obj,_| obj.set_type(AbilityType::empty.into()));
            let _ = slot.map_mut(|obj,_| obj.set_amount(0));
        }
    }
}


#[derive(NativeClass)]
#[inherit(Control)]
#[register_with(Self::register_signals)]
pub struct Pocket{
    slots:Vec<Instance<InventorySlot>>,
    operations:Instance<InventoryOperations>,
    tx:Sender<InventoryAction>,
    rx:Receiver<InventoryAction>,
}
impl Instanced<Control> for Pocket{
    fn make() -> Self{
        let (tx,rx) = mpsc::unbounded_channel::<InventoryAction>();
        Pocket{
            slots:Vec::new(),
            operations:InventoryOperations::make_instance(&tx).into_shared(),
            tx:tx,
            rx:rx,
        }
    }
}
#[methods]
impl Pocket{
    fn register_signals(builder:&ClassBuilder<Self>){
        builder
            .signal("unpocketed")
            .with_param("type",VariantType::I64)
            .with_param("amount",VariantType::I64)
            .done();
    }
    #[method]
    fn _ready(&mut self,#[base] owner:TRef<Control>){
        self.add_slot(owner,AbilityType::empty.into(),0);
        self.add_slot(owner,AbilityType::empty.into(),0);
        self.add_slot(owner,AbilityType::empty.into(),0);
        self.add_slot(owner,AbilityType::empty.into(),0);
        self.add_slot(owner,AbilityType::empty.into(),0);
        owner.add_child(self.operations.clone(),true);
        let operations = unsafe{self.operations.assume_safe()};
        let _ = operations.map(|_,control| control.set_visible(false));
        let _ = operations.map(|obj,_| obj.set_place_button_visible(false));
    }
    #[method]
    fn _process(&mut self,#[base] owner:TRef<Control>,delta:f64){
        match self.rx.try_recv(){
            Ok(InventoryAction::clicked(typ,id)) => {
                let operations = unsafe{self.operations.assume_safe()};
                let slot = &self.slots[id as usize];
                let slot = unsafe{slot.assume_safe()};
                
                let _ = slot
                    .map(|_,control|control.position())
                    .map(|position| operations.map(|_,op_control| op_control.set_position(position,false)));
                let _ = operations.map(|_,control| control.set_visible(true));
                let _ = operations.map_mut(|obj,_| obj.position = id);
                let _ = operations.map_mut(|obj,_| obj.set_type(typ.into()));
            }
            Ok(InventoryAction::pocketed(typ,id)) => { }
            Ok(InventoryAction::unpocketed(typ,id)) => {
                owner.emit_signal("unpocketed",&[Variant::new(typ),Variant::new(1)]);
                godot_print!("Item unPocketed");
            }
            Ok(_) => {}
            Err(_) => {}
        }
        let num_slots = self.slots.len() as f32;

        let slot_size = 100.0;
        let slot_size_v = Vector2{x:slot_size,y:slot_size};
        let size = OS::godot_singleton().window_size();
        let self_size = Vector2{x:slot_size * num_slots,y:slot_size + 20.0};
        owner.set_size(self_size,false);
        let mut position = size/2.0 - self_size/2.0;
        position.x = size.x/2.0 - self_size.x/2.0;
        position.y = size.y - self_size.y - 20.0;
        owner.set_position(position,false);

        let max_per_row = (owner.size().x / slot_size).floor();
        let mut idx = 0.0;
        for slot in &self.slots{
            let slot = unsafe{slot.assume_safe()};
            let _ = slot.map(|_,control|control.set_size(slot_size_v,false));
            let _ = slot.map(|_,control|
                control.set_position(
                    Vector2{
                        x:slot_size * (idx%max_per_row),
                        y:slot_size * (idx/max_per_row).floor()
                    },
                    false
                )
            );
            idx += 1.0;
        }
        let operations = unsafe{self.operations.assume_safe()};
        let _ = operations.map(|_,control| control.set_size(slot_size_v,false));
    }
    #[method]
    fn _input(&self,#[base] owner:TRef<Control>,event:Ref<InputEvent>){
        let event = unsafe{event.assume_safe()};
        if event.is_action_released("inventory_toggle",true){
            //owner.set_visible(!owner.is_visible());
        }
    }
    #[method]
    fn add_slot(&mut self ,#[base] owner:TRef<Control>,typ:u8,amount:i32){
        let slot = InventorySlot::make_instance(&self.tx).into_shared();
        let id = self.slots.len() as i32;
        self.slots.push(slot.clone());
        let slot = unsafe{slot.assume_safe()};
        let _ = slot.map_mut(|obj,_| obj.set_type(typ));
        let _ = slot.map_mut(|obj,_| obj.set_id(id));
        let _ = slot.map_mut(|obj,_| obj.set_amount(amount));
        owner.add_child(slot,true);
    }
    #[method]
    fn fill_slot(&mut self,#[base] owner:TRef<Control>, typ:u8,amount:i32){
        let mut slots = unsafe{self.slots.clone().into_iter().map(|x| x.assume_safe())};
        let empty_slot = slots.find(|x| x.map(|obj,_| obj.is_empty() || obj.typ == AbilityType::from(typ)).unwrap());
        empty_slot.map(|slot| {
            let _ = slot.map_mut(|obj,_| obj.set_type(typ));
            let _ = slot.map_mut(|obj,_| obj.add_amount(amount));
        });
    }
    #[method]
    fn fill_client_slot(&mut self,#[base] owner:TRef<Control>,id:String, typ:u8,amount:i32){
        self.fill_slot(owner,typ,amount);
    }
    #[method]
    fn remove_item(&mut self,#[base] owner:TRef<Control>, typ:u8,amount:i32){
        let mut slots = unsafe{self.slots.clone().into_iter().map(|x| x.assume_safe())};
        let item_slot = slots.find(|x| x.map(|obj,_| obj.is_type(typ)).unwrap());
        item_slot.map(|slot| slot.map_mut(|obj,_| {
                    let new_amount = obj.amount - amount;
                    match new_amount{
                        x if x <= 0 => obj.set_type(AbilityType::empty.into()),
                        _ => {}
                    };
                    obj.set_amount(new_amount);
                }));
    }
    #[method]
    fn remove_client_item(&mut self,#[base] owner:TRef<Control>,id:String, typ:u8,amount:i32){
        self.remove_item(owner,typ,amount);
    }
    #[method]
    fn clear(&mut self,#[base] owner:TRef<Control>){
        for slot in &self.slots{
            let slot = unsafe{slot.assume_safe()};
            let _ = slot.map_mut(|obj,_| obj.set_type(AbilityType::empty.into()));
            let _ = slot.map_mut(|obj,_| obj.set_amount(0));
        }
    }
}



pub enum InventorySlotCommand{
    Hovered,
    Unhovered,
    Clicked,
    Error,
}
