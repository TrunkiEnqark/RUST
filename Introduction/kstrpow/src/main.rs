use std::io;

fn main() {
    let mut lines = io::stdin().lines();
    let _string_1 = lines.next().unwrap().unwrap();
    let _string_2 = lines.next().unwrap().unwrap();
    let _times = lines.next().unwrap().unwrap();
    let mut _times: usize = _times.parse().unwrap();
    let mut temp_string: String = String::new();
    while _times > 0 {
        temp_string += &_string_1;
        _times -= 1;
    }
    if _string_2 == temp_string {
        print!("YES");
    } else {
        print!("NO");
    }
}
