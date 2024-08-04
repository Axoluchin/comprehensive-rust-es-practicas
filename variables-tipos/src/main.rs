// fn main() {
//     let mut x: i32 = 10;
//     println!("x: {x}");
//      x = 20;
//     println!("x: {x}");
// }

// fn interproduct(a: i32, b: i32, c: i32) -> i32 {
//     return a * b + b * c + c * a;
// }

// fn main() {
//     println!("resultado: {}", interproduct(120, 100, 248));
// }

fn main() {
    let greeting: &str = "Saludos";
    let planet: &str = "🪐";
    let mut sentence = String::new();
    sentence.push_str(greeting);
    sentence.push_str(", ");
    sentence.push_str(planet);
    println!("frase final: {}", sentence);
    println!("{:?}", &sentence[0..5]);
    // error -> println!("{:?}", &sentence[12..13]);
}