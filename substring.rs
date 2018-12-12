fn main() {
    let name = String::from("Daniel");
    let a = substring(name, 0, 4);
    println!("{}", a);
}

fn substring(_string: String, _start: usize, _end: usize) -> String {
    let mut _subs = String::new();
    
    for (i, c) in _string.chars().enumerate() {
        if i < _start {
            continue;
        }
        if i == _end {
            break;
        }
        _subs.push(c);
    }
    return _subs;
}
