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

// fn main() {
//     let mut i = 0;
//     loop {
//         i += 1;
//         if i % 2 == 1 {continue}

//         println!("{i}");

//         if i > 100 {
//             break;
//         }
//     }
// }

// fn main() {
//     let z = 13;
//     let x = {
//         let y = 10;
//         println!("y: {y}");
//         z - y
//     };
//     println!("x: {x}");
// }

// fn gcd(a: u32, b: u32) -> u32 {
//    if b > 0 {
//        gcd(b, a % b)
//    } else {
//        a
//    }
// }

// fn main() {
//    println!("gcd: {}", gcd(143, 52));
// }

/// Determina la longitud de la secuencia de Collatz que empieza por `n`.
fn collatz_length(n: i32, paso: i16) -> u32 {
    println!("Paso n{}",paso);
  if n == 1 {return 1;}
  if n%2 == 0 {return collatz_length(n/2, paso+1)}
return  collatz_length(3*n+1, paso+1);

}

fn main() {
    let num = 3;

    collatz_length(num, 1);
}