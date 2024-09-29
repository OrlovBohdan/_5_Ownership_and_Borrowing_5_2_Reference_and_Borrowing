#[test]

/*

// Remove something to make it work
// Don't remove a whole line !
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);

    println!("Success!");
}
*/



// Remove something to make it work
// Don't remove a whole line !
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &r1;

    println!("{}, {}", r1, r2);

    println!("Success!");
}

/*
В коде ошибка возникает из-за того, что Rust не позволяет иметь несколько изменяемых ссылок
на одну и ту же переменную одновременно.
Чтобы исправить это, нужно удалить одну из ссылок на s:
*/