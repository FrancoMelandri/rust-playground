

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

## Iterators

string iterator

```rust

fn print_nth_char(s: &str) {
    let mut iter: std::str::Chars = s.chars();
    loop {
        match  iter.next() {
            Some(c) => { print!("{} {}", c, s as u32); },
            None => { break; },
        }
    }
}
print_nth_char("€èe");

```

we can also write in this way, witha  drastic syntax semplification

```rust

fn print_nth_char(s: &str) {
    for c in s.chars() {
        print!("{} {}", c, s as u32);
    }
}
print_nth_char("€èe");

```

you can also iterate through a slice using the `iter()` to get the iterator

```rust

for item_ref in (&[11u8, 22, 33]).iter() {
    // *item_ref += 1;
    print!("{} ", *item_ref);
}

for item_ref in [44, 55, 66].iter() {
    // *item_ref += 1;
    print!("{} ", *item_ref);
}

for item_ref in vec!['a', 'b', 'c'].iter() {
    // *item_ref = if *item_ref == 'b' { 'B' } else { '-' };
    print!("{} ", *item_ref);
}

```

if we need to mutate the item of an interator we should ues `iter_mut()`

```rust

let slice = &mut [3, 4, 5];
{
    let iterator = slice.iter_mut();
    for item_ref in iterator {
        *item_ref += 1;
    }
}
print!("{:?}", slice);

// This will print: "[4, 5, 6]".

```

### filter()

we can filter items in the iteator using a callback function injected in the `filter()` function of athe standard library.
Theclosure function is a predicate, meaning it must return a boolean value.

```rust

let arr = [66, -8, 43, 19, 0, -31];
for n in arr.iter().filter(|x| **x < 0) {
    print!("{} ", n);
}
// This will print: "-8 -31".

```

### map()

The map function is another iterator adapter in the standard library. Its purpose
is to “transform” the values produced by an iterator into other values. Differing from
the filter function, the value returned by the closure can be of any type. Such value
represents the transformed value.

```rust

let arr = [66, -8, 43, 19, 0, -31];
for n in arr.iter().map(|x| *x * 2) {
    print!("{} ", n);
}
// This will print: "132 -16 86 38 0 -62".

```

### enumerate()

In the second line, the loop variable is actually a tuple of a variable integer and a
reference to a character. At the first iteration, the i variable gets 0 as a value, while the ch
value gets as a value the address of the first character of the array. At every iteration, both
i and ch are incremented.

```rust

let arr = ['a', 'b', 'c'];
for (i, ch) in arr.iter().enumerate() {
    print!("{} {}, ", i, *ch);
}

```


### any()

```rust

print!("{} ",
    [45, 8, 2, 6].iter().any(|n| *n < 0));
print!("{} ",
    [45, 8, -2, 6].iter().any(|n| *n < 0));

// This will print: "false true".

```

### all()

```rust

print!("{} ", [45, 8, 2, 6].iter()
    .all(|n: &i32| -> bool { *n > 0 }));
print!("{} ", [45, 8, -2, 6].iter()
    .all(|n: &i32| -> bool { *n > 0 }));

// This will print: "true false".

```

### count(), sum(), min(), max() 

they are all itrator consumers

### collect()

to collect all iterator values into a Vector

```rust

let arr = [36, 1, 15, 9, 4];
let v = arr.iter().collect::<Vec<&i32>>();
print!("{:?}", v);

// This will print: "[36, 1, 15, 9, 4]".


let arr = [36, 1, 15, 9, 4];
let v = arr.iter().collect::<Vec<_>>();
print!("{:?}", v);


```

### chains

```rust

let arr = [66, -8, 43, 19, 0, -31];
let v = arr
    .iter()
    .filter(|x| **x > 0)
    .map(|x| *x * 2)
    .collect::<Vec<_>>();
print!("{:?}", v);

// This will print [132, 86, 38].

```

