
use gdnative::prelude::*;
use gdnative::api::*;
use crate::traits::{Instanced,InstancedDefault};
use crate::field_abilities::{AbilityType};
use crate::ui_traits::{AnimationWindow,Windowed,LabelButton,Action,Centering,TileButton};
use crate::button_tiles::{TileType,Tile};
use tokio::sync::mpsc;

type Sender<T> = mpsc::UnboundedSender<T>;
type Receiver<T> = mpsc::UnboundedReceiver<T>;


#[derive(NativeClass)]
#[inherit(Control)]
pub struct ServerStats{
    bg_rect:Ref<ColorRect>,
    main_rect:Ref<ColorRect>,
    label:Ref<Label>,
    desc_label:Instance<HoverStats>,
    tx:Sender<Action>,
    hovering:bool,
}
impl InstancedDefault<Control,Sender<Action>> for ServerStats{
    fn make(args:&Sender<Action>) -> Self{
        ServerStats{
            bg_rect:ColorRect::new().into_shared(),
            main_rect:ColorRect::new().into_shared(),
            label:Label::new().into_shared(),
            desc_label:HoverStats::make_instance().into_shared(),
            tx:args.clone(),
            hovering:false,
        }
    }
}
impl Windowed<Action> for ServerStats{
    const BG_HIGHLIGHT_COLOR:Color = Color{r:255.0,g:255.0,b:255.0,a:1.0};
    const BG_COLOR:Color = Color{r:255.0,g:255.0,b:255.0,a:1.0};
    const MAIN_COLOR:Color = Color{r:255.0,g:0.0,b:0.0,a:1.0};
    const MARGIN_SIZE:f32 = 5.0;
    fn hovering(&self) -> bool {self.hovering}
    fn set_hovering(&mut self,value:bool){self.hovering = value}
    fn centering(&self) -> Centering {Centering::center}
    fn tx(&self) -> &Sender<Action> {&self.tx}
    fn bg_rect(&self) -> &Ref<ColorRect>{&self.bg_rect}
    fn main_rect(&self) -> &Ref<ColorRect>{&self.main_rect}
}
impl LabelButton<Action> for ServerStats{
    fn label(&self) -> &Ref<Label>{&self.label}
}
#[methods]
impl ServerStats{
    #[method]
    fn _ready(&self,#[base] owner:TRef<Control>){
        <Self as LabelButton<Action>>::ready(self,owner);
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
    fn set_amount(&self,amount:i32){
        self.set_text(amount.to_string());
    }
}
#[derive(NativeClass)]
#[inherit(Control)]
pub struct HoverStats{
    
}
impl Instanced<Control> for HoverStats{
    fn make() -> Self{
        HoverStats{}
    }
}
