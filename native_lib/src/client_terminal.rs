use gdnative::prelude::*;
use gdnative::api::*;

#[derive(NativeClass)]
#[inherit(CanvasLayer)]
pub struct ClientTerminal{
    #[property]
    bg_rect: Ref<ColorRect>,
    label:Ref<TextEdit>,
}

#[methods]
impl ClientTerminal{
    fn new(_base:&CanvasLayer) -> Self{
        ClientTerminal{
            bg_rect: ColorRect::new().into_shared(),
            label : TextEdit::new().into_shared(),
        }
    }

    #[method]
    fn _ready(&self,#[base] owner:&CanvasLayer){
        let rect = unsafe{ self.bg_rect.assume_safe()};
        let label = unsafe{ self.label.assume_safe()};
        rect.set_anchors_preset(Control::PRESET_WIDE,true);
        rect.set_frame_color(Color{r:0.0,g:0.0,b:0.0,a:0.5});
        label.set_anchors_preset(Control::PRESET_LEFT_WIDE,true);
        owner.add_child(rect,true);
        owner.add_child(label,true);

    }

    #[method]
    fn _input(&self,#[base] owner: &CanvasLayer,event: Ref<InputEvent>){
        if let Ok(event) = event.try_cast::<InputEventKey>(){
            let event = unsafe{ event.assume_safe()};
            if event.is_action_released("terminal_toggle",false){
                owner.set_visible(!owner.is_visible());
            }
        }
    }

    #[method]
    fn _process(&self,#[base] owner:&CanvasLayer,delta:f64){

    }
}
