use num::clamp;

fn main() {
    let mut game_loop = true;
    let mut posx: i8 = 0;
    let mut posy: i8 = 0;

    while game_loop {
        let mut input = String::new();
        input = spec_read(input);

        if input == "q" {
            game_loop = false;
        }
        if input == "w" { posy+=1; }
        if input == "a" { posx-=1; }
        if input == "s" { posy-=1; }
        if input == "d" { posx+=1; }
        let tempx = posx;
        let tempy = posy;
        posx = clamp(posx, -8, 8);
        posy = clamp(posy, -8, 8);
        if tempx != posx { println!("You've bumped into a wall!"); }
        if tempy != posy { println!("You've bumped into a wall!"); }
        println!("{}, {}", posx, posy);

    }
}

fn spec_read(mut string: String) -> String {
    std::io::stdin().read_line(&mut string).expect("borked");
    string.pop();
    string = string.to_lowercase();

    return string;
}
