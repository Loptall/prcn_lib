
pub fn yes_no(b: bool) {
    if b {
        println!("Yes");
    } else {
        println!("No");
    }
}

pub fn chars_to_string(v: &[char]) -> String {
    let mut ret = String::with_capacity(v.len());
    for c in v.iter() {
        ret.push(*c);
    }
    ret
}


/// プログラムを終了
pub fn exit() {
    use std::process;
    process::exit(0)
}
