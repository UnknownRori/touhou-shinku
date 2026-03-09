use godot::prelude::*;

use crate::{bullet_manager::BulletManager, components::*, entities::*};

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct GameState {
    #[var]
    pub player: Option<Gd<Player>>,
    #[var]
    pub player_hp: Option<Gd<HealthComponent>>,
    #[var]
    pub bullet_manager: Option<Gd<BulletManager>>,
    base: Base<Node>,
}

#[godot_api]
impl GameState {
    #[func]
    pub fn spawn_bullet(&mut self, position: Vector2, velocity: Vector2, texture: Rect2) {
        let mut bm = self.bullet_manager.clone().unwrap();
        bm.run_deferred(move |bm| bm.spawn(position, velocity, texture));
    }

    #[func]
    pub fn get_player_position(&self) -> Vector2 {
        self.player.clone().unwrap().get_global_position()
    }
}
