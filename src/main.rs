use raylib::prelude::*;
use rand::*;

// order to follow is: init window, init game
// then, within the game loop, update game and draw game
// and finally unload the game (unsure if needed in rust) and close da window

const TARGET_FPS: u32 = 4; // this is a hard fps cap
const PLAYER_SPEED: u32 = 60; // hardcoded but might change in future
const SCREEN_WIDTH: i32 = 840;
const SCREEN_HEIGHT: i32 = 840;

// let score: i32 = 0; // also keeps track of how many segments the snake has!! handy
                              
fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("gaem")
        .build();

    rl.set_target_fps(TARGET_FPS);

    // TODO: this is to be replaced with an actual game func!!
    let mut game = Game::default();

    while !rl.window_should_close() { // which contains our game loop!
        update_game(&mut game, &mut rl);
        draw_game(&mut game, &mut rl, &thread)
    }
}

fn init_game(game: &mut Game, rl: &RaylibHandle) {
    // TODO: well so far this is not needed but i'll keep it here just in case 
    todo!();
}

fn update_game(game: &mut Game, rl: &RaylibHandle) {

    // boilerplate
    use raylib::consts::KeyboardKey::*;
    let mut rng = rand::thread_rng();

    // checks so that snake controls like snake
    if rl.is_key_down(KEY_W) && game.player.body[0].direction != Direction::DOWN {
        game.player.body[0].direction = Direction::UP;
        println!("up we go!");
    } else if rl.is_key_down(KEY_A) && game.player.body[0].direction != Direction::RIGHT {
        game.player.body[0].direction = Direction::LEFT;
        println!("left we go!");
    } else if rl.is_key_down(KEY_S) && game.player.body[0].direction != Direction::UP {
        game.player.body[0].direction = Direction::DOWN;
        println!("down we go!");
    } else if rl.is_key_down(KEY_D) && game.player.body[0].direction != Direction::LEFT {
        game.player.body[0].direction = Direction::RIGHT;
        println!("right we go!");
    }

    // NOTE TO SELF: an idea of how this could work might be having the new segment remain
    // static until it detects no segment of the array shares its same position, then start
    // moving once it's free (?)
    
    let mut immobile: bool = false;

    // TODO: iteration
    for i in 0..game.player.body.len() { // the segment we're actually moving 
        for j in 0..game.player.body.len() { // emptiness check! 
            if game.player.body[i].position == game.player.body[j].position && i < j {
                // in this case, the segment we want to move has to be BEFORE the similar segment
                // therefore, no movement
                immobile = true;
                println!("whoopsies! segment overlap");
            }
        }

        if !immobile{
            match game.player.body[i].direction {
                Direction::UP    => game.player.body[i].position.1 -= 1,
                Direction::DOWN  => game.player.body[i].position.1 += 1,
                Direction::LEFT  => game.player.body[i].position.0 -= 1,
                Direction::RIGHT => game.player.body[i].position.0 += 1,
            }
        }

        immobile = false;

        println!("segment {0} position is {1},{2}", i, game.player.body[i].position.0, game.player.body[i].position.1);
    }

    // is food being eaten??
    if game.player.body[0].position.0 == game.food.position.0 && game.player.body[0].position.1 == game.food.position.1 {
        // behold! an segment
        game.player.body.push(
            Segment::new(
                (game.player.body[game.player.body.len()-1].position.0, game.player.body[game.player.body.len()-1].position.1),
                game.player.body[0].direction.clone()
            )
        );
        println!("new segment created at {0},{1}", game.player.body[game.player.body.len()-1].position.0, game.player.body[game.player.body.len()-1].position.1);

        game.food.position.0 = rng.gen_range(1..21);
        game.food.position.1 = rng.gen_range(1..21);
    }
    // TODO: are you going ouroboros mode??
    
    // this is out here because updates to segment direction should happen AFTER segments move,
    // iteratively - otherwise all segments would change direction at once and the game would be very silly
    for i in 1..game.player.body.len() {
        // segments move in the direction of the preceding segment, unless that segment
        // is the first (because you wouldn't move otherwise, you dingus)
        game.player.body[i].direction = game.player.body[i-1].direction.clone();
    }

    // out of bounds check 
    if game.player.body[0].position.0 > 21 || game.player.body[0].position.0 < 0 || game.player.body[0].position.1 > 21 || game.player.body[0].position.1 < 0 {
        game.player.body[0].position.0 = 10;
        game.player.body[0].position.1 = 10;

        game.player.body.truncate(1); // and all your progress goes whoosh
    }
}

fn draw_game(game: &mut Game, rl: &mut RaylibHandle, thread: &RaylibThread) {
    let mut d = rl.begin_drawing(&thread);

    d.clear_background(Color::WHITE);

    for s in &game.player.body {
        d.draw_rectangle(
            s.position.0 * (SCREEN_WIDTH/21), 
            s.position.1 * (SCREEN_HEIGHT/21), 
            SCREEN_WIDTH/21, 
            SCREEN_HEIGHT/21, 
            Color::PINK
        );
    }

    d.draw_rectangle(
        game.food.position.0 * (SCREEN_WIDTH/21), 
        game.food.position.1 * (SCREEN_HEIGHT/21),
        SCREEN_WIDTH/21,
        SCREEN_HEIGHT/21,
        Color::ORANGE
    );
}

// structs
struct Game { // doesn't derive from Default because default() is an impl within !!
    player: Player,
    food: Food,
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

#[derive(PartialEq, Clone)]
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
        let food = Food::default();

        Game {
            game_over,
            pause,
            player,
            food
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

impl Segment {
    fn default() -> Self {
        let position = ( 10, 10 );
        let direction = Direction::UP;

        Self { position, direction }
    }

    fn new(new_position: (i32, i32), new_direction: Direction) -> Self {
        let position = new_position;
        let direction = new_direction;

        Self { position, direction }
    }
}

impl Default for Food {
    fn default() -> Self {
        let mut rng = rand::thread_rng();

        let position = (
            (rng.gen_range(1..21)),
            (rng.gen_range(1..21))
        );

        Self { position }
    }
}
