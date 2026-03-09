use godot::prelude::*;

#[derive(Default)]
pub struct Bullet {
    pub position: Vector2,
    pub velocity: Vector2,
    pub texture: Rect2,
    pub active: bool,
}

pub struct BulletPool {
    pub items: Vec<Bullet>,
    next_id: u32,
}

impl BulletPool {
    pub fn new(pool_size: usize) -> Self {
        let mut items = Vec::with_capacity(pool_size);
        for _ in 0..pool_size {
            items.push(Bullet::default());
        }

        Self { items, next_id: 0 }
    }

    pub fn spawn(&mut self, position: Vector2, velocity: Vector2, texture: Rect2) {
        let bullet = &mut self.items[self.next_id as usize];

        bullet.position = position;
        bullet.velocity = velocity;
        bullet.texture = texture;
        bullet.active = true;

        self.next_id = (self.next_id + 1) % self.items.len() as u32;
    }

    pub fn update(&mut self, dt: f64) {
        for bullet in &mut self.items {
            if !bullet.active {
                continue;
            }
            bullet.position += bullet.velocity * dt as f32;
        }
    }
}

impl Default for BulletPool {
    fn default() -> Self {
        BulletPool::new(1024)
    }
}
