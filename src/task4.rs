#[test]

/*

// Fix error
fn main() {
    let mut s = String::from("hello, ");

    push_str(s);

    println!("Success!");
}

fn push_str(s: &mut String) {
    s.push_str("world")
}
*/


// Fix error
fn main() {
    let mut s = String::from("hello, ");

    push_str(&mut s);

    println!("Success!");
}

fn push_str(s: &mut String) {
    s.push_str("world")
}
/*

В коде есть проблема с передачей владения переменной s в функцию push_str.
Чтобы исправить это, нужно передать изменяемую ссылку на строку вместо самой строки.
*/