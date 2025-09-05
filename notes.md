

In rust there are these enum defined in standard library that are very useful>

The `Result` enum to handle success failure

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}


fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0. {
        Err(format!("Divide by zero"))
    } else {
        Ok(numerator / denominator)
    }
}

fn show_divide(num: f64, den: f64) {
    match divide(num, den) {
        Ok(val) => println!("{} / {} = {}", num, den, val),
        Err(msg) => println!("Cannot divide {} by {}: {}", num, den, msg),
    }
}
```
    
    
The `Option` enum to handle the presence or not aof a value

```rust
enum Option<T> {
    Some(T),
    None,
}

let mut v = vec![11, 22, 33];
for _ in 0..5 {
    let item: Option<i32> = v.pop();
    match item {
        Some(number) => print!("{}, ", number),
        None => print!("#, "),
    }
}    
```

## Allocating memory

### static allocation

```rust

static _A: u32 = 3;
static _B: i32 = -1_000_000;

```
The static keyword is similar to the let keyword.
static requires the explicit specification of the type of the variable,
which is optional using let.
Normal code cannot change the value of a static variable, even if it
has the mut specification.

The style guidelines require that the names of static variables contain
only uppercase letters, with words separated by underscore.


### stack allocation

Rust allocates an object in the “stack”, every time a variable is declared using the let keyword, and every time an argument is passed to a function invocation

```rust

let _a: u32 = 3;
let _b: i32 = -1_000_000;

```

The stack has usually a quite limited size

Rust allows us to allocate in the stack only objects whose size is known at compilation type like primitive types and arrays, and does not allow us to allocate in the stack objects whose size is determined only at runtime, like vectors.


### heap allocation

```rust

let _a: Box<u32> = Box<u32>::new(12);

```

## The Sizes of the Primitive Types

```rust

use std::mem::*;
print!("{} {} {} {} {} {} {} {} {} {} {} {}",
    size_of::<i8>(),
    size_of::<u8>(),
    size_of::<i16>(),
    size_of::<u16>(),
    size_of::<i32>(),
    size_of::<u32>(),
    size_of::<i64>(),
    size_of::<u64>(),
    size_of::<f32>(),
    size_of::<f64>(),
    size_of::<bool>(),
    size_of::<char>());

// In any computer, this will print
1 1 2 2 4 4 8 8 4 8 1 4

```


## The Representation of Primitive Types

Rust discourages accessing the internal representation of objects, and so it is not easy to do; but there is a trick to do that.

```rust

fn as_bytes<T>(o: &T) -> &[u8] {
    unsafe {
        std::slice::from_raw_parts(
            o as *const _ as *const u8,
            std::mem::size_of::<T>())
    }
}

println!("{:?}", as_bytes(&1i8));
println!("{:?}", as_bytes(&2i16));
println!("{:?}", as_bytes(&3i32));
println!("{:?}", as_bytes(&'A'));
println!("{:?}", as_bytes(&true));

// In an x86_64 system, this could print:
[1]
[2, 0]
[3, 0, 0, 0]
[65, 0, 0, 0]
[1]

```

## Location of Bytes in Memory

You can also discover the (virtual) memory location of any object, which is its address:

```rust

let b1 = true;
let b2 = true;
let b3 = false;
print!("{} {} {}",
    &b1 as *const bool as usize,
    &b2 as *const bool as usize,
    &b3 as *const bool as usize);

// In a 64-bit system, this will print three huge numbers, resembling 

140727116566237 140727116566238 140727116566239
```
Each one of the three objects occupies just one byte. The first printed number is the address of the b1 variable; the second the address of the b2 variable; and the third the address of the b3 variable. As it appears, the three numbers are consecutive, and that means that the three objects are allocated in contiguous virtual memory locations.


## Defining Closures

```rust

let mut arr = [4, 8, 1, 10, 0, 45, 12, 7];
use std::cmp::Ordering;
fn desc(a: &i32, b: &i32) -> Ordering {
    if a < b { Ordering::Greater }
    else if a > b { Ordering::Less }
    else { Ordering::Equal }
}

// HOF
arr.sort_by(desc);

print!("{:?}", arr);

```

using closures

A closure is just a handier kind of function, fit to define small anonymous functions, and to invoke them just where they have been defined.

```rust

let mut arr = [4, 8, 1, 10, 0, 45, 12, 7];
use std::cmp::Ordering;
let desc = |a: &i32, b: &i32| -> Ordering {
    if a < b { Ordering::Greater }
    else if a > b { Ordering::Less }
    else { Ordering::Equal }
};

arr.sort_by(desc);

print!("{:?}", arr);


```

```rust

let mut arr = [4, 8, 1, 10, 0, 45, 12, 7];
use std::cmp::Ordering;
arr.sort_by(|a, b|
    if a < b { Ordering::Greater }
    else if a > b { Ordering::Less }
    else { Ordering::Equal });

print!("{:?}", arr);

```

if we wanna refer to a function we can use the Fn keyword

```rust

let factor = 2;
let multiply = |a| a * factor;
print!("{}", multiply(13));
let multiply_ref: &(Fn(i32) -> i32) = &multiply;
print!(
    " {} {} {} {} {}",
    (*multiply_ref)(13),
    multiply_ref(13),
    (|a| a * factor)(13),
    (|a: i32| a * factor)(13),
    |a| -> i32 { a * factor }(13));

// this will print
26 26 26 26 26 26
```

## Changeable strings

### static strings

```rust

use std::mem::*;
let a: &str = "";
let b: &str = "0123456789";
let c: &str = "abcdè";
print!("{} {} {}",
    size_of_val(a),
    size_of_val(b),
    size_of_val(c));

// will print
0 10 6

```

The str word is defined in the standard library as the type of an unmodifiable array of bytes representing a UTF-8 string

### dynamic strings

if we want to create or change the contents of a string at runtime, the &str type, which we always used so far, is unfit

But Rust provides also another kind of strings, the dynamic strings, whose content can be changed:

```rust

let mut a: String = "He".to_string();
a.push('l');
a.push('l');
a.push('o');
print!("{}", a);

// will print
Hello

```

There are several ways to create an empty dynamic string.

```rust

let s1 = String::new();
let s2 = String::from("");
let s3 = "".to_string();
let s4 = "".to_owned();
let s5 = format!("");
print!("({}{}{}{}{})", s1, s2, s3, s4, s5);

```

concatenating strings:

```rust

let mut dyn_str = "Hello".to_string();
dyn_str = format!("{}{}", dyn_str, ", ");
dyn_str = format!("{}{}", dyn_str, "world");
dyn_str = format!("{}{}", dyn_str, "!");
print!("{}", dyn_str);

```

```rust

let mut dyn_str = "Hello".to_string();
dyn_str.push_str(", ");
dyn_str.push_str("world");
dyn_str.push_str("!");
print!("{}", dyn_str);

```

```rust

let mut dyn_str = "Hello".to_string();
dyn_str += ", ";
dyn_str += "world";
dyn_str += "!";
print!("{}", dyn_str);

```
