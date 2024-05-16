// Define the Logger traits
trait Logger {
    fn log(&self, msg: &str);
}

// Implement FileLogger struct and Logger trait
struct FileLogger;

impl Logger for FileLogger {
    fn log(&self, msg: &str) {
        println!("> Log to file: {}", msg);
    }
}

// Implement DatabaseLogger struct and Logger trait
struct DatabaseLogger;

impl Logger for DatabaseLogger {
    fn log(&self, msg: &str) {
        println!("> Log to database: {}", msg);
    }
}

pub fn example() {
    println!("Open/Closed Principle");

    let f_log = FileLogger;
    let d_log = DatabaseLogger;

    f_log.log("Devไปวันๆ");
    d_log.log("Devไปวันๆ");
}