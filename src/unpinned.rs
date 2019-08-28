trait Quack {
    fn quack(&self);
}

struct Duck;

impl Quack for Duck {
    fn quack(&self) {
        println!("Quack!")
    }
}

struct Transporter<A>(A);

impl<Animal: Quack> Quack for Transporter<Animal> {
    fn quack(&self) {
        self.0.quack()
    }
}

fn main() {
    let duck_in_transporter = Transporter(Duck);
    duck_in_transporter.quack();
}
