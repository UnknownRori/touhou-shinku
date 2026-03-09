use godot::prelude::*;

#[derive(GodotConvert, Var, Export, Default, Clone, Debug)]
#[godot(via = GString)]
pub enum BulletCollision {
    Player,
    #[default]
    Enemy,
}

#[derive(GodotConvert, Var, Export, Default, Clone, Debug)]
#[godot(via = GString)]
pub enum BulletType {
    #[default]
    Default,
}

#[derive(Default)]
pub struct Bullet {
    pub position: Vector2,
    pub velocity: Vector2,
    pub rotation: f32,
    pub texture: Rect2,
    pub collision: BulletCollision,
    pub bullet_type: BulletType,
    pub active: bool,
    pub padding: u128,
}

impl Bullet {
    pub fn new(
        position: Vector2,
        velocity: Vector2,
        rotation: f32,
        texture: Rect2,
        collision: BulletCollision,
        bullet_type: BulletType,
    ) -> Self {
        Self {
            position,
            velocity,
            rotation,
            texture,
            collision,
            bullet_type,
            active: true,
            padding: 0,
        }
    }
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

    pub fn spawn(&mut self, out: Bullet) {
        let bullet = &mut self.items[self.next_id as usize];
        *bullet = out;

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
