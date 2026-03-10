use godot::classes::{CharacterBody2D, ICharacterBody2D};
use godot::prelude::*;

use crate::FlanExtension;
use crate::autoload::GameState;
use crate::components::*;

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
}
