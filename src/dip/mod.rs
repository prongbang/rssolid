// Define the User struct
struct User {
    id: i32,
}

// Define the UserRepository trait
trait UserRepository {
    fn get_user(&self, id: i32) -> Result<User, Box<dyn std::error::Error>>;
}

// Implement the DatabaseUserRepository struct and UserRepository trait
struct DatabaseUserRepository;

impl UserRepository for DatabaseUserRepository {
    fn get_user(&self, id: i32) -> Result<User, Box<dyn std::error::Error>> {
        if id == 1 {
            Ok(User { id: 1 })
        } else {
            Ok(User { id: 0 })
        }
    }
}

// Define the UserService struct and its methods
struct UserService<T: UserRepository> {
    repo: T,
}

impl<T: UserRepository> UserService<T> {
    fn register_user(&self, user: User) -> Result<(), Box<dyn std::error::Error>> {
        let u = self.repo.get_user(user.id)?;
        if u.id == 0 {
            println!("> Registered user id: {}", user.id);
            Ok(())
        } else {
            Err("User already exists".into())
        }
    }
}

pub fn example() {
    println!("Dependency Inversion Principle");

    let usr_repo = DatabaseUserRepository;
    let usr_svc = UserService { repo: usr_repo };
    let usr = User { id: 1 };

    let _ = usr_svc.register_user(usr);
}
