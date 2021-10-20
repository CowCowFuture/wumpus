mod clamp;

fn main() {
    let mut game_loop = true;
    let mut player_pos = [0, 0];

    while game_loop {
        let mut input = String::new();
        input = spec_read(input);

        if input == "q" {
            game_loop = false;
        }

        player_pos = player_move(player_pos, input);
        
        println!("{}, {}", player_pos[0], player_pos[1]);

    }
}

fn spec_read(mut string: String) -> String {
    std::io::stdin().read_line(&mut string).expect("borked");
    string.pop();
    string = string.to_lowercase();

    return string;
}

fn player_move(mut position: [i8; 2], direction: String) -> [i8; 2] {
    if direction == "w" { position[1]+=1; }
    if direction == "a" { position[0]-=1; }
    if direction == "s" { position[1]-=1; }
    if direction == "d" { position[0]+=1; }
    let tempx = position[0];
    let tempy = position[1];
    position[0] = clamp::clamp(position[0], -8, 8);
    position[1] = clamp::clamp(position[1], -8, 8);
    position = [position[0], position[1]];
    if tempx != position[0] || tempy != position[1] { println!("You've bumped into a wall!"); }

    return position;
}
