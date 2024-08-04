// fn main() {
//     let x = 10;
//     if x < 20 {
//         println!("pequeño");
//     } else if x < 100 {
//         println!("muy grande");
//     } else {
//         println!("enorme");
//     }
// }

// fn main() {
//     let x = 10;
//     let size = if x < 20 { "pequeño" } else if x < 60 {"mediano"} else { "grande" };
//     println!("tamaño del número: {}", size);
// }

// fn main() {
//     let mut x = 200
//     ;
//     while x >= 10 {
//         x = x / 2;
//     }
//     println!("x final: {x}");
// }

// fn main() {
//     for x in 0..5 {
//         println!("x: {x}");
//     }
// }

fn main() {
    let mut i = 0;
    loop {
        i += 1;
        if i % 2 == 1 {continue}

        println!("{i}");

        if i > 100 {
            break;
        }
    }
}