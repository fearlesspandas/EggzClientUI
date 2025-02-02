
use std::collections::HashMap;
use gdnative::prelude::*;
use gdnative::api::*;
use crate::traits::{CreateSignal,Instanced,InstancedDefault,Defaulted};
use crate::field_ability_mesh::{FieldAbilityMesh,ToMesh};
use crate::field_ability_actions::ToAction;
use crate::field_abilities::{OpType,SubOpType};
use tokio::sync::mpsc;

enum Error{
    set_bg_color_error,
    set_main_color_error,
}
const set_bg_color_error:u8 = 0;
const set_main_color_error:u8 = 1;
impl ToVariant for Error{
    fn to_variant(&self) -> Variant{
        match self{
            Error::set_bg_color_error => Variant::new(set_bg_color_error),
            Error::set_main_color_error => Variant::new(set_main_color_error),
        }
    }
}
type Sender<T> = mpsc::UnboundedSender<T>;
type Receiver<T> = mpsc::UnboundedReceiver<T>;
pub enum Command{
    clicked,
    hover,
    unhover,
}
pub trait Windowed<T:From<Command>>{
    const bg_highlight_color:Color;
    const bg_color:Color;
    const main_color:Color;
    const margin_size:f32;
    fn hovering(&self) -> bool;
    fn set_hovering(&mut self,value:bool);
    fn tx(&self) -> &Sender<T>;

    fn bg_rect(&self) -> &Ref<ColorRect>;
    fn main_rect(&self) -> &Ref<ColorRect>;
    fn ready(&self,owner:TRef<Control>){
        let bg_rect = unsafe{self.bg_rect().assume_safe()};
        let main_rect = unsafe{self.main_rect().assume_safe()};
        bg_rect.set_frame_color(Self::bg_color);
        main_rect.set_frame_color(Self::main_color);
        bg_rect.set_mouse_filter(control::MouseFilter::IGNORE.into());
        main_rect.set_mouse_filter(control::MouseFilter::IGNORE.into());
        owner.add_child(bg_rect,true);
        owner.add_child(main_rect,true);
    }
    fn input(&self,owner:TRef<Control>,event:Ref<InputEvent>){
        if let Ok(event) = event.try_cast::<InputEventMouseButton>(){
            let event = unsafe{event.assume_safe()};
            if event.is_action_released("left_click",true) && self.hovering(){
                owner.emit_signal("clicked",&[]);
                self.tx().send(T::from(Command::clicked));
            }
        }
    }
    fn process(&self,owner:TRef<Control>,delta:f64){
        let bg_rect = unsafe{self.bg_rect().assume_safe()};
        let main_rect = unsafe{self.main_rect().assume_safe()};
        let mut size = owner.size();
        bg_rect.set_size(size,false);
        size.x -= Self::margin_size;
        size.y -= Self::margin_size;
        main_rect.set_size(size,false);
        main_rect.set_position(Vector2{x:Self::margin_size/2.0,y:Self::margin_size/2.0},false);
    }
    fn hover(&mut self){
        let bg_rect = unsafe{self.bg_rect().assume_safe()};
        bg_rect.set_frame_color(Self::bg_highlight_color);
        self.tx().send(T::from(Command::hover));
        self.set_hovering(true);
    }
    fn unhover(&mut self){
        let bg_rect = unsafe{self.bg_rect().assume_safe()};
        bg_rect.set_frame_color(Self::bg_color);
        self.tx().send(T::from(Command::unhover));
        self.set_hovering(false);
    }

}
trait LabelButton<T:From<Command>> where Self:Windowed<T>{
    fn label(&self) -> &Ref<Label>;
    fn ready(&self,owner:TRef<Control>){
        <Self as Windowed<T>>::ready(self,owner);
        let label = unsafe{self.label().assume_safe()};
        owner.add_child(self.label(),true);
        label.set_mouse_filter(control::MouseFilter::IGNORE.into());
    }
    fn process(&self,owner:TRef<Control>,delta:f64){
        <Self as Windowed<T>>::process(self,owner,delta);
        let label = unsafe{self.label().assume_safe()};
        let owner_size = owner.size();
        label.set_size(owner_size/2.0,true);
        let label_position = owner_size/2.0 - (label.size()/2.0) ;
        label.set_position(label_position,true);
    }
    fn set_text(&self,text:String){
        let label = unsafe{self.label().assume_safe()};
        label.set_text(text);
    }
}
#[derive(NativeClass)]
#[inherit(Control)]
#[register_with(Self::register_signals)]
pub struct InventorySlot{
    bg_rect:Ref<ColorRect>,
    main_rect:Ref<ColorRect>,
    label:Ref<Label>,
    tx:Sender<Command>,
    hovering:bool,
}
impl InstancedDefault<Control,Sender<Command>> for InventorySlot{
    fn make(args:&Sender<Command>) -> Self{
        InventorySlot{
            bg_rect:ColorRect::new().into_shared(),
            main_rect:ColorRect::new().into_shared(),
            label:Label::new().into_shared(),
            tx:args.clone(),
            hovering:false,
        }
    }
}
impl Windowed<Command> for InventorySlot{
    const bg_highlight_color:Color = Color{r:255.0,g:255.0,b:255.0,a:1.0};
    const bg_color:Color = Color{r:0.0,g:0.0,b:0.0,a:1.0};
    const main_color:Color = Color{r:255.0,g:255.0,b:255.0,a:1.0};
    const margin_size:f32 = 5.0;
    
