use godot::{classes::Texture, prelude::*};

#[derive(GodotClass)]
#[class(init, base=Resource)]
pub struct UpgradeResource {
    #[export]
    pub id: GString,
    #[export]
    pub name: GString,
    #[export]
    pub texture: Option<Gd<Texture>>,

    // TODO : Start thinking how to append the upgrade effect
    base: Base<Resource>,
}