an important think to notice is that all iterator actions are lazy, that mean the closure function are called only when needed

## Input/Output and Error Handling

### Command-Line Arguments

The args standard library function returns an iterator over the command-line
arguments. Such an iterator has type Args, and it produces String values. The first
value produced is the program name, with the path used to reach it. The others are the
program arguments.

```rust

let command_line: std::env::Args = std::env::args();
for argument in command_line {
    println!("[{}]", argument);
}

```

### Process Return Code

```rust

std::process::exit(107);

```

### Environment Variables

```rust

for var in std::env::vars() {
    println!("[{}]=[{}]", var.0, var.1);
}

```


```rust

print!("{}",
    if std::env::var("abcd").is_ok() {
        "Already defined"
    } else {
        "Undefined"
    });
std::env::set_var("abcd", "This is the value");
print!(", {}.", match std::env::var("abcd") {
    Ok(value) => value,
    Err(err) => format!("Still undefined: {}", err),
});

```

### Reading from the Console

For command-line oriented programs, a typical way to get input is to read a line from the
keyboard until the user presses Enter.

```rust

let mut line = String::new();
println!("{:?}", std::io::stdin().read_line(&mut line));
println!("[{}]", line);

```

### Proper Runtime Error Handling

In real-world software, often it happens to make many invocations of functions that
return a "Result" type value. Let’s call “fallible” such functions. A fallible function
normally returns an "Ok," but in exceptional cases it returns an "Err"


```rust

fn f(x: i32) -> Result<i32, String> {
    f4(f3(f2(f1(x)?)?)?)
}

```

The question mark is a special macro such that, when applied to an expression like
in "e?", if "e" is of generic type "Result<T,E>", it is expanded as the expression "match
e { Some(v) => v, _ => return e }"; instead, if "e" is of a generic type "Option<T>",
it is expanded as the expression "match e { Ok(v) => v, _ => return e }". In other
words, such macro examines if its argument is "Some" or "Ok", and in such case unwraps
it, or otherwise returns it as a return value of the containing function.

It can be applied only to expressions of type "Result<T,E>" or "Option<T>", and, of
course, if can be used only inside a function with a proper return value type


### Writing to console

The "stdout" standard library function returns a handle to the standard output
stream of the current process. On that handle, the "write" function can be applied.

### Converting a Value to a String

If you want to print the textual representation of another kind of value, you can try to use
the "to_string" function, defined for all primitive types



```rust

let int_str: String = 45.to_string();
let float_str: String = 4.5.to_string();
let bool_str: String = true.to_string();
print!("{} {} {}", int_str, float_str, bool_str);

```

### File Input/Output


write to a file 

```rust

use std::io::Write;
let mut file = std::fs::File::create("data.txt").unwrap();
file.write_all("eè€".as_bytes()).unwrap();

```

read from file

```rust

use std::io::Read;
let mut file = std::fs::File::open("data.txt").unwrap();
let mut contents = String::new();
file.read_to_string(&mut contents).unwrap();
print!("{}", contents);

```

read from source and write to destination

```rust

use std::io::Read;
use std::io::Write;
let mut command_line: std::env::Args = std::env::args();
command_line.next().unwrap();
let source = command_line.next().unwrap();
let destination = command_line.next().unwrap();
let mut file_in = std::fs::File::open(source).unwrap();
let mut file_out = std::fs::File::create(destination).unwrap();
let mut buffer = [0u8; 4096];
loop {
    let nbytes = file_in.read(&mut buffer).unwrap();
    file_out.write(&buffer[..nbytes]).unwrap();
    if nbytes < buffer.len() { break; }
}

```

## Using Traits

A Rust trait is a container of function signatures.
The meaning of a trait is the capability to use some functions

for example:

