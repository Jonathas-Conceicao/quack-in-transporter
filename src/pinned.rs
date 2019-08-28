use std::pin::Pin;

trait Quack {
    fn quack(self: Pin<&Self>);
}

struct Duck;

impl Quack for Duck {
    fn quack(self: Pin<&Self>) {
        println!("Quack!")
    }
}

struct Transporter<A>(A);

impl<Animal: Quack> Quack for Transporter<Animal> {
    fn quack(self: Pin<&Self>) {
        self.0.quack()
    }
}

fn main() {
    let duck_in_transporter = Transporter(Duck);
    duck_in_transporter.quack();
}
