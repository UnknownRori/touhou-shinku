use godot::prelude::*;

#[derive(GodotConvert, Var, Export, Default, Clone, Debug, PartialEq)]
#[godot(via = GString)]
pub enum EntityCollision {
    Player,
    #[default]
    Enemy,
}

#[derive(Default)]
#[repr(align(16))]
pub struct Entity {
    pub position: Vector2,
    pub radius: f32,
    pub collision: EntityCollision,
}

impl Entity {
    pub fn new(position: Vector2, radius: f32, collision: EntityCollision) -> Self {
        Self {
            position,
            radius,
            collision,
        }
    }
}

// TODO : Need to convert into Spatial Hash for efficient collision resolver
pub struct EntityPool {
    pub items: Vec<Entity>,
}

impl EntityPool {
    pub fn new(pool_size: usize) -> Self {
        let items = Vec::with_capacity(pool_size);
        Self { items }
    }

    pub fn insert(&mut self, entity: Entity) {
        self.items.push(entity);
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }
}
