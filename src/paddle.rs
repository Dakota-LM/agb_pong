use crate::GRAPHICS;
use crate::entity::Entity;
use crate::ball::Ball;
use crate::common::intersects;
use agb::{
    self, display::object::{OamManaged,Tag},
    fixnum::Vector2D,
};

// MAKE REFERENCE OBJECTS TO PADDLE SPRITES
pub static PADDLE_END:   &Tag      = GRAPHICS.tags().get("Paddle End");
pub static PADDLE_MID:   &Tag      = GRAPHICS.tags().get("Paddle Mid");

// Which side of the screen the sprint is on
#[derive(PartialEq)]
pub enum Side {
    Left,
    Right,
}
/// A simple entity struct that holds the sprite and position for a paddle object
pub struct Paddle<'a> {
    pub top: Entity<'a>,
    pub middle: Entity<'a>,
    pub bottom: Entity<'a>,
    pub velocity: Vector2D<i32>,
    pub side: Side,
}

/// Impl of paddle to allow for methods to interact with the sprite and setup
/// The paddle is made up of 3 sprites, top, middle and bottom.
impl<'a> Paddle<'a> {
    pub fn new(object: &'a OamManaged, target: &Tag, which_side: Side) -> Self {
        let x_pos_of_paddle = match which_side {
            Side::Left => 1,
            Side::Right => 224,
        };

        let paddle_collision_mask: Vector2D<u16> = (16_u16, 16_u16).into();

        let mut paddle_middle: Entity = Entity::new(object, target, paddle_collision_mask);
        paddle_middle.sprite.set_sprite(object.sprite(PADDLE_MID.sprite(0)));
        paddle_middle.velocity.y = 3;
        paddle_middle.set_spawn((x_pos_of_paddle, 50).into());
        paddle_middle.sprite.show();

        let mut paddle_top: Entity = Entity::new(object, target, paddle_collision_mask);
        paddle_top.sprite.set_sprite(object.sprite(PADDLE_END.sprite(0)));
        paddle_top.velocity.y = 3;
        paddle_top.set_spawn((x_pos_of_paddle, 34).into());
        paddle_top.sprite.show();

        let mut paddle_bottom: Entity = Entity::new(object, target, paddle_collision_mask);
        paddle_bottom.sprite.set_sprite(object.sprite(PADDLE_END.sprite(0)));
        paddle_bottom.velocity.y = 3;
        paddle_bottom.set_spawn((x_pos_of_paddle, 66).into());
        paddle_bottom.sprite.show();
        paddle_bottom.sprite.set_vflip(true);

        if matches!(which_side, Side::Right) {
            paddle_top.sprite.set_hflip(true);
            paddle_middle.sprite.set_hflip(true);
            paddle_bottom.sprite.set_hflip(true);
        }

        Paddle {
            top: paddle_top,
            middle: paddle_middle,
            bottom: paddle_bottom,
            velocity: (0, 0).into(),
            side: which_side,
        }
    }

    /// Checks to make sure the paddle is within the bounds of the screen
    pub fn checks_and_keeps_in_bounds(&mut self) {
        self.top.position.y    = (self.top.position.y + self.top.velocity.y).clamp(0, agb::display::HEIGHT - 48);
        self.middle.position.y = (self.middle.position.y + self.middle.velocity.y).clamp(16, agb::display::HEIGHT - 32);
        self.bottom.position.y = (self.bottom.position.y + self.bottom.velocity.y).clamp(32, agb::display::HEIGHT - 16);
    }

    /// Moves the paddle based on the input of the y axis of the dpad
    pub fn move_paddle_with_input(&mut self, y_input: i32) {
        self.top.velocity.y = y_input;
        self.middle.velocity.y = y_input;
        self.bottom.velocity.y = y_input;

        self.top.update_sprite_position();
        self.middle.update_sprite_position();
        self.bottom.update_sprite_position();
    }

    pub fn checks_all_collisions(&mut self, ball: &mut Ball) {
        let ball_half_width = 7; // Half of the ball's width (14/2)
        let ball_half_height = 7; // Assuming the ball is square
        let paddle_half_width = 8; // Half of the paddle's width (16/2)
        let paddle_half_height = 24; // Half of the paddle's height (48/2)
        let offset = 1; // Small offset to prevent sticking

        // Function to check collision using SAT
        fn check_collision(ball: &Ball, paddle: &Entity, is_left_side: bool) -> Option<(bool, bool)> {
            let ball_center = ball.entity.position + Vector2D::new(ball_half_width, ball_half_height);
            let paddle_center = paddle.position + Vector2D::new(paddle_half_width, paddle_half_height);

            let dx = (ball_center.x - paddle_center.x).abs();
            let dy = (ball_center.y - paddle_center.y).abs();

            if dx > (ball_half_width + paddle_half_width) || dy > (ball_half_height + paddle_half_height) {
                return None;
            }

            let overlap_x = ball_half_width + paddle_half_width - dx;
            let overlap_y = ball_half_height + paddle_half_height - dy;

            let horizontal_collision = overlap_x < overlap_y;
            let from_left = if is_left_side { ball_center.x > paddle_center.x } else { ball_center.x < paddle_center.x };

            Some((horizontal_collision, from_left))
        }

        // Check collision with each part of the paddle
        for paddle_part in [&self.top, &self.middle, &self.bottom].iter() {
            if let Some((horizontal_collision, from_left)) = check_collision(ball, paddle_part, self.side == Side::Left) {
                if horizontal_collision {
                    ball.entity.velocity.x = -ball.entity.velocity.x;
                    if self.side == Side::Left {
                        ball.entity.position.x = paddle_part.position.x + 16 + offset;
                    } else {
                        ball.entity.position.x = paddle_part.position.x - 14 - offset;
                    }
                } else {
                    ball.entity.velocity.y = -ball.entity.velocity.y;
                    if from_left {
                        ball.entity.position.y = paddle_part.position.y - 14 - offset;
                    } else {
                        ball.entity.position.y = paddle_part.position.y + 16 + offset;
                    }
                }

                // Add a small random adjustment to the ball's velocity
                use agb::rand::Rng;
                let mut rng = agb::rand::Rng::new();
                let random_adjust = rng.gen_range(-1..=1);
                if ball.entity.velocity.x.abs() > 1 {
                    ball.entity.velocity.x += random_adjust;
                }
                if ball.entity.velocity.y.abs() > 1 {
                    ball.entity.velocity.y += random_adjust;
                }

                return; // Exit after handling the collision
            }
        }
    }

    // This function will make the AI paddle move towards the ball.
    pub fn update_ai_paddle(&mut self, ball: &Entity, speed: i32) {
        match ball.position.y {
            x if x < self.middle.position.y => self.velocity.y = -speed,
            x if x > self.middle.position.y => self.velocity.y = speed,
            _ => self.velocity.y = 0
        }

        self.move_paddle_with_input(self.velocity.y);
    }

}
}