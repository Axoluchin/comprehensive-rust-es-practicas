# Variables y tipos

Definir una variable

```rs
let nombre_variable: i32 = 10
```

Donde

- `i32` es el tipo de dato
- `let` es la definición de variable NO MUTABLE

Para mutar una variable agregamos `mut` al inicio de la declaración de la variable

```rs
let mut variable_mutable: i32 = 10
```

## Tipos de datos

|                           | Tipos                                      | Literales                      |
| ------------------------- | ------------------------------------------ | ------------------------------ |
| Enteros con signo         | `i8`, `i16`, `i32`, `i64`, `i128`, `isize` | `-10`, `0`, `1_000`, `123_i64` |
| Enteros sin signo         | `u8`, `u16`, `u32`, `u64`, `u128`, `usize` | `0`, `123`, `10_u16`           |
| Números de coma flotante  | `f32`, `f64`                               | `3.14`, `-10.0e20`, `2_f32`    |
| Valores escalares Unicode | `char`                                     | `'a'`, `'α'`, `'∞'`            |
| Booleanos                 | `bool`                                     | `true`, `false`                |

Los tipos tienen la siguiente anchura:

- `iN`, `uN`, and `fN` son _N_ bits de capacidad,
- `isize` y `usize` tienen el ancho de un puntero,
- `char` tiene un tamaño de 32 bits,
- `bool` tiene 8 bits de ancho.

> Todos guiones bajos en los números pueden no utilizarse, ya que solo sirven para facilitar la lectura. Por lo tanto, 1_000 se puede escribir como 1000 (o 10_00), y 123_i64 se puede escribir como 123i64.

## Strings

- `String`: a modifiable, owned string.
- `&str`: es una cadena de solo lectura. Los literales de cadena son de este tipo.
