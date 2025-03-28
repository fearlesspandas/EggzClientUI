
use gdnative::prelude::*;
use gdnative::api::*;
use crate::traits::{Defaulted,Instanced,InstancedDefault};
use crate::field_abilities::{AbilityType};
use crate::ui_traits::{AnimationWindow,Windowed,LabelButton,Action,Centering,TileButton};
use crate::button_tiles::{TileType,Tile};
use tokio::sync::mpsc;
use std::collections::{HashMap,HashSet};

type Sender<T> = mpsc::UnboundedSender<T>;
type Receiver<T> = mpsc::UnboundedReceiver<T>;

pub enum NotificationAction{
    unhandled,
    toggle_expand(i64),
    remove_displayed(i64),
}
impl From<Action> for NotificationAction{
    fn from(item:Action) -> NotificationAction{ NotificationAction::unhandled }
}
#[derive(NativeClass)]
#[inherit(Control)]
pub struct TextPanel{
    tx:Sender<NotificationAction>,
    bg_rect:Ref<ColorRect>,
    main_rect:Ref<ColorRect>,
    label:Ref<Label>,
    hovering:bool,
}
impl InstancedDefault<Control,Sender<NotificationAction>> for TextPanel{
    fn make(args:&Sender<NotificationAction>) -> Self{
        TextPanel{
            tx:args.clone(),
            bg_rect:ColorRect::new().into_shared(),
            main_rect:ColorRect::new().into_shared(),
            label:Label::new().into_shared(),
            hovering:false,
        }
    }
}
impl Windowed<NotificationAction> for TextPanel{
    const BG_HIGHLIGHT_COLOR:Color = Color{r:255.0,g:255.0,b:255.0,a:1.0};
    const BG_COLOR:Color = Color{r:0.0,g:0.0,b:0.0,a:0.5};
    const MAIN_COLOR:Color = Color{r:0.0,g:0.0,b:0.0,a:0.6};
    const MARGIN_SIZE:f32 = 5.0;
    fn hovering(&self) -> bool {self.hovering}
    fn set_hovering(&mut self,value:bool){self.hovering = value}
    fn centering(&self) -> Centering {Centering::center}
    fn tx(&self) -> &Sender<NotificationAction> {&self.tx}
    fn bg_rect(&self) -> &Ref<ColorRect>{&self.bg_rect}
    fn main_rect(&self) -> &Ref<ColorRect>{&self.main_rect}
}
impl LabelButton<NotificationAction> for TextPanel{
    fn label(&self) -> &Ref<Label>{ &self.label }
}
#[methods]
impl TextPanel{
    #[method]
    fn _ready(&self,#[base] owner:TRef<Control>){
        <Self as LabelButton<NotificationAction>>::ready(self,owner);
        let _ = owner.connect("mouse_entered",owner,"hover",VariantArray::new_shared(),0);
        let _ = owner.connect("mouse_exited",owner,"unhover",VariantArray::new_shared(),0);
    }
    #[method]
    fn _process(&self,#[base] owner:TRef<Control>,delta:f64){
        <Self as LabelButton<NotificationAction>>::process(self,owner,delta);
    }
    #[method]
    fn hover(&mut self){
        <Self as Windowed<NotificationAction>>::hover(self);
    }
    #[method]
    fn unhover(&mut self){
        <Self as Windowed<NotificationAction>>::unhover(self);
    }
}
#[derive(Clone)]
pub struct DetailsConfig{
    tx:Option<Sender<NotificationAction>>,
    detail_messages:Vec<String>
}
impl Defaulted for DetailsConfig{
    fn default() -> Self{
        DetailsConfig{
            tx:None,
            detail_messages:Vec::new(),
        }
    }
}
impl DetailsConfig{
    fn make(tx:Sender<NotificationAction>,messages:&Vec<String>) -> Self{
        DetailsConfig{
            tx:Some(tx.clone()),
            detail_messages:messages.clone(),
        }
    }
}
#[derive(NativeClass)]
#[inherit(Control)]
pub struct Details{
    details:Vec<Instance<TextPanel>>,
}
impl InstancedDefault<Control,DetailsConfig> for Details{
    fn make(args:&DetailsConfig) -> Self{
        let details = args
            .detail_messages
            .clone()
            .into_iter()
            .map(|dtl| {
                let res = TextPanel::make_instance(&args.tx.clone().unwrap()).into_shared();
                let panel = unsafe{res.assume_safe()};
                let _ = panel.map(|obj,_| obj.set_text(dtl));
                res
            }).collect();
        Details{
            details:details,
        }
    }
}
#[methods]
impl Details{
    #[method]
    fn _ready(&self,#[base] owner:TRef<Control>){
        for dtl in &self.details{
            owner.add_child(dtl,true);
        }
    }
    #[method]
    fn _process(&self,#[base] owner:TRef<Control>,delta:f64){
        let owner_size = owner.size();
        let num_elements = std::cmp::max(self.details.len(),1) as f32;
        let element_size = Vector2::new(owner_size.x,owner_size.y/num_elements);
        let mut idx = 0.0;
        for detail in &self.details{
            let detail = unsafe{detail.assume_safe()};
            let _ = detail.map(|_,control| control.set_size(element_size,false));
            let _ = detail.map(|_,control| control.set_position(Vector2::new(0.0,element_size.y * idx),false));
            idx += 1.0;
        }
    }
}
pub struct NotificationConfig{
    id:Option<i64>,
    text:Option<String>,
    detail_messages:Vec<String>,
    tx:Option<Sender<NotificationAction>>
}
impl Defaulted for NotificationConfig{
    fn default() -> Self{
        NotificationConfig{
            id:None,
            text:None,
            detail_messages:Vec::new(),
            tx:None,
        }
    }
}
impl NotificationConfig{
    fn new(id:i64,text:String,detail_messages:&Vec<String>,tx:Option<Sender<NotificationAction>>) -> Self{
        NotificationConfig{
            id:Some(id),
            text:Some(text),
            detail_messages:detail_messages.clone(),
            tx:tx,
        }
    }
}

