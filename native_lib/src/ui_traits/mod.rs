
use std::collections::HashMap;
use gdnative::prelude::*;
use gdnative::api::*;
use crate::traits::{CreateSignal,Instanced,InstancedDefault,Defaulted};
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
pub enum Action{
    clicked,
    hover,
    unhover,
}
pub trait Windowed<T:From<Action>>{
    const bg_highlight_color:Color;
    const bg_color:Color;
    const main_color:Color;
    const margin_size:f32;
    fn hovering(&self) -> bool;
    fn set_hovering(&mut self,value:bool);
    fn tx(&self) -> &Sender<T>;

    fn bg_rect(&self) -> &Ref<ColorRect>;
    fn main_rect(&self) -> &Ref<ColorRect>;

    fn from_command(&self,cmd:Action) -> T{
        T::from(cmd)
    }

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
                self.tx().send(self.from_command(Action::clicked));
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
        self.tx().send(self.from_command(Action::hover));
        self.set_hovering(true);
    }
    fn unhover(&mut self){
        let bg_rect = unsafe{self.bg_rect().assume_safe()};
        bg_rect.set_frame_color(Self::bg_color);
        self.tx().send(self.from_command(Action::unhover));
        self.set_hovering(false);
    }

}
pub trait LabelButton<T:From<Action>> where Self:Windowed<T>{
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
