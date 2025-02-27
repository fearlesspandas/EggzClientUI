
use gdnative::prelude::*;
use gdnative::api::*;
use crate::traits::{CreateSignal,Instanced,InstancedDefault,Defaulted};
use crate::field_ability_mesh::{FieldAbilityMesh};
use crate::field_ability_actions::ToAction;
use crate::field_abilities::{AbilityType,SubAbilityType};
use tokio::sync::mpsc;
use rand::Rng;

const radius:f32 = 10.0;

#[derive(NativeClass)]
#[inherit(Spatial)]
pub struct DamageIndicator{
    mesh_instance:Ref<MeshInstance>,
    text_mesh:Ref<TextMesh>,
    visibility_timer:Ref<Timer>,
}
impl Instanced<Spatial> for DamageIndicator{
    fn make() -> Self{
        DamageIndicator{
            mesh_instance:MeshInstance::new().into_shared(),
            text_mesh:TextMesh::new().into_shared(),
            visibility_timer:Timer::new().into_shared(),
        }
    }
}
#[methods]
impl DamageIndicator{
    #[method]
    fn _ready(&self,#[base] owner:TRef<Spatial>){
        let timer = unsafe{self.visibility_timer.assume_safe()};
        timer.set_wait_time(3.0);
        let _ = timer.connect("timeout",owner,"hide",VariantArray::new_shared(),0);
        owner.add_child(timer,true);

        let text_mesh = unsafe{self.text_mesh.assume_safe()};
        let font = DynamicFont::new().into_shared();
        let font = unsafe{font.assume_safe()};
        let font_data = DynamicFontData::new().into_shared();
        let font_data = unsafe{font_data.assume_safe()};
        font_data.set_font_path("res://user_interface/client/overheads/Chicago.ttf");
        font.set_font_data(font_data);
        font.set_size(200);
        font.set_outline_color(Color{r:255.0,g:0.0,b:0.0,a:1.0});
        font.set_outline_size(40);
        text_mesh.set_font(font);
        let material = SpatialMaterial::new().into_shared();
        let material = unsafe{material.assume_safe()};
        material.set_albedo(Color{r:255.0,g:0.0,b:0.0,a:1.0});
        text_mesh.set_material(material);

        let mesh_instance = unsafe{self.mesh_instance.assume_safe()};
        
        mesh_instance.set_mesh(text_mesh);
        mesh_instance.translate(Vector3{x:0.0, y:5.0,z:0.0});
        owner.add_child(mesh_instance,true);
    }

    #[method]
    fn set_text(&self,text:String){
        let text_mesh = unsafe{self.text_mesh.assume_safe()};
        text_mesh.set_text(text);
    }

    #[method]
    fn hide(&self,#[base] owner:TRef<Spatial>){
        let mesh_instance = unsafe{self.mesh_instance.assume_safe()};
        let timer = unsafe{self.visibility_timer.assume_safe()};

        mesh_instance.set_translation(Vector3{x:0.0,y:0.0,z:0.0});

        owner.set_visible(false);
        timer.stop();
    }
    #[method]
    fn show_for_duration(&self, #[base] owner:TRef<Spatial>,duration:f64){
        let mesh_instance = unsafe{self.mesh_instance.assume_safe()};
        let timer = unsafe{self.visibility_timer.assume_safe()};

        let x = rand::thread_rng().gen_range(-radius..radius);
        let y = rand::thread_rng().gen_range(radius/2.0..10.0);
        let z = rand::thread_rng().gen_range(-radius..radius);

        mesh_instance.translate(Vector3{x:x,y:y,z:z});
        owner.set_visible(true);
        timer.start(duration);
    }
    #[method]
    fn show(&self, #[base] owner:TRef<Spatial>){
        owner.set_visible(true);
    }
}
#[derive(Clone)]
pub enum DamageError{
    ShowForDurationError(String),
}
impl Into<u8> for DamageError{
    fn into(self) -> u8{
        match self{
            DamageError::ShowForDurationError(_) => 0, 
        }
    }
}
impl ToVariant for DamageError{
    fn to_variant(&self) -> Variant{
        Variant::new(Into::<u8>::into(self.clone()))
    }

}

type DAMAGE_DISPLAY = Spatial;
const NUM_INDICATORS:i64 = 10;
#[derive(NativeClass)]
#[inherit(Spatial)]
pub struct DamageDisplay{

    damage_indicators:Vec<Instance<DamageIndicator>>,
}

impl Instanced<DAMAGE_DISPLAY> for DamageDisplay{
    fn make() -> Self{

        let mut indicators = Vec::new();
        for i in 0..NUM_INDICATORS{
            indicators.push(DamageIndicator::make_instance().into_shared());
        }
        DamageDisplay{
            damage_indicators:indicators,
        }
    }
}

#[methods]
impl DamageDisplay{
    #[method]
    fn _ready(&self,#[base] owner:TRef<DAMAGE_DISPLAY>){
        for indicator in &self.damage_indicators{
            let indicator = unsafe{indicator.assume_safe()};

            let _ = indicator.map(|obj,body| obj.hide(body));
            owner.add_child(indicator,true);
        }
    }

    #[method]
    fn find_empty_slot(&self) -> Option<usize>{
        let mut idx = 0;
        for i in 0..self.damage_indicators.len(){
            let indicator =  &self.damage_indicators[i];
            let indicator = unsafe{indicator.assume_safe()};
            let res = indicator.map(|_,body|body.is_visible());
            match res{
                Ok(visible) if !visible => return Some(idx), 
                Err(e) => {
                    assert!(false,"Could not find visibility of DamageIndicator");
                    idx += 1;
                }
                _ => idx += 1
            }
            
        }
        None
    }

    #[method]
    fn show_for_duration(&self,text:String,duration:f64) -> Result<(),DamageError>{
        let idx_op = self.find_empty_slot();
        match idx_op{
            None => Err(DamageError::ShowForDurationError("Could not find empty slot".to_string())),
            Some(idx) => {
                let indicator:&Instance<DamageIndicator> = &self.damage_indicators[idx];
                let indicator = unsafe{indicator.assume_safe()};
                
                indicator.map(|obj,body| {
                    obj.set_text(text);
                    obj.show_for_duration(body,duration);
                }).map_err(|_| DamageError::ShowForDurationError("Error attempting to display damage".to_string()))
            }
        }
    }
}

