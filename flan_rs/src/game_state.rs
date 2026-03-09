use godot::prelude::*;

use crate::{bullet_manager::BulletManager, components::health_component::HealthComponent};

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct GameState {
    #[var]
    pub player_hp: Option<Gd<HealthComponent>>,
    #[var]
    pub bullet_manager: Option<Gd<BulletManager>>,
    base: Base<Node>,
}
