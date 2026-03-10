use godot::classes::{CharacterBody2D, ICharacterBody2D};
use godot::prelude::*;

use crate::autoload::GameState;
use crate::pools::EntityCollision;
use crate::{FlanExtension, components::*};

#[derive(GodotClass)]
#[class(init, base=CharacterBody2D)]
pub struct Enemy {
    #[export]
    hitbox: Option<Gd<HitboxComponent>>,
    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for Enemy {
    fn ready(&mut self) {
        //
    }

    fn physics_process(&mut self, _dt: f64) {
        let mut gm = FlanExtension::get_singleton::<GameState>().unwrap();
        let position = self.base().get_global_position();
        let radius = self.hitbox.clone().unwrap().bind().radius as f32;
        let collision = EntityCollision::Enemy;
        gm.bind_mut().register_entity(position, radius, collision);
    }
}
