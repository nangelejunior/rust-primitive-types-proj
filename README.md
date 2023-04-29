# [Tipos Primitivos em Rustlang | Aprenda Rust | 03](https://youtu.be/IEFrj4znVIU)

## Escalares (scalar type)

- Representam um único valor contido dentro de uma escala conhecida.
- Permitem a comparação direta entre valores.

### Tipos

- inteiro (integer) ex: `5`
- flutuante (floating point) ex: `42.1`
- booleano (bool) ex: `true`, `false`
- caractere (char) ex: `'a'`, `'ॐ'`, `'🙂️'`  

## Compostos (compound types)

- Servem para agregar multiplos valores.

### Tipos

- Tupla (tuple) ex: `(5, true, 42.1, 'a')`
- Matriz (array) ex: `[1, 2, 3, 4, 5, 6]`

## Números inteiros

| bits | signed | unsigned |
|------|--------|----------|
| 8    | i8     | u8       |
| 16   | i16    | u16      |
| 32   | i32    | u32      |
| 64   | i64    | u64      |
| 128  | i128   | u128     |
| arch | isize  | usize    |

### Signed

range: -(2n- 1) até 2n- 1 - 1
i8: -128 até 127 [-($2^7$) até $2^7$ - 1]

### Unsigned

range: 0 até $2^n$ - 1
u8: 0 até 255 [0 até $2^8 - 1$]

```rust
fn main() {
	let x = 5;
	let y: i32 = 5;
	let z = 5_i32;
}
```

## Overflow

```rust
fn main() {
	let x: i8 = 5;
	let y: u8 = x - 20; //error
}
```

```rust
fn main() {
	let x: i8 = 5;
	let y: i8 = x * 200; //error
}
```

## Literais

```rust
fn main() {
	let x = 5;
	let y = 199_456_898_9;
	let h = 0xff;
	let o = 0o77;
	let b = 0b1111_0000;
	let by = b'A';
}
```

## Números float

```rust
fn main() {
	let x = 42.1;
	let x: f64 = 42.1; // f64 padrão -> precisão dupla
	let x: f32 = 42.1;
}
```

## Bool

```rust
fn main() {
	let x = true;
	let x: bool = false;
}
```

## Char e unicode

- Armazena até 4 bits da tabela unicode.

```rust
fn main() {
	let letra = 'a';
	let letra: char = 'a';
	let letra = '😁️';
}
```

## Tupla

```rust
fn main() {
	//tupla
	let numbers = (1, 2, 3);
	println!("{:?}", numbers);
	
	let numbers: (i32, i32, i32) = (1, 2, 3);
	println!("{:?}", numbers);
	
	let numbers: (i32, i32, f64) = (1, 2, 3.5);
	println!("{:?}", numbers);
}
```

```rust
fn main() {
	//tupla
	let numbers = (1, 2, 3);	
	println!("{:?}", numbers.1);
}
```

## Pattern match em tuplas

```rust
fn main() {
	//tupla
	let numbers = (1, 2, 3);
	println!("{:?}", a);
	
	let (a, b, c) = numbers;
	println!("{:?}", a);
}
```

## Mutabilidade de tupla

```rust
fn main() {
	//tupla
	let mut numbers = (1, 2, 3);
	println!("{:?}", numbers);
	
	numbers.0 = 50;
	println!("{:?}", numbers);
}
```

```rust
fn main() {
	//tupla
	let mut numbers = (1, 2, 3);
	println!("{:?}", numbers);
	
	numbers = (4, 5, 6); //pattern match
	println!("{:?}", numbers);
}
```

## Array

```rust
fn main() {
	let numbers = [1, 2, 3];
	println!("{:?}", numbers);
	
	let numbers: [i32;3] = [1, 2, 3];
	println!("{:?}", numbers);
}
```

```rust
fn main() {
	let numbers = [1.1, 2.0, 3.3];
	println!("{:?}", numbers[0]);
}
```

## Mutabilidade em array

```rust
fn main() {
	let mut numbers = [1.1, 2.0, 3.3];
	println!("{:?}", numbers[0]);
	
	numbers[0] = 50.3;
	println!("{:?}", numbers[0]);
}
```

```rust
fn main() {
	let mut numbers = [1.1, 2.0, 3.3];
	println!("{:?}", numbers);
	
	numbers = [4.4, 5.5, 6.6]; 
	println!("{:?}", numbers);
}
```

## Out of Bound Error

```rust
fn main() {
	let mut numbers = [1.1, 2.0, 3.3];
	println!("{:?}", numbers[10]); //error
}
```

## Slicing

```rust
fn main() {
	let mut numbers = [1.1, 2.0, 3.3];
	println!("{:?}", numbers);
	
	//slice
	println!("{:?}", &numbers[1..3]);
	println!("{:?}", &numbers[1..]);
	println!("{:?}", &numbers[..2]);
}
```
