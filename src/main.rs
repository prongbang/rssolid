mod dip;
mod isp;
mod lsp;
mod ocp;
mod srp;

fn main() {
    // 1. Single Responsibility Principle
    srp::example();

    // 2. Open/Closed Principle
    ocp::example();

    // 3. Liskov Substitution Principle
    lsp::example();

    // 4. Interface Segregation Principle
    isp::example();

    // 5. Dependency Inversion Principle
    dip::example();
}
