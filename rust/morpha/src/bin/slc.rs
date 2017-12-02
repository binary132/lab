extern crate morpha;

use morpha::Morpha;

fn main() {
    let r = std::io::stdin();
    {
        let r = r.lock();
        println!("{}", Morpha::new().lex(r).next() == None);
    }
}
