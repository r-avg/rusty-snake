use raylib::prelude::*;

// order to follow is: init window, init game
// then, within the game loop, update game and draw game
// and finally unload the game (unsure if needed in rust) and close da window

const TARGET_FPS: u32 = 4; // this is a hard fps cap
const PLAYER_SPEED: u32 = 60; // hardcoded but might change in future
const SCREEN_WIDTH: i32 = 640;
const SCREEN_HEIGHT: i32 = 640;
                              
fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("gaem")
        .build();

    rl.set_target_fps(TARGET_FPS);

    // TODO: this is to be replaced with an actual game func!!
    let mut game = Game::default();

    while !rl.window_should_close() { // which contains our game loop!
        update_game(&mut game, &rl);
        draw_game(&mut game, &mut rl, &thread)
    }
}

fn init_game(game: &mut Game, rl: &RaylibHandle) {
    // TODO: spawn da food
    todo!();
}

fn update_game(game: &mut Game, rl: &RaylibHandle) {

    // basic controls

    use raylib::consts::KeyboardKey::*;

    if rl.is_key_down(KEY_W) && game.player.body[0].direction != Direction::DOWN {
        game.player.body[0].direction = Direction::UP;
    } else if rl.is_key_down(KEY_A) && game.player.body[0].direction != Direction::RIGHT {
        game.player.body[0].direction = Direction::LEFT;
    } else if rl.is_key_down(KEY_S) && game.player.body[0].direction != Direction::UP {
        game.player.body[0].direction = Direction::DOWN;
    } else if rl.is_key_down(KEY_D) && game.player.body[0].direction != Direction::LEFT {
        game.player.body[0].direction = Direction::RIGHT;
    }

    match game.player.body[0].direction {
        Direction::UP    => game.player.body[0].position.1 -= SCREEN_HEIGHT/21,
        Direction::DOWN  => game.player.body[0].position.1 += SCREEN_HEIGHT/21,
        Direction::LEFT  => game.player.body[0].position.0 -= SCREEN_HEIGHT/21,
        Direction::RIGHT => game.player.body[0].position.0 += SCREEN_HEIGHT/21,
    }

    // TODO: is food being eaten??
    // TODO: are you going ouroboros mode??
}

fn draw_game(game: &mut Game, rl: &mut RaylibHandle, thread: &RaylibThread) {
    let mut d = rl.begin_drawing(&thread);

    d.clear_background(Color::WHITE);

    d.draw_rectangle(
        game.player.body[0].position.0, 
        game.player.body[0].position.1, 
        SCREEN_WIDTH/21, 
        SCREEN_HEIGHT/21, 
        Color::PINK);
}

// structs
struct Game { // doesn't derive from Default because default() is an impl within !!
    player: Player,
    game_over: bool,
    pause: bool
}

struct Player {
    body: Vec<Segment>
}

struct Segment {
    position: (i32, i32),
    direction: Direction
}

struct Food {
    position: (i32, i32)
}

#[derive(PartialEq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

// impl
impl Default for Game {
    fn default() -> Game {
        let game_over = false;
        let pause = false;

        let player = Player::default();

        Game {
            game_over,
            pause,
            player
        }
    }
}

impl Default for Player { // basically a constructor
    fn default() -> Self { 
        let mut body: Vec<Segment> = vec![];

        let mut segment = Segment::default(); 

        body.push(segment);

        Self { body }
    }
}

impl Default for Segment {
    fn default() -> Self {
        let position = ( SCREEN_HEIGHT/2, SCREEN_WIDTH/2 );
        let direction = Direction::UP;

        Self { position, direction }
    }
}

impl Default for food {
    fn default() -> Self {
        let position = (
            // this has to be something that can be divided by 21
        );

        Self { position }
    }
}
