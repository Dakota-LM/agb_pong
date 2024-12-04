use crate::GRAPHICS;
use crate::entity::Entity;
use crate::paddle::Side;
use crate::common::random_number_between;
use agb::{
    self, display::object::{OamManaged, Tag},
};

// MAKE REFERENCE OBJECTS TO BALL SPRITE
pub static BALL: &Tag = GRAPHICS.tags().get("Ball");

/// Ball struct that holds the sprite of the ball
pub struct Ball<'a> {
    pub entity: Entity<'a>,
}

/// Impl of ball to allow for methods to interact with the sprite
impl<'a> Ball<'a> {
    pub fn new(object: &'a OamManaged, target: &Tag, y_position: i32) -> Self {
        let mut ball: Entity = Entity::new(object, target, (16_u16, 16_u16).into());
        ball.sprite.set_sprite(object.sprite(target.sprite(0)));
        ball.velocity.x = 2;
        ball.velocity.y = 2;
        ball.set_spawn((50, y_position).into());
        ball.sprite.show();
        Self { entity: ball }
    }

    /// Keeps the ball within the bounds of the screen not allowing it to move pass the limit
    pub fn checks_and_keeps_in_bounds(&mut self) {
        self.entity.position.x = (self.entity.position.x + self.entity.velocity.x)
            .clamp(-16, agb::display::WIDTH);    // previously .clamp(0, agb::display::WIDTH - 16)
        self.entity.position.y = (self.entity.position.y + self.entity.velocity.y)
            .clamp(0, agb::display::HEIGHT - 16);
    }

    /// Bounces the ball if it hits the edge of the screen
    pub fn bounce_if_hits_screen_bounds(&mut self) {
        if self.entity.position.y == 0 || self.entity.position.y == agb::display::HEIGHT - 16 {
            self.entity.velocity.y = -self.entity.velocity.y;
        }
    }
    
    pub fn spawn_to_centre(&mut self, y_position: i32, last_score: Side) {
        self.entity.sprite.hide();
        match last_score {
            Side::Right => { self.entity.set_spawn((50, y_position).into()) },
            Side::Left  => { self.entity.set_spawn((190, y_position).into()) },
        }
        self.entity.velocity.x = 0;
        self.entity.velocity.y = 0;
        self.entity.sprite.show();
    }

    pub fn start_ball(&mut self, last_score: Side) {
        if last_score == Side::Right {
            self.entity.velocity.x = random_number_between(2,3);
            self.entity.velocity.y = random_number_between(2,3);
        } else {
            self.entity.velocity.x = random_number_between(-3,-2);
            self.entity.velocity.y = random_number_between(-3,-2);
        }
    }
}