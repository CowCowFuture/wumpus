use rand::Rng;

mod special;

const MIN_LEN: i8 = 0;
const MAX_LEN: i8 = 3;
const MIN_WID: i8 = 0;
const MAX_WID: i8 = 3;

fn main() {
    let mut game_loop = true;
    let mut player_pos = [0, 0];
    let mut arrow_count = 5;

    let wumpus_pos = [rand::thread_rng().gen_range(MIN_LEN..MAX_LEN), rand::thread_rng().gen_range(MIN_WID..MAX_WID)];

    println!("Arrows: {}", arrow_count);
    println!("{}, {}", player_pos[0], player_pos[1]);

    while game_loop {
        let mut input = String::new();
        input = special::read(input);

        if input == "q" {
            game_loop = false;
        }

        player_pos = player_move(player_pos, &input);
        
        for dir in ["u", "h", "j", "k"] {
            if &input == dir {
                arrow_count -= 1;
                if player_shoot(player_pos, wumpus_pos, &input) {
                    println!("You hit a Wumpus!");
                    game_loop = false;
                }
                else { println!("You missed!"); }
            }
        }
        if arrow_count <= 0 { game_loop = false; }
        
        // Print Stats
        println!("Arrows: {}", arrow_count);
        println!("{}, {}", player_pos[0], player_pos[1]);

    }
}

fn player_move(mut position: [i8; 2], direction: &String) -> [i8; 2] {
    if direction == "w" { position[1]+=1; }
    if direction == "a" { position[0]-=1; }
    if direction == "s" { position[1]-=1; }
    if direction == "d" { position[0]+=1; }
    let tempx = position[0];
    let tempy = position[1];
    position[0] = special::clamp(position[0], MIN_LEN, MAX_LEN);
    position[1] = special::clamp(position[1], MIN_WID, MAX_WID);
    position = [position[0], position[1]];
    if tempx != position[0] || tempy != position[1] { println!("You've bumped into a wall!"); }

    return position;
}

fn player_shoot(start_position: [i8; 2], target_position: [i8; 2], direction: &String) -> bool {
    let mut hit_wumpus = false;

    if direction == "u" {
        for point in start_position[1]..MAX_WID {
            if start_position[0] == target_position[0] && point == target_position[1] {
                hit_wumpus = true;
            }

        }
    }
    if direction == "h" {
        for point in MIN_LEN..start_position[0] {
            if start_position[1] == target_position[1] && point == target_position[0] {
                hit_wumpus = true;
            }

        }
    }
    if direction == "j" {
        for point in MIN_WID..start_position[1] {
            if start_position[0] == target_position[0] && point == target_position[1] {
                hit_wumpus = true;
            }

        }
    }
    if direction == "k" {
        for point in start_position[0]..MAX_LEN {
            if start_position[1] == target_position[1] && point == target_position[0] {
                hit_wumpus = true;
            }

        }
    }

    return hit_wumpus;
}
