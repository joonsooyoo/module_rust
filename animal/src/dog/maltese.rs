use crate::color::Color;

pub struct Maltese {
    pub age: i32,
    pub color: Color::White;
    // feet: i32,
}

impl Maltese {
    pub fn cry(&self) {
        println!("Bow Wow!");
    }    
}


// impl Maltese {
//     pub fn new(age: i32, color: i32) -> Maltese {
//         Maltese {
//             age,
//             color,
//             feet = 4;
//         }
//     }
// }
