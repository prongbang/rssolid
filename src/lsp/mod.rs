// Define the Flyer and Walker traits
trait Flyer {
    fn fly(&self);
}

trait Walker {
    fn walk(&self);
}

// Define the Bird struct and implement Flyer and Walker traits
struct Bird;

impl Flyer for Bird {
    fn fly(&self) {
        println!("> Flying bird is flying");
    }
}

impl Walker for Bird {
    fn walk(&self) {
        println!("> Working bird is walking");
    }
}

// Define the Duck struct and implement Flyer and Walker traits
struct Duck {
    bird: Bird,
}

impl Flyer for Duck {
    fn fly(&self) {
        self.bird.fly();
    }
}

impl Walker for Duck {
    fn walk(&self) {
        println!("> Working duck is walking");
    }
}

// Define the Penguin struct and implement Walker trait
struct Penguin;

impl Walker for Penguin {
    fn walk(&self) {
        println!("> Working penguin is walking");
    }
}

pub fn example() {
    println!("Liskov Substitution Principle");

    let bird = Bird;
    bird.fly();
    bird.walk();

    let duck = Duck { bird: Bird };
    duck.fly();
    duck.walk();

    let penguin = Penguin;
    penguin.walk();
}
