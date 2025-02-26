
use gdnative::prelude::*;
use gdnative::api::*;
use crate::traits::{Instanced,InstancedDefault};
use crate::button_tiles::{Tile,TileType};
use tokio::sync::mpsc;

type Sender<T> = mpsc::UnboundedSender<T>;
pub enum Action{
    clicked,
    hover,
    unhover,
}
#[derive(Debug,Clone,Copy)]
pub enum Centering{
    top_right,
    top_left,
    bottom_right,
    bottom_left,
    center,
}
impl From<u8> for Centering{
    fn from(item:u8) -> Self{
        match item{
            0 => Centering::center,
            1 => Centering::top_right,
            2 => Centering::top_left,
            3 => Centering::bottom_left,
            4 => Centering::bottom_right,
            _ => todo!(),
        }
    }
}
impl FromVariant for Centering{
    fn from_variant(variant:&Variant) -> Result<Self,FromVariantError>{
        match variant.get_type(){
            VariantType::I64 => variant.try_to::<u8>().map(|typ| Centering::from(typ)),
            VariantType::F64 => variant.try_to::<f64>().map(|typ| Centering::from(typ.round() as i64 as u8)),
            typ => Err(FromVariantError::InvalidVariantType{variant_type:typ,expected:VariantType::I64}),
        }
    }
}
pub trait Windowed<T:From<Action>>{
    const BG_HIGHLIGHT_COLOR:Color;
    const BG_COLOR:Color;
    const MAIN_COLOR:Color;
    const MARGIN_SIZE:f32;
    fn hovering(&self) -> bool;
    fn set_hovering(&mut self,value:bool);
    fn tx(&self) -> &Sender<T>;

    fn bg_rect(&self) -> &Ref<ColorRect>;
    fn main_rect(&self) -> &Ref<ColorRect>;
    fn centering(&self) -> Centering;

    fn from_command(&self,cmd:Action) -> T{
        T::from(cmd)
    }

    fn ready(&self,owner:TRef<Control>){
        let bg_rect = unsafe{self.bg_rect().assume_safe()};
        let main_rect = unsafe{self.main_rect().assume_safe()};
        bg_rect.set_frame_color(Self::BG_COLOR);
        main_rect.set_frame_color(Self::MAIN_COLOR);
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
                let _ = self.tx().send(self.from_command(Action::clicked));
            }
        }
    }
    fn process(&self,owner:TRef<Control>,delta:f64){
        let bg_rect = unsafe{self.bg_rect().assume_safe()};
        let main_rect = unsafe{self.main_rect().assume_safe()};
        let mut size = owner.size();
        bg_rect.set_size(size,false);
        let position = match self.centering(){
            Centering::center => {
                size.x -= Self::MARGIN_SIZE;
                size.y -= Self::MARGIN_SIZE;
                Vector2{x:Self::MARGIN_SIZE/2.0,y:Self::MARGIN_SIZE/2.0}
            }
            Centering::top_left => { 
                size.x -= Self::MARGIN_SIZE/2.0;
                size.y -= Self::MARGIN_SIZE/2.0;
                Vector2{x:Self::MARGIN_SIZE,y:Self::MARGIN_SIZE} 
            }
            Centering::top_right => {
                size.x -= Self::MARGIN_SIZE/2.0;
                size.y -= Self::MARGIN_SIZE/2.0;
                Vector2{x:0.0,y:Self::MARGIN_SIZE} 
            }
            Centering::bottom_right => {
                size.x -= Self::MARGIN_SIZE/2.0;
                size.y -= Self::MARGIN_SIZE/2.0;
                Vector2{x:0.0,y:0.0} 
            }
            Centering::bottom_left => {
                size.x -= Self::MARGIN_SIZE/2.0;
                size.y -= Self::MARGIN_SIZE/2.0;
                Vector2{x:Self::MARGIN_SIZE,y:0.0} 
            }
        };
        size.x -= Self::MARGIN_SIZE/2.0;
        size.y -= Self::MARGIN_SIZE/2.0;
        main_rect.set_size(size,false);
        main_rect.set_position(position,false);
    }
    fn hover(&mut self){
        let bg_rect = unsafe{self.bg_rect().assume_safe()};
        bg_rect.set_frame_color(Self::BG_HIGHLIGHT_COLOR);
        let _ = self.tx().send(self.from_command(Action::hover));
        self.set_hovering(true);
    }
    fn unhover(&mut self){
        let bg_rect = unsafe{self.bg_rect().assume_safe()};
        bg_rect.set_frame_color(Self::BG_COLOR);
        let _ = self.tx().send(self.from_command(Action::unhover));
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
pub trait TileButton<T:From<Action>> where Self:Windowed<T>{
    const SYMBOL_COLOR:Color = Self::BG_COLOR;
    const SYMBOL_HIGHLIGHT_COLOR:Color = Self::BG_HIGHLIGHT_COLOR;
    fn tile(&self) -> &Instance<Tile>;
    fn ready(&self,owner:TRef<Control>){
        <Self as Windowed<T>>::ready(self,owner);
        let tile = unsafe{self.tile().assume_safe()};
        owner.add_child(self.tile(),true);
        let _ = tile.map(|_,control|control.set_mouse_filter(control::MouseFilter::IGNORE.into()));
        let _ = tile.map_mut(|obj,_|obj.set_color(Self::SYMBOL_COLOR));
        
    }
    fn process(&self,owner:TRef<Control>,delta:f64){
        <Self as Windowed<T>>::process(self,owner,delta);
        let tile = unsafe{self.tile().assume_safe()};
        let owner_size = owner.size();
        let main_rect = unsafe{self.main_rect().assume_safe()};
        let tile_size = main_rect.size();
        let tile_position =  main_rect.position();
        let _ = tile.map(|_,control|control.set_size(tile_size,false));
        let _ = tile.map(|_,control|control.set_position(tile_position,false));
    }
    fn set_tile(&self,typ:TileType){
        let tile = unsafe{self.tile().assume_safe()};
        let _ = tile.map_mut(|obj,_|obj.set_type(typ));
    }
    fn hover_symbol(&self){
        let tile = unsafe{self.tile().assume_safe()};
        let _ = tile.map_mut(|obj,_|obj.set_color(Self::SYMBOL_HIGHLIGHT_COLOR));
    }
    fn unhover_symbol(&self){
        let tile = unsafe{self.tile().assume_safe()};
        let _ = tile.map_mut(|obj,_|obj.set_color(Self::SYMBOL_COLOR));
    }
}
pub trait AnimationWindow<T:From<Action>> where Self:Windowed<T>{
    fn animation(start:Vector2,delta:f64) -> Vector2;
    fn shapes(&self) -> Vec<Ref<Control>>;
    fn ready(&self,owner:TRef<Control>){
        <Self as Windowed<T>>::ready(self,owner);
        for shape in &self.shapes(){
            owner.add_child(shape,true);
            owner.move_child(shape,0);
        }
    }
    fn process(&self,owner:TRef<Control>,delta:f64){
        <Self as Windowed<T>>::process(self,owner,delta);
        for shape in &self.shapes(){
            let shape = unsafe{shape.assume_safe()};
            let mut position = Self::animation(shape.position(),delta);
            position.x = position.x % (owner.position().x + owner.size().x); 
            position.y = position.y % (owner.position().y + owner.size().y);
            shape.set_position(position,false);
        }

    }
    
}
