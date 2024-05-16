// Define the Employee struct
#[derive(Debug)]
struct Employee {
    name: String,
    email: String,
}

// Traits for updating the name and email
trait NameUpdater {
    fn update_name(&self, employee: &mut Employee, name: &str);
}

trait EmailUpdater {
    fn update_email(&self, employee: &mut Employee, email: &str);
}

// Concrete types implementing the traits
struct NameUpdaterImpl;
struct EmailUpdaterImpl;

impl NameUpdater for NameUpdaterImpl {
    fn update_name(&self, employee: &mut Employee, name: &str) {
        employee.name = name.to_string();
    }
}

impl EmailUpdater for EmailUpdaterImpl {
    fn update_email(&self, employee: &mut Employee, email: &str) {
        employee.email = email.to_string();
    }
}

pub fn example() {
    println!("Single Responsibility Principle:");

    let mut emp = Employee {
        name: String::new(),
        email: String::new(),
    };

    let email_updater = EmailUpdaterImpl {};
    let name_updater = NameUpdaterImpl {};

    name_updater.update_name(&mut emp, "Devไปวันๆ");
    email_updater.update_email(&mut emp, "name@email.dev");

    println!(" > {:?}", &emp);
}