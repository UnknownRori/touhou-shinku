use godot::{classes::Texture, prelude::*};

#[derive(GodotClass)]
#[class(init, base=Resource)]
pub struct LevelResource {
    #[export]
    pub id: GString,
    #[export]
    pub name: GString,
    #[export]
    pub scene: Option<Gd<PackedScene>>,
    #[export]
    pub icon: Option<Gd<Texture>>,

    base: Base<Resource>,
}
