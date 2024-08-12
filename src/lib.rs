#![no_std]
#![no_main]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]
#![deny(clippy::all)]

/*
    Build:
        cargo build --release

    Run in mGBA-qt:
        cargo run --release

    Convert binary to gba file:
        agb-gbafix target\thumbv4t-none-eabi\release\<filename> -o <filename>.gba
        example: agb-gbafix target\thumbv4t-none-eabi\release\pong -o Pong.gba
*/

#[allow(unused_imports)]
use agb::{
    self, display::{
        object::{Graphics, OamManaged, Object, SpriteVram, Tag},
        Priority
    }, fixnum::Vector2D, include_aseprite, input::{Button, ButtonController}
};


extern crate alloc;
// use alloc::vec::Vec;
// use alloc::boxed::Box;


/*
    TODO:
        - Text for win/lose conditions
        - Text with "press A to start again"
        - Better paddle collisions (sides are vulnerable to "carrying" the ball across the length of the paddle)
        - Better paddle AI (allow different speeds, as well as pauses)
        - Randomize start location of the ball (currently the ball spawns in the same spot with the same directional velocity making moves very predictable)
        - Currently highest possible score is 9. Would like to make it 11 for authenticity, but not sure how to have side by side sprites that update
*/


// IMPORT SPRITES
static GRAPHICS:     &Graphics = include_aseprite!("gfx/sprites.aseprite");
static NUM_GRAPHICS: &Graphics = include_aseprite!("gfx/numbers.aseprite");

// MAKE REFERENCE OBJECTS TO BALL & PADDLES
static PADDLE_END:   &Tag      = GRAPHICS.tags().get("Paddle End");
static PADDLE_MID:   &Tag      = GRAPHICS.tags().get("Paddle Mid");
static BALL:         &Tag      = GRAPHICS.tags().get("Ball");

// MAKE REFERENCE OBJECTS TO NUMBER SPRITES:
static ZERO:         &Tag      = NUM_GRAPHICS.tags().get("Zero_GR");
static ONE:          &Tag      = NUM_GRAPHICS.tags().get("One_GR");
static TWO:          &Tag      = NUM_GRAPHICS.tags().get("Two_GR");
static THREE:        &Tag      = NUM_GRAPHICS.tags().get("Three_GR");
static FOUR:         &Tag      = NUM_GRAPHICS.tags().get("Four_GR");
static FIVE:         &Tag      = NUM_GRAPHICS.tags().get("Five_GR");
static SIX:          &Tag      = NUM_GRAPHICS.tags().get("Six_GR");
static SEVEN:        &Tag      = NUM_GRAPHICS.tags().get("Seven_GR");
static EIGHT:        &Tag      = NUM_GRAPHICS.tags().get("Eight_GR");
static NINE:         &Tag      = NUM_GRAPHICS.tags().get("Nine_GR");

static MAX_SCORE:    u16       = 9;