#[derive(NativeClass)]
#[inherit(Control)]
#[register_with(Self::register_window_signals)]
pub struct Notification{
    id:i64,
    detail_messages:Vec<String>,
    details:Instance<Details>,
    tx:Sender<NotificationAction>,
    bg_rect:Ref<ColorRect>,
    main_rect:Ref<ColorRect>,
    label:Ref<Label>,
    hovering:bool,
    font:Ref<DynamicFont>,
    timer:Ref<Timer>,
}
impl InstancedDefault<Control,NotificationConfig> for Notification{
    fn make(args:&NotificationConfig) -> Self{
        Notification{
            id:args.id.unwrap(),
            detail_messages:args.detail_messages.clone(),
            details:Details::make_instance( &DetailsConfig::make(args.tx.clone().unwrap(),&args.detail_messages)).into_shared(),
            tx:args.tx.clone().unwrap(),
            bg_rect:ColorRect::new().into_shared(),
            main_rect:ColorRect::new().into_shared(),
            label:Label::new().into_shared(),
            hovering:false,
            font: Self::base_font().unwrap(),
            timer:Timer::new().into_shared(),
        }
    }
}
impl Windowed<NotificationAction> for Notification{
    const BG_HIGHLIGHT_COLOR:Color = Color{r:255.0,g:25.0,b:0.0,a:1.0};
    const BG_COLOR:Color = Color{r:0.0,g:50.0,b:50.0,a:0.7};
    const MAIN_COLOR:Color = Color{r:0.0,g:0.0,b:0.0,a:1.0};
    const MARGIN_SIZE:f32 = 5.0;
    fn hovering(&self) -> bool {self.hovering}
    fn set_hovering(&mut self,value:bool){self.hovering = value}
    fn centering(&self) -> Centering {Centering::center}
    fn tx(&self) -> &Sender<NotificationAction> {&self.tx}
    fn bg_rect(&self) -> &Ref<ColorRect>{&self.bg_rect}
    fn main_rect(&self) -> &Ref<ColorRect>{&self.main_rect}
    fn from_command(&self,cmd:Action) -> NotificationAction{
        match cmd{
            Action::clicked => NotificationAction::toggle_expand(self.id),
            _ => NotificationAction::from(cmd),
        }
    }
}
impl LabelButton<NotificationAction> for Notification{
    fn base_font() -> Option<Ref<DynamicFont>>{
        let font_ref = DynamicFont::new().into_shared();
        let font = unsafe{font_ref.assume_safe()};
        let font_data = DynamicFontData::new().into_shared();
        let font_data = unsafe{font_data.assume_safe()};
        font_data.set_font_path("res://user_interface/client/overheads/Chicago.ttf");
        font.set_font_data(font_data);
        font.set_size(48);
        font.set_outline_color(Color{r:255.0,g:0.0,b:0.0,a:1.0});
        font.set_outline_size(Self::OUTLINE_SIZE);
        Some(font_ref)
    }
    fn font(&self) -> Option<Ref<DynamicFont>>{
        Some(self.font.clone())
    }
    fn label(&self) -> &Ref<Label>{&self.label}
}

