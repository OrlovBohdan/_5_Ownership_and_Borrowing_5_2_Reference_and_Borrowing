#[test]

/*

fn main() {
    let c = '中';

    let r1 = &c;
    // Fill the blank，dont change other code
    let __ r2 = c;

    assert_eq!(*r1, *r2);

    // Check the equality of the two address strings
    assert_eq!(get_addr(r1),get_addr(r2));

    println!("Success!");
}

// Get memory address string
fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}
*/


fn main() {
    let c = '中';

    let r1 = &c;
    // Fill the blank，dont change other code
    let  r2 = &c;

    assert_eq!(*r1, *r2);

    // Check the equality of the two address strings
    assert_eq!(get_addr(r1),get_addr(r2));

    println!("Success!");
}

// Get memory address string
fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}

/*
В коде нужно заполнить пропуск так, чтобы r2 был ссылкой на char.
Поскольку get_addr принимает ссылку на char, нужно создать ссылку на c,
как и в случае с r1.
*/