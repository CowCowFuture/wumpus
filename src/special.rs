pub fn clamp<T: PartialOrd<T>>(mut num: T, min: T, max: T) -> T {
    if num <= min { num = min; }
    if num >= max { num = max; }

    return num;
}

pub fn read(mut string: String) -> String {
    std::io::stdin().read_line(&mut string).expect("borked");
    string.pop();
    string = string.to_lowercase();

    return string;
}
