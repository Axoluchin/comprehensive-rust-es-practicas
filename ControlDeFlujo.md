# Condicionales

Gran parte de la sintaxis de Rust te resultará familiar de C, C++ o Java:

- Los bloques están delimitados por llaves.
- Los comentarios de línea empiezan por `//`, mientras que los comentarios de bloque están delimitados por `/* ... */`.
- Palabras clave como `if` y `while` funcionan igual.
- La asignación de variables se realiza con `=` y la comparación con `==`.

## If

```rust
fn main() {
    let x = 10;
    if x < 20 {
        println!("pequeño");
    } else if x < 100 {
        println!("muy grande");
    } else {
        println!("enorme");
    }
}
```

> No se usa paréntesis, que raro

También puedes hacer operaciones ternarias con `if`

```rust
fn main() {
    let x = 10;
    let size = if x < 20 { "pequeño" } else { "grande" };
    println!("tamaño del número: {}", size);
}
```

## Bucles

### While

```rust
fn main() {
    let mut x = 200;
    while x >= 10 {
        x = x / 2;
    }
    println!("x final: {x}");
}
```

### For

```rust
fn main() {
    for x in 1..5 {
        println!("x: {x}");
    }
}
```

> Para rangos :D

### Loop (el do while para los C amigos)

```rust
fn main() {
    let mut i = 0;
    loop {
        i += 1;
        println!("{i}");
        if i > 100 {
            break;
        }
    }
}
```

> Siempre el bucle hasta llegar a un `break`

## break y continue

- `break` rompe el bucle, se sale, bye
- `continue` interrumpe la iteración, no el bucle
