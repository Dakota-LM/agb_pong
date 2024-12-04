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

use agb::{
    self, display::{
        object::{Graphics,OamManaged},
        busy_wait_for_vblank
    },
    fixnum::Vector2D,
    include_aseprite,
    input::{Button, ButtonController},
};

mod pause_menu;
mod entity;
mod common;
mod score;
mod paddle;
mod ball;
use crate::{common::*,score::*,paddle::*,ball::*};

/*
    TODO:
        - Text for win/lose conditions   - Text seems to be done with ObjectTextRenderer and writeln!
        - Work on pause_menu.rs (Add Continue, Save, Restart, and maybe Credits)
        - Add splash screen
        - Add background
        - Add music
        - Add collision sounds
        - Fix collision bugs 
*/

// IMPORT SPRITES
pub static GRAPHICS:     &Graphics = include_aseprite!("gfx/sprites.aseprite");
pub static NUM_GRAPHICS: &Graphics = include_aseprite!("gfx/numbers.aseprite");

static MAX_SCORE: u16 = 11;

pub fn main(mut gba: agb::Gba) -> ! {
    // Get the object manager
    let object:           OamManaged<'_>   = gba.display.object.get_managed();

    // Create an input object
    let mut input:        ButtonController = agb::input::ButtonController::new();

    let random_min = 0;
    let random_max = agb::display::HEIGHT - 16;
    let mut ball:              Ball          = Ball::new(&object, BALL, random_number_between(random_min, random_max));
    let mut right_paddle:      Paddle        = Paddle::new(&object, BALL, Side::Right);
    let mut left_paddle:       Paddle        = Paddle::new(&object, BALL, Side::Left);
    let player_score_position: Vector2D<i32> = (88,5).into();
    let mut player_score:      Score         = Score::new(&object, BALL, ZERO, player_score_position);
    let ai_score_position:     Vector2D<i32> = (136,5).into();
    let mut ai_score:          Score         = Score::new(&object, BALL, ZERO, ai_score_position);

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
        
        // let random = random_number_between(random_min, random_max);
        // AI scores and the ball respawns to the centre
        if ball.entity.position.x < -15 {
            ai_score.increment(&object, ai_score_position, MAX_SCORE);
            ball.spawn_to_centre(random_number_between(random_min, random_max), Side::Right);
            ball.start_ball(Side::Right);
        }

        // Player scores and the ball respawns to the centre
        if ball.entity.position.x > agb::display::WIDTH - 1 {
            player_score.increment(&object, player_score_position, MAX_SCORE);
            ball.spawn_to_centre(random_number_between(random_min, random_max), Side::Left);
            ball.start_ball(Side::Left);
        }

        // If the AI reaches the max score, the ball disappears and the game hangs until the A button is pressed
        if ai_score.score == MAX_SCORE {
            ai_score.increment(&object, ai_score_position, MAX_SCORE);
            agb::println!("AI won at: {}", ai_score.score);
            ball.entity.sprite.hide();
            busy_wait_for_vblank();
            object.commit();
            loop {
                if input.is_pressed(Button::A){
                    // TODO: write who wins here
                    player_score.reset_score(&object, player_score_position);
                    ai_score.reset_score(&object, ai_score_position);
                    agb::display::busy_wait_for_vblank();
                    ball.entity.sprite.show();
                    break;
                }
                input.update();
            }
        }

        // If the Player reaches the max score, the ball disappears and the game hangs until the A button is pressed
        if player_score.score == MAX_SCORE {
            player_score.increment(&object, player_score_position, MAX_SCORE);
            agb::println!("Player won at: {}", player_score.score);
            ball.entity.sprite.hide();
            busy_wait_for_vblank();
            object.commit();
            loop {
                if input.is_pressed(Button::A){
                    // TODO: write who wins here
                    player_score.reset_score(&object, player_score_position);
                    ai_score.reset_score(&object, ai_score_position);
                    agb::display::busy_wait_for_vblank();
                    ball.entity.sprite.show();
                    break;
                }
                input.update();
            }
        }

        // Updates sprites with input
        // Set the position of the ball to match our new calculated position
        ball.entity.update_sprite_position();
        left_paddle.move_paddle_with_input(input.y_tri() as i32 * 2);

        // ball speed is 2 so reduced ai speed to 1 to allow some scoring by player
        right_paddle.update_ai_paddle(&ball.entity, 1);

        // Wait for vblank, then commit the objects to the screen
        busy_wait_for_vblank();
        object.commit();
        input.update();

    }
}
