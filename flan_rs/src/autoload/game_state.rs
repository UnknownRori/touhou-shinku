use godot::{classes::Timer, prelude::*};

use crate::{bullet_manager::BulletManager, components::*, entities::*, pools::*};

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct GameState {
    #[var]
    pub player: Option<Gd<Player>>,
    #[var]
    pub player_hp: Option<Gd<HealthComponent>>,
    #[var]
    pub boss_hp: Option<Gd<HealthComponent>>,
    #[var]
    pub boss_timeout_timer: Option<Gd<Timer>>,
    #[var]
    pub bullet_manager: Option<Gd<BulletManager>>,
    base: Base<Node>,
}

#[godot_api]
impl GameState {
    #[func]
    pub fn spawn_bullet(
        &mut self,
        position: Vector2,
        velocity: Vector2,
        rotation: f32,
        radius: f32,
        texture: Rect2,
        collision: EntityCollision,
        bullet_type: BulletType,
        damage: i64,
    ) {
        let mut bm = self.bullet_manager.clone().unwrap();
        bm.run_deferred(move |bm| {
            bm.spawn(
                position,
                velocity,
                rotation,
                radius,
                texture,
                collision,
                bullet_type,
                damage,
            )
        });
    }

    #[func]
    pub fn register_entity(&mut self, entity: Gd<Node2D>) {
        let mut bm = self.bullet_manager.clone().unwrap();
        bm.run_deferred(move |bm| bm.register(entity));
    }

    #[func]
    pub fn get_player_position(&self) -> Vector2 {
        self.player.clone().unwrap().get_global_position()
    }

    // Alert UI stuff
    // Maybe there is much better way
    #[signal]
    pub fn show_alert(text: GString, duration: f64);

    #[signal]
    pub fn done_alert();
}
