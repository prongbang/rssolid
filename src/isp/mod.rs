// Define the Worker, Sleeper, and Eater traits
trait Worker {
    fn work(&self);
}

trait Sleeper {
    fn sleep(&self);
}

trait Eater {
    fn eat(&self);
}

// Define the HumanWorker struct and implement Worker, Sleeper, and Eater traits
struct HumanWorker;

impl Worker for HumanWorker {
    fn work(&self) {
        println!("> Humen wark");
    }
}

impl Sleeper for HumanWorker {
    fn sleep(&self) {
        println!("> Humen sleep");
    }
}

impl Eater for HumanWorker {
    fn eat(&self) {
        println!("> Humen eat");
    }
}

// Define the RobotWorker struct and implement Worker trait
struct RobotWorker;

impl Worker for RobotWorker {
    fn work(&self) {
        println!("> Robot wark");
    }
}

pub fn example() {
    println!("Interface Segregation Principle");

    let humen = HumanWorker;
    humen.work();
    humen.eat();
    humen.sleep();

    let robot = RobotWorker;
    robot.work();
}