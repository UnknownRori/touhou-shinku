use godot::classes::{CharacterBody2D, ICharacterBody2D};
use godot::prelude::*;

use crate::FlanExtension;
use crate::autoload::GameState;
use crate::components::*;
use crate::pools::EntityCollision;

#[derive(GodotClass)]
#[class(init, base=CharacterBody2D)]
pub struct Player {
    #[export]
    hp: Option<Gd<HealthComponent>>,
    #[export]
    hitbox: Option<Gd<HitboxComponent>>,
    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn ready(&mut self) {
        let mut gm = FlanExtension::get_singleton::<GameState>().unwrap();
        if let Some(hp) = &self.hp {
            let hp = hp.clone();
            gm.bind_mut().player_hp = Some(hp);
            gm.bind_mut().player = Some(self.to_gd().clone());
        }
    }

    fn physics_process(&mut self, _dt: f64) {
        let mut gm = FlanExtension::get_singleton::<GameState>().unwrap();
        let position = self.base().get_global_position();
        let radius = self.hitbox.clone().unwrap().bind().radius as f32;
        let collision = EntityCollision::Player;
        gm.bind_mut().register_entity(position, radius, collision);
    }
}
