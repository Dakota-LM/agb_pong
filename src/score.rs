use crate::entity::Entity;
use crate::NUM_GRAPHICS;
use agb::{
    self, display::object::{OamManaged,Tag},
    fixnum::Vector2D,
};

// MAKE REFERENCE OBJECTS TO NUMBER SPRITES:
pub static ZERO:   &Tag = NUM_GRAPHICS.tags().get("Zero_GR");
pub static ONE:    &Tag = NUM_GRAPHICS.tags().get("One_GR");
pub static TWO:    &Tag = NUM_GRAPHICS.tags().get("Two_GR");
pub static THREE:  &Tag = NUM_GRAPHICS.tags().get("Three_GR");
pub static FOUR:   &Tag = NUM_GRAPHICS.tags().get("Four_GR");
pub static FIVE:   &Tag = NUM_GRAPHICS.tags().get("Five_GR");
pub static SIX:    &Tag = NUM_GRAPHICS.tags().get("Six_GR");
pub static SEVEN:  &Tag = NUM_GRAPHICS.tags().get("Seven_GR");
pub static EIGHT:  &Tag = NUM_GRAPHICS.tags().get("Eight_GR");
pub static NINE:   &Tag = NUM_GRAPHICS.tags().get("Nine_GR");
pub static TEN:    &Tag = NUM_GRAPHICS.tags().get("Ten_GR");
pub static ELEVEN: &Tag = NUM_GRAPHICS.tags().get("Eleven_GR");

pub struct Score<'a> {
    pub entity: Entity<'a>,
    pub sprite: &'a Tag,  //This could maybe be turned into a Box so that sprites could be positioned next to each other
    pub score: u16,
}

impl<'a> Score <'a> {

    pub fn new(object: &'a OamManaged, target: &Tag, tag: &'a Tag, pos: Vector2D<i32>) -> Self {
        let mut score: Entity = Entity::new(object, target, (16_u16, 16_u16).into());
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
        let mut sprite_ref: &Tag = self.sprite;

        match self.score {
            0   => sprite_ref = ZERO,
            1   => sprite_ref = ONE,
            2   => sprite_ref = TWO,
            3   => sprite_ref = THREE,
            4   => sprite_ref = FOUR,
            5   => sprite_ref = FIVE,
            6   => sprite_ref = SIX,
            7   => sprite_ref = SEVEN,
            8   => sprite_ref = EIGHT,
            9   => sprite_ref = NINE,
            10  => sprite_ref = TEN,
            11  => sprite_ref = ELEVEN,
            _   => sprite_ref = ZERO,
        }
        if self.score <= max {
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