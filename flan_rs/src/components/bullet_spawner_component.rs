use godot::prelude::*;

use crate::{FlanExtension, autoload::GameState, resources::BulletResource};

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct BulletSpawnerComponent {
    #[export]
    entity: Option<Gd<Node2D>>,

    #[export]
    bullet: Option<Gd<BulletResource>>,
    base: Base<Node>,
}

#[godot_api]
impl BulletSpawnerComponent {
    #[func]
    fn spawn(&mut self, velocity: Vector2) {
        let mut gm = FlanExtension::get_singleton::<GameState>().unwrap();
        let texture = self.bullet.clone().unwrap().bind().texture;
        let position = self.entity.clone().unwrap().get_global_position();
        gm.bind_mut().spawn_bullet(position, velocity, texture);
    }
}
