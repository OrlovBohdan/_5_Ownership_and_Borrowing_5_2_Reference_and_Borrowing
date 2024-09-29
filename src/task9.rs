#[test]

/*

// This code has no errors!
fn main() {
    let mut s = String::from("hello, ");

    borrow_object(&s);

    s.push_str("world");

    println!("Success!");
}

fn borrow_object(s: &String) {}
*/


fn main() {
    let mut s = String::from("hello, ");

    borrow_object(&s); // неизменяемая ссылка на изменяемый объект

    s.push_str("world"); // изменяем строку, так как владеем s

    println!("Success! {}", s); // выводим результат
}

fn borrow_object(s: &String) {} // функция получает неизменяемую ссылку на строку
