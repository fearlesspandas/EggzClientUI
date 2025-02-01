
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
pub enum BoxCommand{
    Hovered,
    Unhovered,
    Clicked,
    Error(String),
}
impl From<u8> for BoxCommand{
    fn from(item:u8) -> Self{
        match item{
            0 => BoxCommand::Hovered,
            1 => BoxCommand::Unhovered,
            2 => BoxCommand::Clicked,
            _ => BoxCommand::Error("unique id not set".to_string())
        }
    }
}
impl Into<u8> for BoxCommand{
    fn into(self) -> u8{
        match self{
            BoxCommand::Hovered => 0,
            BoxCommand::Unhovered => 1,
            BoxCommand::Clicked => 2,
            BoxCommand::Error(_) => 255,

        }
    }
}
#[derive(NativeClass)]
#[inherit(Control)]
#[register_with(Self::register_signals)]
pub struct ControlBox<T>{
    bg_rect:Ref<ColorRect>,
    main_rect:Ref<ColorRect>,
    external_tx:Sender<T>,
    hovering:bool,
    bg_color:Color,
    main_color:Color,
    bg_highlight_color:Color,
    margin_size:f32,
}
impl <T> InstancedDefault<Control,Sender<T>> for ControlBox<T>{
    fn make(args:&Sender<T>) -> Self{
        ControlBox{
            bg_rect:ColorRect::new().into_shared(),
            main_rect:ColorRect::new().into_shared(),
            external_tx:args.clone(),
            hovering:false,
            bg_color:Color{r:0.0,g:0.0,b:0.0,a:1.0},
            main_color:Color{r:0.0,g:0.0,b:0.0,a:1.0},
            bg_highlight_color:Color{r:255.0,g:255.0,b:255.0,a:1.0},
            margin_size:5.0,
        }
    }

}
impl <T: 'static> ControlBox<T>{
    fn register_signals(builder:&ClassBuilder<Self>){
        builder.signal("clicked").done();
        builder.signal("hovered").done();
        builder.signal("unhovered").done();
    }
}
#[methods]
impl <T: From<u8> + Into<u8> + 'static> ControlBox<T>{
    #[method]
    fn _ready(&self,#[base] owner:TRef<Control>){
        let bg_rect = unsafe{self.bg_rect.assume_safe()};
        let main_rect = unsafe{self.main_rect.assume_safe()};
        bg_rect.set_mouse_filter(control::MouseFilter::IGNORE.into());
        main_rect.set_mouse_filter(control::MouseFilter::IGNORE.into());
        owner.add_child(self.bg_rect,true);
        owner.add_child(self.main_rect,true);
        owner.connect("mouse_entered",owner,"hover",VariantArray::new_shared(),0);
        owner.connect("mouse_exited",owner,"unhover",VariantArray::new_shared(),0);
    }
    #[method]
    fn _input(&self,#[base] owner:TRef<Control>,event:Ref<InputEvent>){
        if let Ok(event) = event.try_cast::<InputEventMouseButton>(){
            let event = unsafe{event.assume_safe()};
            if event.is_action_released("left_click",true) && self.hovering{
                owner.emit_signal("clicked",&[]);
                self.external_tx.send(T::from(BoxCommand::Clicked.into()));
            }
        }
    }
    #[method]
    fn _process(&self,#[base] owner:TRef<Control>,delta:f64){
        let bg_rect = unsafe{self.bg_rect.assume_safe()};
        let main_rect = unsafe{self.main_rect.assume_safe()};
        let mut size = owner.size();
        bg_rect.set_size(size,false);
        size.x -= self.margin_size;
        size.y -= self.margin_size;
        main_rect.set_size(size,false);
        main_rect.set_position(Vector2{x:self.margin_size/2.0,y:self.margin_size/2.0},false);
    }
    #[method]
    fn set_bg_color(&mut self,color:Color) -> Result<(),Error>{
        self.bg_color = color;
        let bg_rect = unsafe{self.bg_rect.assume_safe()};
        Ok(bg_rect.set_frame_color(color))
    }
    #[method]
    fn set_main_color(&mut self,color:Color) -> Result<(),Error>{
        self.main_color = color;
        let main_rect = unsafe{self.main_rect.assume_safe()};
        Ok(main_rect.set_frame_color(color))
    }
    #[method]
    fn set_bg_highlight_color(&mut self,color:Color) -> Result<(),Error>{
        self.bg_highlight_color = color;
        Ok(())
    }
    #[method]
    fn hover(&mut self){
        self.hovering = true;
        let bg_rect = unsafe{self.bg_rect.assume_safe()};
        bg_rect.set_frame_color(self.bg_highlight_color);
        //owner.emit_signal("hovered",&[]);
        self.external_tx.send(T::from(BoxCommand::Hovered.into()));
    }
    #[method]
    fn unhover(&mut self){
        self.hovering = true;
        let bg_rect = unsafe{self.bg_rect.assume_safe()};
        bg_rect.set_frame_color(self.bg_color);
        //owner.emit_signal("unhovered",&[]);
        self.external_tx.send(T::from(BoxCommand::Unhovered.into()));
    }
}
pub trait IntoBox<T:'static >{
    fn into_box(&self) -> Instance<ControlBox<T>>;
}
//////Instances//////////
#[derive(NativeClass,Clone)]
#[inherit(Control)]
pub struct LabelButton<T:'static>{
    control_box:Instance<ControlBox<T>>,
    label:Ref<Label>,
}
impl <T> InstancedDefault<Control,Sender<T>> for LabelButton<T>{
    fn make(args:&Sender<T>) -> Self{
        LabelButton{
            control_box:ControlBox::<T>::make_instance(args).into_shared(),
            label:Label::new().into_shared(),
        }
    }
}
impl <T> IntoBox<T> for LabelButton<T>{
    fn into_box(&self) -> Instance<ControlBox<T>>{
        self.control_box.clone()
    }
}
#[methods]
impl <T: From<u8> + Into<u8> + 'static> LabelButton<T>{
    #[method]
    fn _ready(&self, #[base] owner:TRef<Control>){
        let label = unsafe{self.label.assume_safe()};
        owner.add_child(self.control_box.clone(),true);
        owner.add_child(label.clone(),true);
        label.set_mouse_filter(control::MouseFilter::IGNORE.into());
    }
    #[method]
    fn _process(&self, #[base] owner:TRef<Control>,delta:f64){
        let control_box = unsafe{self.control_box.assume_safe()};
        let label = unsafe{self.label.assume_safe()};

        let owner_size = owner.size();
        control_box.map(|obj,control| control.set_size(owner_size,true));
        label.set_size(owner_size/2.0,true);
        let label_position = owner_size/2.0 - (label.size()/2.0) ;
        label.set_position(label_position,true);
    }
    #[method]
    fn set_text(&self,text:String){
        let label = unsafe{self.label.assume_safe()};
        label.set_text(text);
    }
}
#[derive(NativeClass)]
#[inherit(Control)]
pub struct InventorySlot<T:'static>{
    button:Instance<LabelButton<T>>,
}
impl <T:'static> InstancedDefault<Control,Sender<T>> for InventorySlot<T>{
    fn make(args:&Sender<T>) -> Self{
        InventorySlot{
            button:LabelButton::make_instance(args).into_shared(),
        }
    }
}
impl <T> IntoBox<T> for InventorySlot<T>{
    fn into_box(&self) -> Instance<ControlBox<T>>{
        let button = unsafe{self.button.assume_safe()};
        button.map(|obj,_| obj.into_box()).unwrap()
    }
}
#[methods]
impl <T:'static + From<u8> + Into<u8>> InventorySlot<T>{
    #[method]
    fn _ready(&self,#[base] owner:TRef<Control>){
        owner.add_child(self.button.clone(),true);
    }
    #[method]
    fn _process(&self,#[base] owner:TRef<Control>, delta:f64){
        let button = unsafe{self.button.assume_safe()};
        button.map(|_,control| control.set_size(owner.size(),false));
    }
}

pub enum InventorySlotCommand{
    Hovered,
    Unhovered,
    Clicked,
    Error,
}
impl From<u8> for InventorySlotCommand{
    fn from(item:u8) -> Self{
        match item{
            x if x == Into::<u8>::into(BoxCommand::Hovered) => InventorySlotCommand::Hovered,
            x if x == Into::<u8>::into(BoxCommand::Unhovered) => InventorySlotCommand::Unhovered,
            x if x == Into::<u8>::into(BoxCommand::Clicked) => InventorySlotCommand::Clicked,
            _ => InventorySlotCommand::Error
        }
    }
}
impl Into<u8> for InventorySlotCommand{
    fn into(self) -> u8{
        match self{
            InventorySlotCommand::Hovered => BoxCommand::Hovered.into(),
            InventorySlotCommand::Unhovered => BoxCommand::Unhovered.into(),
            InventorySlotCommand::Clicked => BoxCommand::Clicked.into(),
            InventorySlotCommand::Error => BoxCommand::Error("".to_string()).into(),
        }
    }
}
