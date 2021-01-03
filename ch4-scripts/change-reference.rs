// changing a reference demonstration pg. 72
fn change(some_string: &mut String) {
    // can make changes to string since it's mutable
    some_string.push_str(", world!");
}

fn main() {
    // String must be mutable for push_str() to work in change()
    let mut s = String::from("hello");

    // call change() with &mutable string object
    change(&mut s);

    println!("{}", s);
}
