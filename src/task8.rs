#[test]

/*

fn main() {
    // Fix error by modifying this line
    let  s = String::from("hello, ");

    borrow_object(&mut s);

    println!("Success!");
}

fn borrow_object(s: &mut String) {}
*/


fn main() {
    // Fix error by modifying this line
    let  mut s = String::from("hello, ");

    borrow_object(&mut s);

    println!("Success!");
}

fn borrow_object(_s: &mut String) {}

/*
В коде ошибка возникает из-за того, что попытка передать изменяемую ссылку на переменную,
которая не является изменяемой. Чтобы исправить это, нужно сделать переменную s изменяемой,
добавив ключевое слово mut
*/