```rust
trait HasSquareRoot {
    fn sq_root(self) -> Self;
}
impl HasSquareRoot for f32 {
    fn sq_root(self) -> Self { f32::sqrt(self) }
}
impl HasSquareRoot for f64 {
    fn sq_root(self) -> Self { f64::sqrt(self) }
}
fn quartic_root<Number>(x: Number) -> Number
where Number: HasSquareRoot {
    x.sq_root().sq_root()
}
```

The meaning of the "HasSquareRoot" trait is that the "sq_root" function can be invoked on every type having the "HasSquareRoot" capability, or, as it is usually said, every type that satisfies the "HasSquareRoot" trait

Those "impl" statements mean that the "HasSquareRoot" trait, **which is just a programming interface, or API,** is implemented here for the specified types by the specified code

### Traits with Multiple Functions

more then one function in the trait

```rust
trait HasSqrtAndAbs {
    fn sqrt(self) -> Self;
    fn abs(self) -> Self;
}
impl HasSqrtAndAbs for f64 {
    fn sqrt(self) -> Self { f64::sqrt(self) }
    fn abs(self) -> Self { f64::abs(self) }
}
impl HasSqrtAndAbs for f32 {
    fn sqrt(self) -> Self { f32::sqrt(self) }
    fn abs(self) -> Self { f32::abs(self) }
}
fn abs_quartic_root<Number>(x: Number) -> Number
where Number: HasSqrtAndAbs {
    x.abs().sqrt().sqrt()
}
```

or more traits with one function

```rust
trait HasSquareRoot {
    fn sqrt(self) -> Self;
}
impl HasSquareRoot for f32 {
    fn sqrt(self) -> Self { f32::sqrt(self) }
}
impl HasSquareRoot for f64 {
    fn sqrt(self) -> Self { f64::sqrt(self) }
}
trait HasAbsoluteValue {
    fn abs(self) -> Self;
}
impl HasAbsoluteValue for f32 {
    fn abs(self) -> Self { f32::abs(self) }
}
impl HasAbsoluteValue for f64 {
    fn abs(self) -> Self { f64::abs(self) }
}
fn abs_quartic_root<Number>(x: Number) -> Number
where Number: HasSquareRoot + HasAbsoluteValue {
    x.abs().sqrt().sqrt()
}
```

### Methods

We already saw that there are two possible syntaxes to invoke a function: "f(x, y)" and "x.f(y)". The first one is the “functional” syntax, and the second one is the “object-oriented” syntax

```rust
print!("{},", "abcd".to_string());
print!("{},", [1, 2, 3].len());
let mut v1 = vec![0u8; 0];
v1.push(7u8);
print!("{:?}; ", v1);

print!("{},", std::string::ToString::to_string("abcd"));
print!("{:?},", <[i32]>::len(&[1, 2, 3]));
let mut v2 = vec![0u8; 0];
Vec::push(&mut v2, 7u8);
print!("{:?}", v2);
```

```rust

fn double(x: i32) -> i32 {
    x * 2
}

trait CanBeDoubled {
    fn double(self) -> Self;
}

impl CanBeDoubled for i32 {
    fn double(self) -> Self {
        self * 2
    }
}

print!("{} {}", double(7i32), 7i32.double());
```

A new capability has been declared to be given to some types. It is the ability to
compute the double of the objects of such types, and so it is named CanBeDoubled.

For a type, to have such a capability means that there is a function named double that can get
a value of such type (self), and it can return a value of that same type (Self). There is no
guarantee that such return value is in any way the “double” of the argument.

### The “self” and “Self” Keywords

In the statement "trait CanBeDoubled { fn double(self) -> Self; }", "self"
means the value on which the "double" method will be applied, whichever it will be,
and "Self" means the type of "self".

So, the "self" word is a pseudo-argument of a method, and the "Self" word represents
the type of such an argument. Therefore, "self" and "Self" can be used only inside a "trait
or impl block. And "self", if present, must be the first argument of a method.

### The “Iterator” Trait

About the "Iterator" standard library trait, we said that it contains only one item: the
"next" function signature
the "Iterator" trait must be, in some way, generic in the type of the items it produces.

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

