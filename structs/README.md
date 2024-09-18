## Structs

Structs allow us to store multiple related values under one name.

     struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

Our struct will be a new type which we can then use to instantiate new variables with their data in the respective field name.

    fn main() {
        let mut user1: User = User {
            email: String::from("johndoe@gmail.com"),
            username: String::from("John Doe"),
            active: true,
            sign_in_count: 1,
        };
    }


We can then use '.' to access the fields in our struct instance. This can also be used to updating that field.

    let name: String = user1.username;
    user1.username = String::from("wallace123");

We can also use a function create an instance of our struct.

    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }

    fn main() {
        let user2: User = build_user(
            String::from("jasonborne@gmail.com"),
            String::from("Jason Borne"),
        );
    }

struct update, 
we can use a struct to instantiate another struct.

    let user3: User = User {
        email: String::from("james@gmail.com"),
        username: String::from("james123"),
        ..user2
    };
