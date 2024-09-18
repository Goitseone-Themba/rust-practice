struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1: User = User {
        email: String::from("johndoe@gmail.com"),
        username: String::from("John Doe"),
        active: true,
        sign_in_count: 1,
    };

    let name: String = user1.username;
    user1.username = String::from("wallace123");

    let user2: User = build_user(
        String::from("jasonborne@gmail.com"),
        String::from("Jason Borne"),
    );

    let user3: User = User {
        email: String::from("james@gmail.com"),
        username: String::from("james123"),
        ..user2
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
