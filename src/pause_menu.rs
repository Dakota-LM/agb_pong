// Until this works, make sure to comment everything out to make the game playable

#[allow(unused_imports)]
use agb::{
    display::{
        object::{
            OamIterator, ObjectTextRender, ObjectUnmanaged, PaletteVram, Size, SpriteLoader,
            SpriteVram, TextAlignment,
        },
        palette16::Palette16,
        tiled::{MapLoan, RegularMap, TiledMap, VRamManager},
        HEIGHT,
    },
    fixnum::Vector2D,
    input::{Button, ButtonController, Tri},
};

#[allow(unused_imports)]
use core::cell::RefCell;

// pub struct Menu {
//     pub graphic: &Tag,
//     pub state: bool,
// }

// impl Menu {
//     pub fn new(object: &'a OamUnManaged, target: &Tag) -> Self {
//         let mut paddle_top: Entity = Entity::new(object, target, paddle_collision_mask);
//         paddle_top.sprite.set_sprite(object.sprite(PADDLE_END.sprite(0)));
//         paddle_top.set_spawn((x_pos_of_paddle, 34).into());

//         Menu {
//             graphic: (),
//             state: false,
//         }
//     }

//     pub fn pause (mut self) {
//         if self.state == false {
//             self.state = true;
//             self.graphic.show()
//         }
//     }

//     pub fn unpause (self) {
//         if self.state == false {
//             self.state = true;
//             self.graphic.show()
//         }  
//     }
// }


// pub struct Pausable<'a, 'b> {
//     paused: Paused,
//     menu: PauseMenu,
//     game: Game<'a, 'b>,
// }

// #[derive(Clone, Copy, PartialEq, Eq)]
// enum Paused {
//     Paused,
//     Playing,
// }

// impl Paused {
//     fn change(self) -> Paused {
//         match self {
//             Paused::Paused => Paused::Playing,
//             Paused::Playing => Paused::Paused,
//         }
//     }
// }

// #[derive(Clone, Copy)]
// pub enum PauseSelection {
//     Continue,
//     Save,
//     Restart,
// }

// enum PauseSelectionInner {
//     Continue,
//     Save,
//     Restart,
// }

// struct PauseMenu {
//     option_text: RefCell<[ObjectTextRender<'static>; 2]>,
//     selection: PauseSelectionInner,
//     indicator_sprite: SpriteVram,
// }

// impl PauseMenu {
//     fn text_at_position(
//         text: core::fmt::Arguments,
//         position: Vector2D<i32>,
//     ) -> ObjectTextRender<'static> {
//         let mut t = ObjectTextRender::new(&FONT, Size::S32x16, generate_text_palette());

//         let _ = writeln!(t, "{}", text);
//         t.layout(Vector2D::new(i32::MAX, i32::MAX), TextAlignment::Left, 0);
//         t.next_line();
//         t.update(position);
//         t
//     }

//     fn new(loader: &mut SpriteLoader, maximum_level: usize, current_level: usize) -> Self {
//         PauseMenu {
//             option_text: RefCell::new([
//                 Self::text_at_position(format_args!("Restart"), Vector2D::new(32, HEIGHT / 4)),
//                 Self::text_at_position(
//                     format_args!("Go to level: {}", current_level + 1),
//                     Vector2D::new(32, HEIGHT / 4 + 20),
//                 ),
//             ]),
//             selection: PauseSelectionInner::Restart,
//             indicator_sprite: loader.get_vram_sprite(ARROW_RIGHT.sprite(0)),
//             selected_level: current_level,
//             maximum_level,
//         }
//     }
// }