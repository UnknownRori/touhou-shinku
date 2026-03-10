use godot::classes::{CharacterBody2D, ICharacterBody2D};
use godot::prelude::*;

use crate::components::*;

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
        // TODO : Register collision on bullet manger
    }
}
