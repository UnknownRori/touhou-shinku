use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Resource)]
pub struct BulletResource {
    #[export]
    pub id: GString,
    #[export]
    pub texture: Rect2,
    #[export]
    pub collision_circle: f64,

    base: Base<Resource>,
}
