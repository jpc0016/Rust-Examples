// destructuring pg. 41

fn main() {
    // initialize tuple with defined types
    let x (i32, f64, u8) = (500, 6.4, 1);

    // use indexing to assign elements from tuple x to variables
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
}
