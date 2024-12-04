use agb::{
    self, display::{
        object::{OamManaged, Object, Tag},
        Priority
    }, fixnum::Vector2D,
};

#[allow(dead_code)]
pub struct Entity<'a> {
    pub sprite: Object<'a>,
    pub position: Vector2D<i32>,
    pub velocity: Vector2D<i32>,
    pub collision_mask: Vector2D<u16>,
}

/// impl of entity to allow for methods to interact with the sprite and setup
impl<'a> Entity<'a> {
    pub fn new(object: &'a OamManaged, target: &Tag, collision_mask: Vector2D<u16>) -> Self {
        let mut dummy_object = object.object_sprite(target.sprite(0));

        dummy_object.set_priority(Priority::P1);
        Entity {
            sprite: dummy_object,
            collision_mask,
            position: (0, 0).into(),
            velocity: (12_u16, 48_u16).into(),
        }
    }

    /// Updates the position of the sprite based on what has been set in the position variable
    pub fn update_sprite_position(&mut self) {
        self.sprite
            .set_x(self.position.x as u16)
            .set_y(self.position.y as u16);
    }

    /// Set where the entity should spawn the sprite
    pub fn set_spawn(&mut self, spawn: Vector2D<i32>) {
        self.position = spawn;
        self.sprite
            .set_x(self.position.x as u16)
            .set_y(self.position.y as u16);
    }
}