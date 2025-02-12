
use std::collections::HashMap;
use gdnative::prelude::*;
use gdnative::api::*;
use crate::traits::{CreateSignal,Instanced,InstancedDefault,Defaulted};
use crate::field_ability_mesh::{FieldAbilityMesh,ToMesh};
use crate::field_ability_actions::ToAction;
use crate::field_abilities::{AbilityType};
use crate::ui_traits::{Windowed,LabelButton,Action};
use tokio::sync::mpsc;

type Sender<T> = mpsc::UnboundedSender<T>;
type Receiver<T> = mpsc::UnboundedReceiver<T>;

enum InventoryAction{
    clicked(AbilityType),
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
#[register_with(Self::register_signals)]
pub struct InventorySlot{
    typ:AbilityType,
    bg_rect:Ref<ColorRect>,
    main_rect:Ref<ColorRect>,
    label:Ref<Label>,
    tx:Sender<InventoryAction>,
    hovering:bool,
}
impl InstancedDefault<Control,Sender<InventoryAction>> for InventorySlot{
    fn make(args:&Sender<InventoryAction>) -> Self{
        InventorySlot{
            typ:AbilityType::empty,
            bg_rect:ColorRect::new().into_shared(),
            main_rect:ColorRect::new().into_shared(),
            label:Label::new().into_shared(),
            tx:args.clone(),
            hovering:false,
        }
    }
}
impl Windowed<InventoryAction> for InventorySlot{
    const bg_highlight_color:Color = Color{r:255.0,g:255.0,b:255.0,a:1.0};
    const bg_color:Color = Color{r:0.0,g:0.0,b:0.0,a:1.0};
    const main_color:Color = Color{r:0.0,g:0.0,b:0.0,a:1.0};
    const margin_size:f32 = 5.0;
    fn from_command(&self,cmd:Action) -> InventoryAction{
        match cmd{
            Action::clicked => InventoryAction::clicked(self.typ),
            _ => InventoryAction::unhandled,
        }
    }
    fn hovering(&self) -> bool {self.hovering}
    fn set_hovering(&mut self,value:bool){self.hovering = value}
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
        owner.connect("mouse_entered",owner,"entered",VariantArray::new_shared(),0);
        owner.connect("mouse_exited",owner,"exited",VariantArray::new_shared(),0);
        owner.connect("clicked",owner,"clicked",VariantArray::new_shared(),0);
    }
    #[method]
    fn _process(&self,#[base] owner:TRef<Control>,delta:f64){
        <Self as LabelButton<InventoryAction>>::process(self,owner,delta);
    }
    #[method]
    fn _input(&self,#[base] owner:TRef<Control>,event:Ref<InputEvent>){
        <Self as Windowed<InventoryAction>>::input(self,owner,event.clone());
        let event = unsafe{event.assume_safe()};
        if event.is_action_released("inventory_toggle",true){
            owner.set_visible(!owner.is_visible());
        }
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
    fn set_type(&mut self,typ:u8){
        let label = unsafe{self.label.assume_safe()};
        let typ = AbilityType::from(typ);
        self.set_text(typ.to_string());
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
}
/////////OPERATIONS//////////////////////////////////
pub enum OperationType{
    empty,
}
impl From<u8> for OperationType {
    fn from(item:u8) -> OperationType{
        match item{
            255 => OperationType::empty,
            _ => todo!(),
        }
    }
}
#[derive(NativeClass)]
#[inherit(Control)]
#[register_with(Self::register_signals)]
pub struct OperationButton {
    typ:OperationType,
    bg_rect:Ref<ColorRect>,
    main_rect:Ref<ColorRect>,
    label:Ref<Label>,
    tx:Sender<Action>,
    hovering:bool,
}
impl InstancedDefault<Control,Sender<Action>> for OperationButton{
    fn make(args:&Sender<Action>) -> Self{
        OperationButton{
            typ:OperationType::empty,
            bg_rect:ColorRect::new().into_shared(),
            main_rect:ColorRect::new().into_shared(),
            label:Label::new().into_shared(),
            tx:args.clone(),
            hovering:false,
        }
    }
}
impl Windowed<Action> for OperationButton{
    const bg_highlight_color:Color = Color{r:255.0,g:255.0,b:255.0,a:1.0};
    const bg_color:Color = Color{r:0.0,g:0.0,b:10.0,a:1.0};
    const main_color:Color = Color{r:0.0,g:0.0,b:0.0,a:1.0};
    const margin_size:f32 = 5.0;
    fn hovering(&self) -> bool {self.hovering}
    fn set_hovering(&mut self,value:bool){self.hovering = value}
    fn tx(&self) -> &Sender<Action> {&self.tx}
    fn bg_rect(&self) -> &Ref<ColorRect>{&self.bg_rect}
    fn main_rect(&self) -> &Ref<ColorRect>{&self.main_rect}
}
impl LabelButton<Action> for OperationButton{
    fn label(&self) -> &Ref<Label>{&self.label}
}
#[methods]
impl OperationButton{
    fn register_signals(builder:&ClassBuilder<Self>){
        builder.signal("clicked").done();
    }
    #[method]
    fn _ready(&self,#[base] owner:TRef<Control>){
        <Self as LabelButton<Action>>::ready(self,owner);
        owner.connect("mouse_entered",owner,"entered",VariantArray::new_shared(),0);
        owner.connect("mouse_exited",owner,"exited",VariantArray::new_shared(),0);
        owner.connect("clicked",owner,"clicked",VariantArray::new_shared(),0);
    }
    #[method]
    fn _process(&self,#[base] owner:TRef<Control>,delta:f64){
        <Self as LabelButton<Action>>::process(self,owner,delta);
    }
    #[method]
    fn _input(&self,#[base] owner:TRef<Control>,event:Ref<InputEvent>){
        <Self as Windowed<Action>>::input(self,owner,event.clone());
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
        godot_print!("Operations clicked!");
    }
    #[method]
    fn set_type(&mut self,typ:u8){
        let label = unsafe{self.label.assume_safe()};
        let typ = OperationType::from(typ);
        self.set_text("test".to_string());
        self.typ = typ;
    }
}
////OPERATIONS MENU///////////////////////
#[derive(NativeClass)]
#[inherit(Control)]
#[register_with(Self::register_signals)]
pub struct InventoryOperations {
    typ:AbilityType,
    bg_rect:Ref<ColorRect>,
    main_rect:Ref<ColorRect>,
    place_button:Instance<OperationButton>,
    tx:Sender<InventoryAction>,
    buttons_tx:Sender<Action>,
    buttons_rx:Receiver<Action>,
    hovering:bool,
}
impl InstancedDefault<Control,Sender<InventoryAction>> for InventoryOperations{
    fn make(args:&Sender<InventoryAction>) -> Self{
        let (tx,rx) = mpsc::unbounded_channel::<Action>() ;
        InventoryOperations{
            typ:AbilityType::empty,
            bg_rect:ColorRect::new().into_shared(),
            main_rect:ColorRect::new().into_shared(),
            place_button:OperationButton::make_instance(&tx).into_shared(),
            tx:args.clone(),
            buttons_tx:tx,
            buttons_rx:rx,
            hovering:false,
        }
    }
}
impl Windowed<InventoryAction> for InventoryOperations{
    const bg_highlight_color:Color = Color{r:255.0,g:255.0,b:255.0,a:1.0};
    const bg_color:Color = Color{r:0.0,g:0.0,b:0.0,a:1.0};
    const main_color:Color = Color{r:255.0,g:0.0,b:255.0,a:1.0};
    const margin_size:f32 = 5.0;
    fn from_command(&self,cmd:Action) -> InventoryAction{
        match cmd{
            _ => InventoryAction::unhandled,
        }
    }
    fn hovering(&self) -> bool {self.hovering}
    fn set_hovering(&mut self,value:bool){self.hovering = value}
    fn tx(&self) -> &Sender<InventoryAction> {&self.tx}
    fn bg_rect(&self) -> &Ref<ColorRect>{&self.bg_rect}
    fn main_rect(&self) -> &Ref<ColorRect>{&self.main_rect}
}
#[methods]
impl InventoryOperations{
    fn register_signals(builder:&ClassBuilder<Self>){
        builder.signal("clicked").done();
    }
    #[method]
    fn _ready(&self,#[base] owner:TRef<Control>){
        <Self as Windowed<InventoryAction>>::ready(self,owner);
        owner.connect("mouse_entered",owner,"entered",VariantArray::new_shared(),0);
        owner.connect("mouse_exited",owner,"exited",VariantArray::new_shared(),0);
        owner.connect("clicked",owner,"clicked",VariantArray::new_shared(),0);
        owner.add_child(self.place_button.clone(),true);
    }
    #[method]
    fn _process(&self,#[base] owner:TRef<Control>,delta:f64){
        <Self as Windowed<InventoryAction>>::process(self,owner,delta);
        let place_button = unsafe{self.place_button.assume_safe()};
        place_button.map(|_,control| control.set_size(owner.size()/4.0,false));
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
        godot_print!("Operations clicked!");
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
}
////INVENTORY MENU///////////////////
#[derive(NativeClass)]
#[inherit(Control)]
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
    #[method]
    fn _ready(&mut self,#[base] owner:TRef<Control>){
        self.add_slot(owner,AbilityType::empty.into());
        self.add_slot(owner,AbilityType::empty.into());
        self.add_slot(owner,AbilityType::empty.into());
        self.add_slot(owner,AbilityType::empty.into());
        self.add_slot(owner,AbilityType::empty.into());
        self.add_slot(owner,AbilityType::empty.into());
        self.add_slot(owner,AbilityType::empty.into());
        self.add_slot(owner,AbilityType::empty.into());
        self.add_slot(owner,AbilityType::empty.into());
        self.add_slot(owner,AbilityType::empty.into());
        owner.add_child(self.operations.clone(),true);
    }
    #[method]
    fn _process(&mut self,#[base] owner:TRef<Control>,delta:f64){
        match self.rx.try_recv(){
            Ok(InventoryAction::clicked(typ)) => {
                self.fill_slot(owner,0);
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
            slot.map(|_,control|control.set_size(slot_size_v,false));
            slot.map(|_,control|
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
        operations.map(|_,control| control.set_size(owner.size()/2.0,false));
    }
    #[method]
    fn _input(&self,#[base] owner:TRef<Control>,event:Ref<InputEvent>){
        let event = unsafe{event.assume_safe()};
        if event.is_action_released("inventory_toggle",true){
            owner.set_visible(!owner.is_visible());
        }
    }
    #[method]
    fn add_slot(&mut self ,#[base] owner:TRef<Control>,typ:u8){
        let slot = InventorySlot::make_instance(&self.tx).into_shared();
        self.slots.push(slot.clone());
        let slot = unsafe{slot.assume_safe()};
        slot.map_mut(|obj,_| obj.set_type(typ));
        owner.add_child(slot,true);
    }
    #[method]
    fn fill_slot(&mut self,#[base] owner:TRef<Control>, typ:u8){
        let mut slots = unsafe{self.slots.clone().into_iter().map(|x| x.assume_safe())};
        let empty_slot = slots.find(|x| x.map(|obj,_| obj.is_empty()).unwrap());
        empty_slot.map(|slot| slot.map_mut(|obj,_| obj.set_type(typ)));
    }
    #[method]
    fn fill_client_slot(&mut self,#[base] owner:TRef<Control>,id:String, typ:u8){
        self.fill_slot(owner,typ);
    }
    #[method]
    fn remove_item(&mut self,#[base] owner:TRef<Control>, typ:u8){
        let mut slots = unsafe{self.slots.clone().into_iter().map(|x| x.assume_safe())};
        let item_slot = slots.find(|x| x.map(|obj,_| obj.is_type(typ)).unwrap());
        item_slot.map(|slot| slot.map_mut(|obj,_| obj.set_type(AbilityType::empty.into())));
    }
    #[method]
    fn remove_client_item(&mut self,#[base] owner:TRef<Control>,id:String, typ:u8){
        self.remove_item(owner,typ);
    }
    #[method]
    fn clear(&mut self,#[base] owner:TRef<Control>){
        for slot in &self.slots{
            let slot = unsafe{slot.assume_safe()};
            slot.map_mut(|obj,_| obj.set_type(AbilityType::empty.into()));
        }
    }

}



trait ToColor{
    fn to_color(&self) -> Color;
}
impl ToColor for AbilityType{
    fn to_color(&self) -> Color{
        match self{
            AbilityType::smack => Color{r:255.0,g:255.0,b:0.0,a:1.0},
            AbilityType::globular_teleport => Color{r:100.0,g:0.0,b:30.0,a:1.0},
            AbilityType::empty => Color{r:0.0,g:0.0,b:0.0,a:1.0},

        }
    }
}



pub enum InventorySlotCommand{
    Hovered,
    Unhovered,
    Clicked,
    Error,
}