    fn hovering(&self) -> bool {self.hovering}
    fn set_hovering(&mut self,value:bool){self.hovering = value}
    fn tx(&self) -> &Sender<Command> {&self.tx}
    fn bg_rect(&self) -> &Ref<ColorRect>{&self.bg_rect}
    fn main_rect(&self) -> &Ref<ColorRect>{&self.main_rect}
}
impl LabelButton<Command> for InventorySlot{
    fn label(&self) -> &Ref<Label>{&self.label}
}

#[methods]
impl InventorySlot{
    fn register_signals(builder:&ClassBuilder<Self>){
        builder.signal("clicked").done();
    }
    #[method]
    fn _ready(&self,#[base] owner:TRef<Control>){
        <Self as LabelButton<Command>>::ready(self,owner);
        owner.connect("mouse_entered",owner,"entered",VariantArray::new_shared(),0);
        owner.connect("mouse_exited",owner,"exited",VariantArray::new_shared(),0);
        owner.connect("clicked",owner,"clicked",VariantArray::new_shared(),0);
    }
    #[method]
    fn _process(&self,#[base] owner:TRef<Control>,delta:f64){
        <Self as LabelButton<Command>>::process(self,owner,delta);
    }
    #[method]
    fn _input(&self,#[base] owner:TRef<Control>,event:Ref<InputEvent>){
        <Self as Windowed<Command>>::input(self,owner,event.clone());
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
    fn set_type(&self,typ:u8){
        let label = unsafe{self.label.assume_safe()};
        let typ = OpType::from(typ);
        self.set_text(typ.to_string());
    }
}
trait ToColor{
    fn to_color(&self) -> Color;
}
impl ToColor for OpType{
    fn to_color(&self) -> Color{
        match self{
            OpType::smack => Color{r:255.0,g:255.0,b:0.0,a:1.0},
            OpType::globular_teleport => Color{r:100.0,g:0.0,b:30.0,a:1.0},
            OpType::empty => Color{r:0.0,g:0.0,b:0.0,a:1.0},

        }
    }
}
pub enum InventoryMenuCommand{
    Selected(OpType),
    Unhandled
}
#[derive(NativeClass)]
#[inherit(Control)]
pub struct InventoryMenu{
    slots:Vec<Instance<InventorySlot>>,
    tx:Sender<Command>,
    rx:Receiver<Command>,
}
impl Instanced<Control> for InventoryMenu{
    fn make() -> Self{
        let (tx,rx) = mpsc::unbounded_channel::<Command>();
        InventoryMenu{
            slots:Vec::new(),
            tx:tx,
            rx:rx,
        }
    }
}
#[methods]
impl InventoryMenu{
    #[method]
    fn _ready(&mut self,#[base] owner:TRef<Control>){
        self.add_slot(owner,OpType::smack.into());
        self.add_slot(owner,OpType::smack.into());
        self.add_slot(owner,OpType::smack.into());
        self.add_slot(owner,OpType::smack.into());
        self.add_slot(owner,OpType::smack.into());
        self.add_slot(owner,OpType::smack.into());
        self.add_slot(owner,OpType::smack.into());
        self.add_slot(owner,OpType::smack.into());
        self.add_slot(owner,OpType::smack.into());
        self.add_slot(owner,OpType::smack.into());
    }
    #[method]
    fn _process(&mut self,#[base] owner:TRef<Control>,delta:f64){
        match self.rx.try_recv(){
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
            slot.map(|_,control|control.set_position(Vector2{x:slot_size * (idx%max_per_row),y:slot_size * (idx/max_per_row).floor()},false));
            idx += 1.0;
        }
        
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
        slot.map(|obj,_| obj.set_type(typ));
        owner.add_child(slot,true);
    }
}






pub enum InventorySlotCommand{
    Hovered,
    Unhovered,
    Clicked,
    Error,
}