```

### Using Associated Types to Simplify Generic Traits Use

```rust

trait Searchable { //1
    type Key; //2
    type Count; //3
    fn contains(&self, key: Self::Key) -> bool; //4
    fn count(&self, key: Self::Key) -> Self::Count; //5
}


struct RecordWithId {
    id: u32,
    _descr: String,
}
struct NameSetWithId {
    data: Vec<RecordWithId>,
}

impl Searchable for NameSetWithId { //6
    type Key = u32; //7
    type Count = usize; //8
    fn contains(&self, key: Self::Key) -> bool { //9
        for record in self.data.iter() {
            if record.id == key {
                return true;
            }
        }
        false
    }
    fn count(&self, key: Self::Key) -> usize { //10
        let mut c = 0;
        for record in self.data.iter() {
            if record.id == key {
                c += 1;
            }
        }
        c
    }
}


```

## Object-Oriented Programming

### Composition Instead of Inheritance

Rust does not support inheritance. In place of data inheritance, Rust uses composition

Let's assume we already have a type representing a text to draw on the graphical screen, and we want to create a type representing a text surrounded by a rectangle

``` rust
struct Text { characters: String }
impl Text {
    fn from(text: &str) -> Text {
        Text { characters: text.to_string() }
    }
    fn draw(&self) {
        print!("{}", self.characters);
    }
}

```

``` rust

struct BoxedText {
    text: Text,
    first: char,
    last: char,
}

impl BoxedText {
    fn with_text_and_borders(
        text: &str, first: char, last: char)
        -> BoxedText
    {
        BoxedText {
            text: Text::from(text),
            first: first,
            last: last,
        }
    }
    fn draw(&self) {
        print!("{}", self.first);
        self.text.draw();
        print!("{}", self.last);
    }
}
```

```rust

let boxed_greeting =
    BoxedText::with_text_and_borders("Hi", '[', ']');
print!(", ");
boxed_greeting.draw();

// This will print: "Hello [Hi]".
```


### Static Dispatch

``` rust
trait Draw {
    fn draw(&self);
}

struct Text { characters: String }
impl Text {
    fn from(text: &str) -> Text {
        Text { characters: text.to_string() }
    }
}
impl Draw for Text {
    fn draw(&self) {
        print!("{}", self.characters);
    }
}


struct BoxedText {
    text: Text,
    first: char,
    last: char,
}

impl BoxedText {
    fn with_text_and_borders(
        text: &str, first: char, last: char)
        -> BoxedText
    {
        BoxedText {
            text: Text::from(text),
            first: first,
            last: last,
        }
    }
}

impl Draw for BoxedText {
    fn draw(&self) {
        print!("{}", self.first);
        self.text.draw();
        print!("{}", self.last);
    }
}
```

```rust

let greeting = Text::from("Hello");
let boxed_greeting =
    BoxedText::with_text_and_borders("Hi", '[', ']');

// SOLUTION 1 //
fn draw_text<T>(txt: T) where T: Draw {
    txt.draw();
}

draw_text(greeting);
print!(", ");
draw_text(boxed_greeting);

// This will print: "Hello, [Hi]"
```

The "draw_text" generic function receives an
argument of "T" type, where "T" is any type that implements the "Draw" trait. Because of
this, it is allowed to invoke the "draw" function on that argument.

> In this program, this choice is performed by the compiler, at
compile time, and so this dispatch is “static”.

### Dynamic Dispatch

```rust

// SOLUTION 2 //
fn draw_text(txt: &Draw) {
    txt.draw();
}
draw_text(&greeting);
print!(", ");
draw_text(&boxed_greeting);

```

So now, instead of a generic function, we have a concrete function, which gets as argument a reference to a trait.

A trait is not a type. You cannot declare a variable or a function argument having a trait as type

**A reference to a trait is a valid type.**

