use crate::entity::Entity;
use agb::{
    rng::RandomNumberGenerator,
    fixnum::{Vector2D, Rect},
};

/// Checks if two entities have collided with each other
pub fn intersects(ball: &Entity, paddle_part: &Entity) -> bool {
    let paddle_position = Vector2D::new(paddle_part.position.x,paddle_part.position.y);
    let paddle_size = Vector2D::new(16,16);
    let paddle_rect = Rect::new(paddle_position, paddle_size);
    let ball_position = Vector2D::new(ball.position.x, ball.position.y);
    let ball_size = Vector2D::new(14,14);
    let ball_rect = Rect::new(ball_position, ball_size);

    ball_rect.touches(paddle_rect)
}

pub fn random_number_between(min: i32, max: i32) -> i32 {
    // let mut random_number = RandomNumberGenerator::new_with_seed([16,32,64,128]);
    let mut random_number = RandomNumberGenerator::new();
    let generated_number = random_number.gen();
    match generated_number {
        n if n > max => (random_number.gen() % max) + min,
        n if n < min => (random_number.gen() % max).abs() + min,
        n if min < n && n < max => generated_number,
        _ => panic!("Random Number Generator"),
    }
}