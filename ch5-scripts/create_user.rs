// intro to structs
// build_user struct example pg. 85

struct User {
    email: String,
    username: String,
    active: bool,
    signin_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        // shorthand notation used for email and username to avoid typing variables over and over
        email,
        username,
        active: true,
        signin_count: 1,
    }
}

fn main() {
    let email = String::from("someone@example.com");
    let username = String::from("libcuck69");
    let user1 = build_user(email, username);

    // create user2 with specific email and username. Use struct update syntax to derive unchanged
    // User properties from user1
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("assmaster69"),
        ..user1
    };
}