#[methods]
impl Notification{
    const OUTLINE_SIZE:i64 = 1;
    #[method]
    fn _ready(&self,#[base] owner:TRef<Control>){
        <Self as LabelButton<NotificationAction>>::ready(self,owner);
        self.set_text(self.id.to_string());

        let details = unsafe{self.details.assume_safe()};
        let _ = details.map_mut(|_,control| control.set_visible(false));
        //let _ = details.map(|_,control| control.set_mouse_filter(Control::MOUSE_FILTER_PASS));

        owner.add_child(details,true);

        let timer = unsafe{self.timer.assume_safe()};
        let _ = timer.connect("timeout",owner,"display_timeout",VariantArray::new_shared(),0);
        owner.add_child(timer,true);
        timer.start(3.0);

        let _ = owner.connect("mouse_entered",owner,"hover",VariantArray::new_shared(),0);
        let _ = owner.connect("mouse_exited",owner,"unhover",VariantArray::new_shared(),0);
        let _ = owner.connect("clicked",owner,"clicked",VariantArray::new_shared(),0);
    }
    #[method]
    fn _process(&self,#[base] owner:TRef<Control>,delta:f64){
        <Self as LabelButton<NotificationAction>>::process(self,owner,delta);
        let main_rect = unsafe{self.main_rect.assume_safe()};
        
        let main_size = main_rect.size();
        let owner_size = owner.size();
        let details = unsafe{self.details.assume_safe()};
        let _ = details.map(|_,control|control.set_size(main_size,false));
        let _ = details.map(|_,control|control.set_position(main_rect.position(),false));
        self.set_font_size((main_size.y/2.0).round() as i64);
    }
    #[method]
    fn _input(&self,#[base] owner:TRef<Control>,event:Ref<InputEvent>){
        <Self as Windowed<NotificationAction>>::input(self,owner,event.clone());
    }
    #[method]
    fn hover(&mut self){
        <Self as Windowed<NotificationAction>>::hover(self);
    }
    #[method]
    fn unhover(&mut self){
        <Self as Windowed<NotificationAction>>::unhover(self);
    }
    #[method]
    fn clicked(&self,#[base] owner:TRef<Control>){
        let details = unsafe{self.details.assume_safe()};
        let _ = details.map_mut(|_,control| control.set_visible(!control.is_visible()));
        let timer = unsafe{self.timer.assume_safe()};
        self.toggle_timer();
    }
    #[method]
    fn toggle_timer(&self){
        let timer = unsafe{self.timer.assume_safe()};
        if timer.is_stopped(){
            timer.start(3.0);
        }else{
            timer.stop();
        }
    }
    #[method]
    fn display_timeout(&self){
        let _ = self.tx.send(NotificationAction::remove_displayed(self.id));
    }
}
#[derive(NativeClass)]
#[inherit(Control)]
#[register_with(Self::register_window_signals)]
pub struct Notifications{
    tx:Sender<NotificationAction>,
    rx:Receiver<NotificationAction>,
    bg_rect:Ref<ColorRect>,
    main_rect:Ref<ColorRect>,
    label:Ref<Label>,
    hovering:bool,
    notifications:Vec<Instance<Notification>>,
    expanded:Option<i64>,
    displayed:HashSet<i64>,
}
impl Instanced<Control> for Notifications{
    fn make() -> Self{
        let (tx,rx) = mpsc::unbounded_channel::<NotificationAction>();
        Notifications{
            tx:tx,
            rx:rx,
            bg_rect:ColorRect::new().into_shared(),
            main_rect:ColorRect::new().into_shared(),
            label:Label::new().into_shared(),
            hovering:false,
            notifications:Vec::new(),
            expanded:None,
            displayed:HashSet::new(),
        }
    }
}
impl Windowed<NotificationAction> for Notifications{
    const BG_HIGHLIGHT_COLOR:Color = Color{r:255.0,g:255.0,b:255.0,a:0.0};
    const BG_COLOR:Color = Color{r:0.0,g:0.0,b:0.0,a:0.0};
    const MAIN_COLOR:Color = Color{r:255.0,g:0.0,b:0.0,a:0.0};
    const MARGIN_SIZE:f32 = 5.0;
    fn hovering(&self) -> bool {self.hovering}
    fn set_hovering(&mut self,value:bool){self.hovering = value}
    fn centering(&self) -> Centering {Centering::center}
    fn tx(&self) -> &Sender<NotificationAction> {&self.tx}
    fn bg_rect(&self) -> &Ref<ColorRect>{&self.bg_rect}
    fn main_rect(&self) -> &Ref<ColorRect>{&self.main_rect}
}
impl LabelButton<NotificationAction> for Notifications{
    fn label(&self) -> &Ref<Label>{&self.label}
}
#[methods]
impl Notifications{
    #[method]
    fn _ready(&self,#[base] owner:TRef<Control>){
        <Self as Windowed<NotificationAction>>::ready(self,owner);
        let _ = owner.connect("mouse_entered",owner,"hover",VariantArray::new_shared(),0);
        let _ = owner.connect("mouse_exited",owner,"unhover",VariantArray::new_shared(),0);
        owner.set_visible(true);
    }
    #[method]
    fn _process(&mut self,#[base] owner:TRef<Control>,delta:f64){
        <Self as Windowed<NotificationAction>>::process(self,owner,delta);
        let screen_size = OS::godot_singleton().window_size();
        owner.set_size(screen_size/2.0,false);
        self.handle_panel_rx(owner,delta);
        match self.expanded{
            Some(id) => self.handle_sizing_if_expanded(owner,id),
            None => self.handle_sizing(owner),
        }
    }
    fn handle_sizing(&self,owner:TRef<Control>){
        let num_notifications = self.displayed.len();
        let window_size = OS::godot_singleton().window_size();

        let notification_size = Vector2::new(
            window_size.x/3.0,
            window_size.y/(std::cmp::max(10,num_notifications) as f32),
        );

        let owner_size = Vector2::new(
            notification_size.x,
            notification_size.y * (num_notifications as f32)
        );

        owner.set_size(owner_size,false);

        let mut idx = 0.0;
        for id in &self.displayed{
            let notification = &self.notifications[*id as usize];
            let notification = unsafe{notification.assume_safe()};
            let _ = notification.map(|_,control| control.set_size(notification_size,false));
            let _ = notification.map(|_,control| control.set_position(Vector2::new(0.0,notification_size.y * idx),false));
            idx += 1.0;
        }

    }
    fn handle_sizing_if_expanded(&self,owner:TRef<Control>,expanded:i64){
        let notification = &self.notifications[expanded as usize];
        let notification = unsafe{notification.assume_safe()};
        let _ = notification.map(|_,control| control.set_size(owner.size(),false));
        let _ = notification.map(|_,control| control.set_position(Vector2::new(0.0,0.0),false));
    }
    fn handle_panel_rx(&mut self,owner:TRef<Control>,delta:f64){
        match self.rx.try_recv(){
            Ok(NotificationAction::toggle_expand(id)) => {
                match self.expanded{
                    Some(curr_id) if id == curr_id => {
                        for notification in &self.notifications{
                            let notification = unsafe{notification.assume_safe()};
                            let _ = notification.map(|obj,control|{control.set_visible(true)});
                        }
                        self.expanded = None;
                    }
                    Some(_) => assert!(false),
                    None => {
                        for notification in &self.notifications{
                            let notification = unsafe{notification.assume_safe()};
                            let _ = notification.map(|obj,control|if obj.id != id{control.set_visible(false) });
                        }
                        self.expanded = Some(id);
                    }
                }
            }
            Ok(NotificationAction::remove_displayed(id)) => {
                let notification = &self.notifications[id as usize];
                let notification = unsafe{notification.assume_safe()};
                let _ = notification.map(|_,control| control.set_visible(false));
                self.displayed.remove(&id);
            }
            Ok(NotificationAction::unhandled) => {}
            Err(_) => {}
        }
    }
    #[method]
    fn _input(&self,#[base] owner:TRef<Control>,event:Ref<InputEvent>){
        <Self as Windowed<NotificationAction>>::input(self,owner,event.clone());
        if let Ok(event) = event.try_cast::<InputEventKey>(){
            let event = unsafe{event.assume_safe()};
            if event.is_action_released("server_stats_toggle",true){
                owner.set_visible(!owner.is_visible());
            }
        }
    }
    #[method]
    fn hover(&mut self){
        <Self as Windowed<NotificationAction>>::hover(self);
    }
    #[method]
    fn unhover(&mut self){
        <Self as Windowed<NotificationAction>>::unhover(self);
    }
    #[method]
    fn add_notification(&mut self,#[base] owner:TRef<Control>,text:String,details:Vec<String>) -> Result<(),Error>{
        let id = self.notifications.len() as i64;
        let config = NotificationConfig::new(id,text.clone(),&details,Some(self.tx.clone()));
        let notification = Notification::make_instance(&config).into_shared(); 
        self.notifications.push(notification.clone());
        self.displayed.insert(id);
        owner.add_child(notification.clone(),true);
        let notification = unsafe{notification.assume_safe()};
        let _ = notification.map(|obj,_| obj.set_text(text));
        Ok(())
    }

}
#[derive(Clone)]
pub enum Error{
    AddNotificationError,
}
impl Into<u8> for Error{
    fn into(self) -> u8{
        match self{
            Error::AddNotificationError => 1 
        }
    }
}
impl ToVariant for Error{
    fn to_variant(&self) -> Variant{
        match self{
            Error::AddNotificationError => Variant::new(Into::<u8>::into(self.clone()))
        }
    }
}
