use godot::prelude::*;

use crate::{FlanExtension, autoload::GameState, pools::*, resources::BulletResource};

#[derive(GodotClass, Debug)]
#[class(init, base=Object)]
pub struct GBullet {
    #[var]
    pub position: Vector2,
    #[var]
    pub velocity: Vector2,
    #[var]
    pub rotation: f32,
    #[var]
    pub texture: Rect2,
    #[var]
    pub collision: BulletCollision,
    #[var]
    pub bullet_type: BulletType,

    base: Base<Object>,
}

#[godot_api]
impl GBullet {
    #[func]
    pub fn create(position: Vector2, velocity: Vector2, rotation: f32) -> Gd<GBullet> {
        Gd::from_init_fn(|base| GBullet {
            position,
            velocity,
            rotation,
            texture: Default::default(),
            collision: Default::default(),
            bullet_type: Default::default(),
            base,
        })
    }
}

// ------------------

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct BulletSpawnerComponent {
    #[export]
    collision: BulletCollision,
    #[export]
    bullet_type: BulletType,
    #[export]
    bullet: Option<Gd<BulletResource>>,

    base: Base<Node>,
}

#[godot_api]
impl BulletSpawnerComponent {
    #[func]
    fn spawn(&mut self, mut bullet: Gd<GBullet>) {
        let mut gm = FlanExtension::get_singleton::<GameState>().unwrap();
        let texture = self.bullet.clone().unwrap().bind().texture;
        bullet.bind_mut().texture = texture;
        bullet.bind_mut().collision = self.collision.clone();
        bullet.bind_mut().bullet_type = self.bullet_type.clone();
        gm.bind_mut().spawn_bullet(bullet);
    }
}
