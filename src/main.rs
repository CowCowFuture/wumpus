fn main() {
    let mut game_loop = true;

    while game_loop {
        let mut input = String::new();
        input = spec_read(input);

        if input == "q" {
            game_loop = false;
        }

    }
}

fn spec_read(mut string: String) -> String {
    std::io::stdin().read_line(&mut string).expect("borked");
    string.pop();
    string = string.to_lowercase();

    return string;
}
