use raylib::prelude::*;

// order to follow is: init window, init game
// then, within the game loop, update game and draw game
// and finally unload the game (unsure if needed in rust) and close da window

const TARGET_FPS: u32 = 60; // this is a hard fps cap
const PLAYER_SPEED: u32 = 60; // hardcoded but might change in future
const SCREEN_WIDTH: i32 = 640;
const SCREEN_HEIGHT: i32 = 640;
                              
fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("gaem")
        .build();

    rl.set_target_fps(TARGET_FPS);

    while !rl.window_should_close() { // which contains our game loop!
        // update_game(&mut game, &rl);
        draw_game(/*&mut game,*/ &mut rl, &thread)
    }
}

fn init_game(/*game: &mut Game*/ rl: &RaylibHandle) {
    todo!();
}

fn update_game(/*game: &mut Game*/ rl: &RaylibHandle) {
    todo!();
}

fn draw_game(/*game: &mut Game*/ rl: &mut RaylibHandle, thread: &RaylibThread) {
    let mut d = rl.begin_drawing(&thread);

    d.clear_background(Color::BLACK);

    d.draw_rectangle(SCREEN_WIDTH/2, SCREEN_WIDTH/2, SCREEN_WIDTH/21, SCREEN_HEIGHT/21, Color::PINK);
}

// structs
struct Game {
    player: Player,
    game_over: bool,
    pause: bool
}

struct Player {
    body: Vec<Segment>
}

struct Segment {
    position: (u8, u8),
    direction: Direction
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}
