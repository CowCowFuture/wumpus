use rand::Rng;

mod special;

const MIN_LEN: i8 = 0;
const MAX_LEN: i8 = 5;
const MIN_WID: i8 = 0;
const MAX_WID: i8 = 5;

fn main() {
    let mut game_loop = true;
    let mut player_pos = [rand::thread_rng().gen_range(MIN_LEN..MAX_LEN), rand::thread_rng().gen_range(MIN_WID..MAX_WID)];
    let mut arrow_count = 5;

    let wumpus_pos = [rand::thread_rng().gen_range(MIN_LEN..MAX_LEN), rand::thread_rng().gen_range(MIN_WID..MAX_WID)];
    println!("-------------------------------------------------");
    println!("Use [WASD] to walk.");
    println!("Use [UHJK] to shoot.");
    println!("When the Wumpus is nearby you will get a message.");
    println!("Don't run out of arrows!");
    println!("Press enter to begin!");
    println!("-------------------------------------------------");
    println!("");

    while game_loop {
        // Print Stats
        // println!("wu: {}, {}", wumpus_pos[0], wumpus_pos[1]);
        

        println!("------------------------");
        println!("You have {} arrows left.", arrow_count);
        println!("You are at {}, {}.", player_pos[0], player_pos[1]);
        println!("------------------------");
        
        if smell_animal(wumpus_pos, player_pos) {
            println!("I smell a Wumpus...");
        }
        
        let mut input = String::new();
        input = special::read(input);

        if input == "q" {
            game_loop = false;
        }
        
        for mov in input.chars() {
            player_pos = player_move(player_pos, &String::from(mov));
            if animal_eat(wumpus_pos, player_pos) {
                println!("You were eaten by the Wumpus!");
                game_loop = false;
            }
        }
        
        for dir in ["u", "h", "j", "k"] {
            if &input == dir {
                arrow_count -= 1;
                if player_shoot(player_pos, wumpus_pos, &input) {
                    println!("__   __            _    _ _ _          _   _   _");
                    println!("\\ \\ / /__  _   _  | | _(_) | | ___  __| | | |_| |__   ___ ");
                    println!(" \\ V / _ \\| | | | | |/ / | | |/ _ \\/ _` | | __| '_ \\ / _ \\");
                    println!("  | | (_) | |_| | |   <| | | |  __/ (_| | | |_| | | |  __/");
                    println!("  |_|\\___/ \\__,_| |_|\\_\\_|_|_|\\___|\\__,_|  \\__|_| |_|\\___|");
                    println!("");
                    println!("\\ \\      / /   _ _ __ ___  _ __  _   _ ___| |");
                    println!(" \\ \\ /\\ / / | | | '_ ` _ \\| '_ \\| | | / __| |");
                    println!("  \\ V  V /| |_| | | | | | | |_) | |_| \\__ \\_|");
                    println!("   \\_/\\_/  \\__,_|_| |_| |_| .__/ \\__,_|___(_)");
                    println!("                          |_|                "); /* Should print "You killed the Wumpus!" in large text */
                    game_loop = false;
                }
                else { println!("You missed!"); }
            }
        }
        if arrow_count <= 0 { game_loop = false; }
        
    }
}

fn player_move(mut position: [i8; 2], direction: &String) -> [i8; 2] {
    if direction == "w" { position[1]+=1; }
    if direction == "a" { position[0]-=1; }
    if direction == "s" { position[1]-=1; }
    if direction == "d" { position[0]+=1; }

    // for movement in direction.chars() {
    //     if movement == 'w' { position[1]+=1; }
    //     if movement == 'a' { position[0]-=1; }
    //     if movement == 's' { position[1]-=1; }
    //     if movement == 'd' { position[0]+=1; }
    // }
    
    let tempx = position[0];
    let tempy = position[1];
    position[0] = special::clamp(position[0], MIN_LEN, MAX_LEN);
    position[1] = special::clamp(position[1], MIN_WID, MAX_WID);
    position = [position[0], position[1]];

    if (tempx != position[0]) || (tempy != position[1]) { println!("You've bumped into a wall!"); }

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

fn animal_eat(animal_position: [i8; 2], player_position: [i8; 2]) -> bool {
    let eaten: bool;

    if animal_position == player_position { eaten = true; }
    else { eaten = false; }

    return eaten;
}

fn smell_animal(animal_position: [i8; 2], player_position: [i8; 2]) -> bool {
    let is_nearby: bool;
    let mut x = 0;
    let mut y = 0;

    for point in (player_position[0]-1)..(player_position[0]+2) {
        // println!("{}", point);
        if animal_position[0] == point {
            x = point;
        }
    }

    for point in (player_position[1]-1)..(player_position[1]+2) {
        // println!("{}", point);
        if animal_position[1] == point {
            y = point;
        }
    }

    if [x, y] == animal_position { is_nearby = true; }
    else { is_nearby = false }

    // println!("{}", is_nearby);

    return is_nearby;
}
