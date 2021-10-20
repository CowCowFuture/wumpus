pub fn clamp<T: PartialOrd<T>>(mut num: T, min: T, max: T) -> T {
    if num <= min { num = min; }
    if num >= max { num = max; }

    return num;
}
