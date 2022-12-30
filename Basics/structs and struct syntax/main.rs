
fn main() {
    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );


    let user2 = User {
    active: user1.active,
    username: user1.username,
    email: String::from("another@example.com"),
    sign_in_count: user1.sign_in_count,};

    let _user3 = User{
        email: String::from("example@email.com"),
        ..user2
    };

    let _black = Color(0, 0, 0);

    let _origin = Point(0, 0, 0);


    let _subject = AlwaysEqual;
    
}



fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}


struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);

struct Point(i32, i32, i32);

struct AlwaysEqual;

