// for-loop example pgs. 56-57
fn main() {
    let a = [10, 20, 30, 40, 50];

    // no parentheses in for-loop arguments.
    // 
    for element in a.iter() {
        println!("The value is: {}", element);
    }
}
