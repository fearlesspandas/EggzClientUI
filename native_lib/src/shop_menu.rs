use gdnative::prelude::*;
use gdnative::api::*;
use crate::traits::{Instanced,InstancedDefault,Defaulted};
use tokio::sync::mpsc;

type Sender<T> = mpsc::UnboundedSender<T>;
type Receiver<T> = mpsc::UnboundedReceiver<T>;

impl <T> Defaulted for Sender<T>{
    fn default() -> Self{
        let (tx,rx) = mpsc::unbounded_channel::<T>();
        tx
    }
}
enum ItemType{
    Empty,
    
}
enum Command{
    AddItem(ItemType),
}
#[derive(NativeClass)]
#[inherit(Control)]
pub struct ShopItem{
    item_type:ItemType,
    bg_rect:Ref<ColorRect>,
    menu_tx:Sender<Command>,
    color:Color,
    hovering:bool,
}
impl InstancedDefault<Control,Sender<Command>> for ShopItem{
    fn make(args:&Sender<Command>) -> Self{
        ShopItem{
            item_type:ItemType::Empty,
            bg_rect: ColorRect::new().into_shared(),
            menu_tx:args.clone(),
            color:Color{r:0.0,g:255.0,b:255.0,a:1.0},
            hovering:false,
        }
    }
}
#[methods]
impl ShopItem{
    #[method]
    fn _ready(&self,#[base] owner:TRef<Control>){
        let bg_rect = unsafe{self.bg_rect.assume_safe()};
        bg_rect.set_frame_color(Color{r:0.0,g:255.0,b:255.0,a:1.0});
        owner.add_child(bg_rect,true);
        bg_rect.connect("mouse_entered",owner,"hover",VariantArray::new_shared(),0);
        bg_rect.connect("mouse_exited",owner,"unhover",VariantArray::new_shared(),0);
    }
    #[method]
    fn _process(&self,#[base] owner:TRef<Control>,delta:f64){
        let bg_rect = unsafe{self.bg_rect.assume_safe()};
        bg_rect.set_size(owner.size(),false);
        //if self.hovering{
        //    bg_rect.set_frame_color(Color{r:255.0,g:255.0,b:255.0,a:1.0});
        //}else{
        //    bg_rect.set_frame_color(Color{r:0.0,g:255.0,b:255.0,a:1.0});
        //}
    }
    #[method]
    fn hover(&mut self,#[base] owner:TRef<Control>){
        let bg_rect = unsafe{self.bg_rect.assume_safe()};
        bg_rect.set_frame_color(Color{r:255.0,g:255.0,b:255.0,a:1.0});
        self.hovering = true;
        
    }
    #[method]
    fn unhover(&mut self,#[base] owner:TRef<Control>){
        let bg_rect = unsafe{self.bg_rect.assume_safe()};
        bg_rect.set_frame_color(Color{r:0.0,g:255.0,b:255.0,a:1.0});
        self.hovering = false;
    }
}
#[derive(NativeClass)]
#[inherit(Control)]
pub struct ShopMenu{
    bg_rect:Ref<ColorRect>,
    items:Vec<Instance<ShopItem>>,
    tx:Sender<Command>,
    rx:Receiver<Command>,
}
impl Instanced<Control> for ShopMenu{
    fn make() -> Self{
        let (tx,rx) = mpsc::unbounded_channel::<Command>();
        ShopMenu{
            bg_rect: ColorRect::new().into_shared(),
            items: Vec::new(),
            tx:tx,
            rx:rx,
        }
    }
}
#[methods]
impl ShopMenu{
    #[method]
    fn _ready(&self,#[base] owner:TRef<Control>){
        let bg_rect = unsafe{self.bg_rect.assume_safe()};
        bg_rect.set_frame_color(Color{r:255.0,g:255.0,b:255.0,a:1.0});
        owner.add_child(bg_rect,true);
        owner.set_visible(false);
        self.tx.send(Command::AddItem(ItemType::Empty));
        self.tx.send(Command::AddItem(ItemType::Empty));
    }
    fn add_item(&mut self,item_type:ItemType) {
        self.tx.send(Command::AddItem(item_type));
    }
    #[method]
    fn _process(&mut self,#[base] owner:TRef<Control>,delta:f64){
        match self.rx.try_recv(){
            Ok(Command::AddItem(typ)) => {
                let item = ShopItem::make_instance(&self.tx).into_shared();
                let item_obj = unsafe{item.assume_safe()};
                item_obj.map_mut(|obj,_|obj.item_type = typ);
                owner.add_child(item.clone(),true);
                self.items.push(item);
            }
            Err(_) => {}
        } 
        //sizing and positioning
        let vp = owner.get_viewport().unwrap();
        let vp  = unsafe{vp.assume_safe()};

        let size = vp.get_visible_rect().size;

        owner.set_size(size/2.0,false);
        //background
        let bg_rect = unsafe{self.bg_rect.assume_safe()};
        bg_rect.set_size(owner.size(),false);
        //items
        let mut idx = 0.0;
        let owner_size = owner.size();
        let num_items = self.items.len() as f32;
        let item_size = Vector2{x:owner_size.x,y:owner_size.y/num_items};
        for item in &self.items{
            let item = unsafe{item.assume_safe()};
            item.map(|obj,control| {
                control.set_size(item_size,false);
                let position = Vector2{x:0.0,y:item_size.y * idx};
                control.set_position(position,false);
                idx += 1.0;
            });
        }
    }

    #[method]
    fn _input(&self,#[base] owner:TRef<Control>, event:Ref<InputEvent>){
        if let Ok(event) = event.try_cast::<InputEventKey>(){
            let event = unsafe{event.assume_safe()};
            if event.is_action_released("shop_menu_toggle",false){
                owner.set_visible(!owner.is_visible());
            }
        }
    }
}
