#[test]


/*// Fix error
fn main() {
    let mut s = String::from("hello, ");

    borrow_object(s);

    println!("Success!");
}

fn borrow_object(s: &String) {}*/


// Fix error
fn main() {
    let mut _s = String::from("hello, ");

    borrow_object(&_s);

    println!("Success!");
}

fn borrow_object(_s: &String) {

}