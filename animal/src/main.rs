use animal;
use animal::color::Color;

fn main() {
    animal::cat::cat_cry();
    let charlie = animal::dog::maltese::Maltese {
        age: 1,
        color: Color::White,
    };
    charlie.cry();

}
