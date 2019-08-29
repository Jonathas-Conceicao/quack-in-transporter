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

impl<A> Transporter<A> {
    fn animal(self: Pin<&Self>) -> Pin<&A> {
        unsafe { Pin::map_unchecked(self, |t| &t.0) }
    }
}

impl<Animal: Quack> Quack for Transporter<Animal> {
    fn quack(self: Pin<&Self>) {
        self.animal().quack()
    }
}

fn main() {
    let duck_in_transporter = unsafe { Pin::new_unchecked(&Transporter(Duck)) };
    duck_in_transporter.quack();
}