pub fn main(mut gba: agb::Gba) -> ! {
    // Get the object manager
    let object:           OamManaged<'_>   = gba.display.object.get_managed();
    // Create an input object
    let mut input:        ButtonController = agb::input::ButtonController::new();

    let mut ball:         Ball   = Ball::new(&object);
    let mut right_paddle: Paddle = Paddle::new(&object, Side::Right);
    let mut left_paddle:  Paddle = Paddle::new(&object, Side::Left);
    let mut player_score: Score  = Score::new(&object, ZERO, (88,5).into());
    let mut ai_score:     Score  = Score::new(&object, ZERO, (136,5).into());

    loop {
        // This will calculate the new position and enforce the position
        // of the entities remains within the screen
        ball.checks_and_keeps_in_bounds();
        left_paddle.checks_and_keeps_in_bounds();
        right_paddle.checks_and_keeps_in_bounds();

        // We check if the ball reaches the edge of the screen and reverse it's direction
        ball.bounce_if_hits_screen_bounds();

        //Simple collision detection that is quite faulty at times, but it works for learning
        left_paddle.checks_all_collisions(&mut ball);
        right_paddle.checks_all_collisions(&mut ball);
        
        if ball.entity.position.x < -15 {
            ball.entity.velocity.x = 0;
            ball.entity.velocity.y = 0;
            ai_score.increment(&object, (136, 5).into(), MAX_SCORE);
            
            ball.spawn_to_centre();
            ball.start_ball();
        }
        if ai_score.score == MAX_SCORE {
            ball.entity.sprite.hide();
            object.commit();
            loop {
                if input.is_pressed(Button::A){
                    // TODO: write who wins here
                    
                    player_score.reset_score(&object, (88,5).into());
                    ai_score.reset_score(&object, (136, 5).into());
                    ball.entity.sprite.show();
                    break;
                }
                input.update();
            }
        }

        if ball.entity.position.x > agb::display::WIDTH - 1 {
            ball.entity.velocity.x = 0;
            ball.entity.velocity.y = 0;
            player_score.increment(&object, (88,5).into(), MAX_SCORE);
            
            ball.spawn_to_centre();
            ball.start_ball();
        }

        if player_score.score == MAX_SCORE {
            ball.entity.sprite.hide();
            object.commit();
            loop {
                if input.is_pressed(Button::A){
                    // TODO: write who wins here
                    player_score.reset_score(&object, (88,5).into());
                    ai_score.reset_score(&object, (136, 5).into());
                    ball.entity.sprite.show();
                    break;
                }
                input.update();
            }
        }

        //Updates sprites with input
        // Set the position of the ball to match our new calculated position
        ball.entity.update_sprite_position();
        left_paddle.move_paddle_with_input(input.y_tri() as i32);

        // ball speed is 2 so reduced ai speed to 1 to allow some scoring by player
        right_paddle.update_ai_paddle(&ball.entity, 1);

        // Wait for vblank, then commit the objects to the screen
        agb::display::busy_wait_for_vblank();
        object.commit();
        input.update()
}
    /// Ball struct that holds the sprite of the ball
    pub struct Ball<'a> {
        entity: Entity<'a>,
    }

    /// Impl of ball to allow for methods to interact with the sprite
    impl<'a> Ball<'a> {
        pub fn new(object: &'a OamManaged) -> Self {
            let mut ball: Entity = Entity::new(object, (16_u16, 16_u16).into());
            ball.sprite.set_sprite(object.sprite(BALL.sprite(0)));
            ball.velocity.x = 2;  // switched these from 1 to make it harder for the ai paddle
            ball.velocity.y = 2;
            ball.set_spawn((50, 50).into());
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
        
        pub fn spawn_to_centre(&mut self) {
            self.entity.sprite.hide();
            self.entity.set_spawn((50, 50).into());
            self.entity.sprite.show();
        }

        pub fn start_ball(&mut self) {
            self.entity.velocity.x = 2;
            self.entity.velocity.y = 2;
        }
    }

    /// Which side of the screen the sprint is on
    pub enum Side {
        Left,
        Right,
    }

    /// A simple entity struct that holds the sprite and position for a paddle object
    pub struct Paddle<'a> {
        top: Entity<'a>,
        middle: Entity<'a>,
        bottom: Entity<'a>,
        velocity: Vector2D<i32>,
    }

    /// Impl of paddle to allow for methods to interact with the sprite and setup
    /// The paddle is made up of 3 sprites, top, middle and bottom.
    impl<'a> Paddle<'a> {
        pub fn new(object: &'a OamManaged, which_side: Side) -> Self {
            let x_pos_of_paddle = match which_side {
                Side::Left => 1,
                Side::Right => 224,
            };

            let paddle_collision_mask: Vector2D<u16> = (14_u16, 14_u16).into();

            let mut paddle_middle: Entity = Entity::new(object, paddle_collision_mask);
            paddle_middle
                .sprite
                .set_sprite(object.sprite(PADDLE_MID.sprite(0)));
            paddle_middle.velocity.y = 3;

            paddle_middle.set_spawn((x_pos_of_paddle, 50).into());
            paddle_middle.sprite.show();

            let mut paddle_top: Entity = Entity::new(object, paddle_collision_mask);
            paddle_top
                .sprite
                .set_sprite(object.sprite(PADDLE_END.sprite(0)));
            paddle_top.velocity.y = 3;
            paddle_top.set_spawn((x_pos_of_paddle, 34).into());
            paddle_top.sprite.show();

            let mut paddle_bottom: Entity = Entity::new(object, paddle_collision_mask);
            paddle_bottom
                .sprite
                .set_sprite(object.sprite(PADDLE_END.sprite(0)));
            paddle_bottom.velocity.y = 3;
            paddle_bottom.sprite.set_vflip(true);
            paddle_bottom.set_spawn((x_pos_of_paddle, 66).into());
            paddle_bottom.sprite.show();

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
            }
        }

        /// Checks to make sure the paddle is within the bounds of the screen
        pub fn checks_and_keeps_in_bounds(&mut self) {
            self.top.position.y = (self.top.position.y + self.top.velocity.y)
                .clamp(0, agb::display::HEIGHT - 48);
            self.middle.position.y = (self.middle.position.y + self.middle.velocity.y)
                .clamp(16, agb::display::HEIGHT - 32);
            self.bottom.position.y = (self.bottom.position.y + self.bottom.velocity.y)
                .clamp(32, agb::display::HEIGHT - 16);
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

        /// Checks if any of the three sprites has collided with the ball and bounces it back
        pub fn checks_all_collisions(&mut self, ball: &mut Ball) {
            if intersects(&ball.entity, &self.top) {
                ball.entity.velocity.x = -ball.entity.velocity.x;
                return;
            }

            if intersects(&ball.entity, &self.middle) {
                ball.entity.velocity.x = -ball.entity.velocity.x;
                return;
            }

            if intersects(&ball.entity, &self.bottom) {
                ball.entity.velocity.x = -ball.entity.velocity.x;
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

    pub struct Score<'a> {
        entity: Entity<'a>,
        sprite: &'a Tag,
        score: u16,
    }

    impl<'a> Score <'a> {

        pub fn new(object: &'a OamManaged, tag: &'a Tag, pos: Vector2D<i32>) -> Self {
            let mut score: Entity = Entity::new(object, (16_u16, 16_u16).into());
            score.sprite.set_sprite(object.sprite(tag.sprite(0)));
            score.set_spawn(pos);
            score.sprite.show();
            Score {
                entity: score,
                sprite: tag,
                score: 0
            }
        }

        #[allow(unused_assignments)]
        pub fn increment(&mut self, object: &'a OamManaged, pos: Vector2D<i32>, max: u16) {
            self.score += 1;
            // let mut sprite_ref: &Tag = self.sprite1;
            let mut sprite_ref: &Tag = self.sprite;

            match self.score {
                0 => sprite_ref = ZERO,
                1 => sprite_ref = ONE,
                2 => sprite_ref = TWO,
                3 => sprite_ref = THREE,
                4 => sprite_ref = FOUR,
                5 => sprite_ref = FIVE,
                6 => sprite_ref = SIX,
                7 => sprite_ref = SEVEN,
                8 => sprite_ref = EIGHT,
                9 => sprite_ref = NINE,
                _ => sprite_ref = ZERO,
            }
            if self.score < max {
                self.sprite = sprite_ref;
                self.entity.sprite.set_sprite(object.sprite(sprite_ref.sprite(0)));
                self.entity.set_spawn(pos);
                self.entity.sprite.show();
            }
        }

        pub fn reset_score(&mut self, object: &'a OamManaged, pos: Vector2D<i32>) {
            self.score = 0;
            self.entity.sprite.set_sprite(object.sprite(ZERO.sprite(0)));
            self.entity.set_spawn(pos);
            self.entity.sprite.show();
        }
    }

    /// A simple entity struct that holds the sprite and position for any sprite
    pub struct Entity<'a> {
        sprite: Object<'a>,
        position: Vector2D<i32>,
        velocity: Vector2D<i32>,
        collision_mask: Vector2D<u16>,
    }

    /// impl of entity to allow for methods to interact with the sprite and setup
    impl<'a> Entity<'a> {
        pub fn new(object: &'a OamManaged, collision_mask: Vector2D<u16>) -> Self {
            let mut dummy_object = object.object_sprite(BALL.sprite(0));

            dummy_object.set_priority(Priority::P1);
            Entity {
                sprite: dummy_object,
                collision_mask,
                position: (0, 0).into(),
                velocity: (12_u16, 48_u16).into(),
            }
        }

        /// Updates the position of the sprite based on what has been set in the position variable
        fn update_sprite_position(&mut self) {
            self.sprite
                .set_x(self.position.x as u16)
                .set_y(self.position.y as u16);
        }

        /// Set where the entity should spawn the sprite
        fn set_spawn(&mut self, spawn: Vector2D<i32>) {
            self.position = spawn;
            self.sprite
                .set_x(self.position.x as u16)
                .set_y(self.position.y as u16);
        }
    }

    /// Checks if two entities have collided with each other
    fn intersects(e1: &Entity, e2: &Entity) -> bool {
        let e1_right = e1.position.x + e1.collision_mask.x as i32;
        let e1_bottom = e1.position.y + e1.collision_mask.y as i32;
        let e2_right = e2.position.x + e2.collision_mask.x as i32;
        let e2_bottom = e2.position.y + e2.collision_mask.y as i32;

        e1.position.x < e2_right
            && e1_right > e2.position.x
            && e1.position.y < e2_bottom
            && e1_bottom > e2.position.y
    }
}
