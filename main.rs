// =============================================================================
//  rust_showcase.rs — A comprehensive tour of Rust language features
//  Compile:  rustc rust_showcase.rs -o rust_showcase
//  Run:      ./rust_showcase
// =============================================================================

#![allow(dead_code, unused_variables, unused_mut, unused_imports)]
#![warn(clippy::all)]

// ─────────────────────────────────────────────────────────────────────────────
// 1. MODULES & VISIBILITY
// ─────────────────────────────────────────────────────────────────────────────
mod geometry {
    /// A point in 2-D space.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Point {
        pub x: f64,
        pub y: f64,
    }

    impl Point {
        pub fn new(x: f64, y: f64) -> Self {
            Self { x, y }
        }

        pub fn distance(&self, other: &Point) -> f64 {
            ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
        }
    }

    pub mod shapes {
        use super::Point;

        pub trait Area {
            fn area(&self) -> f64;
            fn perimeter(&self) -> f64;
            fn describe(&self) -> String {
                format!(
                    "area={:.2}, perimeter={:.2}",
                    self.area(),
                    self.perimeter()
                )
            }
        }

        pub struct Circle {
            pub center: Point,
            pub radius: f64,
        }

        pub struct Rectangle {
            pub top_left: Point,
            pub width: f64,
            pub height: f64,
        }

        impl Area for Circle {
            fn area(&self) -> f64 {
                std::f64::consts::PI * self.radius * self.radius
            }
            fn perimeter(&self) -> f64 {
                2.0 * std::f64::consts::PI * self.radius
            }
        }

        impl Area for Rectangle {
            fn area(&self) -> f64 {
                self.width * self.height
            }
            fn perimeter(&self) -> f64 {
                2.0 * (self.width + self.height)
            }
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 2. ENUMS, PATTERN MATCHING & OPTION / RESULT
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug)]
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
    RegularPolygon { sides: u32, side_len: f64 },
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle { base, height } => 0.5 * base * height,
            Shape::RegularPolygon { sides, side_len } => {
                let n = *sides as f64;
                (n * side_len * side_len) / (4.0 * (std::f64::consts::PI / n).tan())
            }
        }
    }

    fn name(&self) -> &'static str {
        match self {
            Shape::Circle { .. } => "Circle",
            Shape::Rectangle { .. } => "Rectangle",
            Shape::Triangle { .. } => "Triangle",
            Shape::RegularPolygon { sides: 3, .. } => "Equilateral Triangle",
            Shape::RegularPolygon { sides: 6, .. } => "Hexagon",
            Shape::RegularPolygon { .. } => "Polygon",
        }
    }
}

fn parse_number(s: &str) -> Result<f64, String> {
    s.trim()
        .parse::<f64>()
        .map_err(|e| format!("parse error: {e}"))
}

fn find_first_positive(nums: &[i32]) -> Option<i32> {
    nums.iter().copied().find(|&n| n > 0)
}

// ─────────────────────────────────────────────────────────────────────────────
// 3. GENERICS, TRAITS & TRAIT BOUNDS
// ─────────────────────────────────────────────────────────────────────────────

use std::fmt::{Debug, Display};
use std::ops::Add;

trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

#[derive(Debug)]
struct Article {
    title: String,
    author: String,
    content: String,
}

impl Summary for Article {
    fn summarize_author(&self) -> String {
        self.author.clone()
    }
    fn summarize(&self) -> String {
        format!("{}, by {} — {}", self.title, self.author, &self.content[..self.content.len().min(40)])
    }
}

/// Generic function with multiple trait bounds
fn print_summary<T: Summary + Debug>(item: &T) {
    println!("  [debug]   {:?}", item);
    println!("  [summary] {}", item.summarize());
}

/// Generic struct
#[derive(Debug)]
struct Pair<T> {
    first: T,
    second: T,
}

impl<T: Display + PartialOrd> Pair<T> {
    fn new(first: T, second: T) -> Self {
        Self { first, second }
    }

    fn larger(&self) -> &T {
        if self.first >= self.second { &self.first } else { &self.second }
    }
}

/// Generic function returning a value (monomorphisation demo)
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest { largest = item; }
    }
    largest
}

// ─────────────────────────────────────────────────────────────────────────────
// 4. LIFETIMES
// ─────────────────────────────────────────────────────────────────────────────

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

struct Important<'a> {
    part: &'a str,
}

impl<'a> Display for Important<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Important: '{}'", self.part)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 5. CLOSURES, ITERATORS & FUNCTIONAL PATTERNS
// ─────────────────────────────────────────────────────────────────────────────

fn apply_twice<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(f(x))
}

fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n           // closure capturing by move
}

fn demonstrate_iterators() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // chained adapters
    let result: Vec<i32> = numbers
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .collect();
    println!("  even squares: {:?}", result);

    // fold / reduce
    let sum: i32 = numbers.iter().sum();
    let product: i32 = numbers.iter().product();
    println!("  sum={sum}, product={product}");

    // zip & enumerate
    let letters = vec!['a', 'b', 'c'];
    let zipped: Vec<_> = numbers.iter().zip(letters.iter()).collect();
    println!("  zipped (first 3): {:?}", &zipped[..3]);

    // flat_map
    let words = vec!["hello world", "foo bar"];
    let chars: Vec<&str> = words.iter().flat_map(|s| s.split_whitespace()).collect();
    println!("  flat_map words: {:?}", chars);

    // custom iterator via take / skip / chain
    let chain: Vec<i32> = (1..=3).chain(8..=10).collect();
    println!("  chain: {:?}", chain);
}

// ─────────────────────────────────────────────────────────────────────────────
// 6. OWNERSHIP, BORROWING & SLICES
// ─────────────────────────────────────────────────────────────────────────────

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        if b == b' ' { return &s[..i]; }
    }
    s
}

fn ownership_demo() {
    let s1 = String::from("hello");
    let s2 = s1.clone();          // deep copy — s1 still valid
    let len = calculate_length(&s1); // borrow
    println!("  s1={s1}, s2={s2}, len={len}");

    let mut s = String::from("hello");
    change(&mut s);               // mutable borrow
    println!("  after change: {s}");

    // slice types
    let arr = [1, 2, 3, 4, 5];
    let slice: &[i32] = &arr[1..3];
    println!("  array slice: {:?}", slice);
}

fn calculate_length(s: &String) -> usize { s.len() }
fn change(s: &mut String) { s.push_str(", world"); }

// ─────────────────────────────────────────────────────────────────────────────
// 7. STRUCTS, METHODS & OPERATOR OVERLOADING
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vec2 {
    x: f64,
    y: f64,
}

impl Vec2 {
    const ZERO: Vec2 = Vec2 { x: 0.0, y: 0.0 };

    fn new(x: f64, y: f64) -> Self { Self { x, y } }
    fn length(&self) -> f64 { (self.x * self.x + self.y * self.y).sqrt() }
    fn dot(&self, other: &Vec2) -> f64 { self.x * other.x + self.y * other.y }
    fn normalize(&self) -> Self {
        let len = self.length();
        Self { x: self.x / len, y: self.y / len }
    }
}

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:.2}, {:.2})", self.x, self.y)
    }
}

impl Default for Vec2 {
    fn default() -> Self { Vec2::ZERO }
}

// ─────────────────────────────────────────────────────────────────────────────
// 8. SMART POINTERS: Box, Rc, RefCell, Cell
// ─────────────────────────────────────────────────────────────────────────────

use std::cell::{Cell, RefCell};
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn smart_pointers_demo() {
    // Box — heap allocation / recursive types
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("  box list: {:?}", list);

    // Rc — shared ownership
    let a = Rc::new(vec![1, 2, 3]);
    let b = Rc::clone(&a);
    println!("  rc refs={}, a={:?}, b={:?}", Rc::strong_count(&a), a, b);

    // RefCell — interior mutability
    let data = RefCell::new(vec![1, 2, 3]);
    data.borrow_mut().push(4);
    println!("  refcell: {:?}", data.borrow());

    // Cell — Copy types with interior mutability
    let flag = Cell::new(false);
    flag.set(true);
    println!("  cell flag: {}", flag.get());

    // Rc<RefCell<T>> — shared + mutable
    let shared = Rc::new(RefCell::new(0));
    let clone1 = Rc::clone(&shared);
    *clone1.borrow_mut() += 10;
    println!("  Rc<RefCell>: {}", shared.borrow());
}

// ─────────────────────────────────────────────────────────────────────────────
// 9. TRAIT OBJECTS & DYNAMIC DISPATCH
// ─────────────────────────────────────────────────────────────────────────────

trait Animal: Debug {
    fn name(&self) -> &str;
    fn sound(&self) -> &str;
    fn speak(&self) { println!("  {} says {}", self.name(), self.sound()); }
}

#[derive(Debug)] struct Dog { name: String }
#[derive(Debug)] struct Cat { name: String }
#[derive(Debug)] struct Parrot { name: String, phrase: String }

impl Animal for Dog    { fn name(&self)->&str{&self.name} fn sound(&self)->&str{"Woof"} }
impl Animal for Cat    { fn name(&self)->&str{&self.name} fn sound(&self)->&str{"Meow"} }
impl Animal for Parrot {
    fn name(&self) -> &str { &self.name }
    fn sound(&self) -> &str { &self.phrase }
}

fn make_all_speak(animals: &[Box<dyn Animal>]) {
    for a in animals { a.speak(); }
}

// ─────────────────────────────────────────────────────────────────────────────
// 10. ERROR HANDLING — custom error types, ? operator, From
// ─────────────────────────────────────────────────────────────────────────────

use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
enum AppError {
    ParseError(ParseIntError),
    NegativeNumber(i64),
    TooBig(i64),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::ParseError(e)    => write!(f, "parse error: {e}"),
            AppError::NegativeNumber(n) => write!(f, "negative number: {n}"),
            AppError::TooBig(n)         => write!(f, "number too big: {n}"),
        }
    }
}

impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self { AppError::ParseError(e) }
}

fn validate(s: &str) -> Result<i64, AppError> {
    let n: i64 = s.trim().parse::<i64>()?; // ? uses From<ParseIntError>
    if n < 0  { return Err(AppError::NegativeNumber(n)); }
    if n > 100 { return Err(AppError::TooBig(n)); }
    Ok(n)
}

// ─────────────────────────────────────────────────────────────────────────────
// 11. ITERATORS — implementing the Iterator trait
// ─────────────────────────────────────────────────────────────────────────────

struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    fn new() -> Self { Fibonacci { a: 0, b: 1 } }
}

impl Iterator for Fibonacci {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let next = self.a + self.b;
        self.a = self.b;
        self.b = next;
        Some(self.a)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 12. CONCURRENCY — threads, channels, Arc<Mutex<T>>
// ─────────────────────────────────────────────────────────────────────────────

use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc;

fn concurrency_demo() {
    // Basic thread spawn + join
    let handle = thread::spawn(|| {
        let sum: u64 = (1..=1_000_000).sum();
        sum
    });
    println!("  thread sum 1..1M = {}", handle.join().unwrap());

    // Arc<Mutex<T>> — shared mutable state
    let counter = Arc::new(Mutex::new(0u32));
    let mut handles = vec![];
    for _ in 0..4 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut lock = c.lock().unwrap();
            *lock += 25;
        }));
    }
    for h in handles { h.join().unwrap(); }
    println!("  arc/mutex counter (4×25) = {}", *counter.lock().unwrap());

    // mpsc channel
    let (tx, rx) = mpsc::channel::<String>();
    let tx2 = tx.clone();
    thread::spawn(move || { tx.send("hello from thread 1".to_string()).unwrap(); });
    thread::spawn(move || { tx2.send("hello from thread 2".to_string()).unwrap(); });
    for _ in 0..2 {
        println!("  channel received: {}", rx.recv().unwrap());
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 13. MACROS
// ─────────────────────────────────────────────────────────────────────────────

/// Declarative macro — a mini `vec!` for `HashMap`
macro_rules! map {
    ($($k:expr => $v:expr),* $(,)?) => {{
        let mut m = std::collections::HashMap::new();
        $( m.insert($k, $v); )*
        m
    }};
}

/// Variadic max macro
macro_rules! max {
    ($x:expr) => ($x);
    ($x:expr, $($rest:expr),+) => {
        std::cmp::max($x, max!($($rest),+))
    };
}

// ─────────────────────────────────────────────────────────────────────────────
// 14. STRING HANDLING
// ─────────────────────────────────────────────────────────────────────────────

fn string_demo() {
    // &str vs String
    let s_literal: &str = "hello, world";
    let s_owned: String = s_literal.to_uppercase();
    let s_formatted = format!("{} — len={}", s_owned, s_owned.len());

    // slicing, splitting, collecting
    let csv = "one,two,three,four";
    let parts: Vec<&str> = csv.split(',').collect();
    println!("  csv parts: {:?}", parts);

    // bytes, chars, grapheme clusters (chars here)
    let emoji = "Hello 🌍!";
    println!("  '{}' — {} chars, {} bytes", emoji, emoji.chars().count(), emoji.len());

    // String building
    let mut built = String::with_capacity(64);
    for word in &parts { built.push_str(word); built.push(' '); }
    println!("  built: '{}'", built.trim());

    // Pattern matching on strings
    let greeting = "Hi there";
    let resp = match greeting {
        s if s.starts_with("Hi")    => "Hey!",
        s if s.starts_with("Hello") => "Greetings!",
        _                           => "...",
    };
    println!("  response: {resp}");
}

// ─────────────────────────────────────────────────────────────────────────────
// 15. COLLECTIONS — HashMap, HashSet, BTreeMap, VecDeque
// ─────────────────────────────────────────────────────────────────────────────

use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

fn collections_demo() {
    // HashMap with entry API
    let mut scores: HashMap<&str, u32> = HashMap::new();
    for name in ["Alice", "Bob", "Alice", "Carol", "Bob", "Alice"] {
        *scores.entry(name).or_insert(0) += 1;
    }
    println!("  word counts: {:?}", scores);

    // macro-built map
    let capitals = map! {
        "Sweden"  => "Stockholm",
        "Germany" => "Berlin",
        "Japan"   => "Tokyo",
    };
    println!("  capitals: {:?}", capitals);

    // HashSet operations
    let a: HashSet<i32> = [1, 2, 3, 4].iter().cloned().collect();
    let b: HashSet<i32> = [3, 4, 5, 6].iter().cloned().collect();
    let mut inter: Vec<i32> = a.intersection(&b).cloned().collect();
    inter.sort();
    println!("  intersection: {:?}", inter);

    // BTreeMap (sorted)
    let mut btree: BTreeMap<&str, i32> = BTreeMap::new();
    btree.insert("banana", 3); btree.insert("apple", 1); btree.insert("cherry", 2);
    println!("  sorted btree: {:?}", btree);

    // VecDeque — efficient front/back operations
    let mut deque: VecDeque<i32> = (1..=5).collect();
    deque.push_front(0);
    deque.push_back(6);
    println!("  deque: {:?}", deque);
}

// ─────────────────────────────────────────────────────────────────────────────
// 16. TYPE SYSTEM — type aliases, newtype pattern, From/Into, TryFrom
// ─────────────────────────────────────────────────────────────────────────────

type Meters = f64;
type Kilograms = f64;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Celsius(f64);

#[derive(Debug, Clone, Copy)]
struct Fahrenheit(f64);

impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Self { Fahrenheit(c.0 * 9.0 / 5.0 + 32.0) }
}

impl From<Fahrenheit> for Celsius {
    fn from(f: Fahrenheit) -> Self { Celsius((f.0 - 32.0) * 5.0 / 9.0) }
}

impl Display for Celsius    { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{:.1}°C", self.0) } }
impl Display for Fahrenheit { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{:.1}°F", self.0) } }

use std::convert::TryFrom;

#[derive(Debug)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = String;
    fn try_from(n: i32) -> Result<Self, Self::Error> {
        if n % 2 == 0 { Ok(EvenNumber(n)) }
        else           { Err(format!("{n} is not even")) }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 17. UNSAFE RUST
// ─────────────────────────────────────────────────────────────────────────────

fn unsafe_demo() {
    // Raw pointer arithmetic
    let mut value: i32 = 42;
    let r = &value as *const i32;
    let rw = &mut value as *mut i32;

    unsafe {
        println!("  raw ptr *r = {}", *r);
        *rw = 100;
        println!("  after write *r = {}", *r);
    }

    // Calling an unsafe function
    unsafe fn dangerous() -> i32 { 7 }
    let result = unsafe { dangerous() };
    println!("  unsafe fn result: {result}");

    // FFI call — abs from C stdlib
    unsafe extern "C" { fn abs(n: i32) -> i32; }
    let neg = -99i32;
    println!("  ffi abs({neg}) = {}", unsafe { abs(neg) });
}

// ─────────────────────────────────────────────────────────────────────────────
// 18. ASSOCIATED TYPES & WHERE CLAUSES
// ─────────────────────────────────────────────────────────────────────────────

trait Container {
    type Item;
    fn first(&self) -> Option<&Self::Item>;
    fn last(&self)  -> Option<&Self::Item>;
    fn len(&self)   -> usize;
    fn is_empty(&self) -> bool { self.len() == 0 }
}

struct Stack<T> { data: Vec<T> }

impl<T> Stack<T> {
    fn new() -> Self { Stack { data: Vec::new() } }
    fn push(&mut self, v: T) { self.data.push(v); }
    fn pop(&mut self) -> Option<T> { self.data.pop() }
}

impl<T> Container for Stack<T> {
    type Item = T;
    fn first(&self) -> Option<&T> { self.data.first() }
    fn last(&self)  -> Option<&T> { self.data.last() }
    fn len(&self)   -> usize      { self.data.len() }
}

fn print_container<C>(c: &C)
where
    C: Container,
    C::Item: Debug + Display,
{
    println!("  container len={}, first={:?}, last={:?}",
        c.len(), c.first(), c.last());
}

// ─────────────────────────────────────────────────────────────────────────────
// 19. BUILDER PATTERN
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug)]
struct Config {
    host:       String,
    port:       u16,
    max_conns:  u32,
    tls:        bool,
}

struct ConfigBuilder {
    host:       String,
    port:       u16,
    max_conns:  u32,
    tls:        bool,
}

impl ConfigBuilder {
    fn new() -> Self {
        ConfigBuilder { host: "localhost".into(), port: 8080, max_conns: 100, tls: false }
    }
    fn host(mut self, h: &str)    -> Self { self.host = h.to_string(); self }
    fn port(mut self, p: u16)     -> Self { self.port = p; self }
    fn max_conns(mut self, n: u32)-> Self { self.max_conns = n; self }
    fn tls(mut self, t: bool)     -> Self { self.tls = t; self }
    fn build(self) -> Config {
        Config { host: self.host, port: self.port, max_conns: self.max_conns, tls: self.tls }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 20. ADVANCED PATTERN MATCHING — guards, binding, destructuring
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

fn process_message(msg: &Message) -> String {
    match msg {
        Message::Quit                     => "quit".to_string(),
        Message::Move { x, y } if *x == 0 && *y == 0
                                          => "move to origin".to_string(),
        Message::Move { x, y }            => format!("move to ({x},{y})"),
        Message::Write(text)              => format!("write: {text}"),
        Message::ChangeColor(r, g, b)     => format!("color rgb({r},{g},{b})"),
    }
}

fn advanced_destructuring() {
    // tuple destructuring
    let (a, b, c) = (1, "two", 3.0_f64);
    println!("  tuple: a={a}, b={b}, c={c}");

    // struct destructuring
    let p = geometry::Point::new(3.0, 4.0);
    let geometry::Point { x, y } = p;
    println!("  point: x={x}, y={y}");

    // nested + @ bindings
    let nums = [1, 2, 3, 4, 5];
    if let [first, .., last] = nums {
        println!("  slice pattern: first={first}, last={last}");
    }

    let n = 15_u32;
    let desc = match n {
        x @ 1..=9   => format!("single digit {x}"),
        x @ 10..=99 => format!("double digit {x}"),
        x           => format!("large {x}"),
    };
    println!("  n={n}: {desc}");
}

// ─────────────────────────────────────────────────────────────────────────────
// 21. CONST & STATIC, CONST GENERICS
// ─────────────────────────────────────────────────────────────────────────────

const MAX_POINTS: u32 = 100_000;
static HELLO_WORLD: &str = "Hello, Rust!";

/// Const-generic array wrapper
#[derive(Debug)]
struct Grid<const W: usize, const H: usize> {
    cells: [[u8; W]; H],
}

impl<const W: usize, const H: usize> Grid<W, H> {
    fn new() -> Self { Grid { cells: [[0; W]; H] } }
    fn set(&mut self, row: usize, col: usize, val: u8) { self.cells[row][col] = val; }
    fn get(&self, row: usize, col: usize) -> u8 { self.cells[row][col] }
}

// ─────────────────────────────────────────────────────────────────────────────
// 22. TRAIT IMPLEMENTATIONS: Display, From, Default, Clone, PartialEq, Hash
// ─────────────────────────────────────────────────────────────────────────────

use std::hash::{Hash, Hasher};

#[derive(Clone, Debug)]
struct Color {
    r: u8, g: u8, b: u8,
}

impl Color {
    fn new(r: u8, g: u8, b: u8) -> Self { Color { r, g, b } }
    fn to_hex(&self) -> String { format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b) }
}

impl Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rgb({},{},{})", self.r, self.g, self.b)
    }
}

impl Default for Color {
    fn default() -> Self { Color { r: 0, g: 0, b: 0 } }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b
    }
}

impl Eq for Color {}

impl Hash for Color {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.r.hash(state); self.g.hash(state); self.b.hash(state);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// MAIN — exercise every section
// ─────────────────────────────────────────────────────────────────────────────

fn main() {
    section("1. MODULES & TRAITS (geometry)");
    {
        use geometry::Point;
        use geometry::shapes::{Area, Circle, Rectangle};
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(3.0, 4.0);
        println!("  distance p1→p2 = {:.2}", p1.distance(&p2));
        let c = Circle { center: p1, radius: 5.0 };
        let r = Rectangle { top_left: p1, width: 4.0, height: 6.0 };
        println!("  circle:    {}", c.describe());
        println!("  rectangle: {}", r.describe());
    }

    section("2. ENUMS & PATTERN MATCHING");
    {
        let shapes = vec![
            Shape::Circle { radius: 3.0 },
            Shape::Rectangle { width: 4.0, height: 5.0 },
            Shape::Triangle { base: 6.0, height: 8.0 },
            Shape::RegularPolygon { sides: 6, side_len: 2.0 },
        ];
        for s in &shapes {
            println!("  {:15} area = {:.2}", s.name(), s.area());
        }
        println!("  parse '3.14'  → {:?}", parse_number("3.14"));
        println!("  parse 'bad'   → {:?}", parse_number("bad"));
        println!("  first_positive([−3,−1,5,2]) → {:?}", find_first_positive(&[-3,-1,5,2]));
    }

    section("3. GENERICS, TRAITS, BOUNDS");
    {
        let article = Article {
            title:   "Rust 2024 Edition".to_string(),
            author:  "The Team".to_string(),
            content: "Major improvements land in the 2024 edition of Rust.".to_string(),
        };
        print_summary(&article);

        let pair = Pair::new(5, 10);
        println!("  larger of ({},{}) = {}", pair.first, pair.second, pair.larger());

        let numbers = vec![34, 50, 25, 100, 65];
        println!("  largest in {:?} = {}", numbers, largest(&numbers));
    }

    section("4. LIFETIMES");
    {
        let s1 = String::from("long string is long");
        let result;
        {
            let s2 = String::from("xyz");
            result = longest(s1.as_str(), s2.as_str());
            println!("  longest = '{result}'");
        }
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first = novel.split('.').next().expect("no sentence");
        let imp = Important { part: first };
        println!("  {imp}");
    }

    section("5. CLOSURES & ITERATORS");
    {
        let double = |x: i32| x * 2;
        println!("  apply_twice(double, 3) = {}", apply_twice(double, 3));
        let add5 = make_adder(5);
        println!("  add5(10) = {}", add5(10));
        demonstrate_iterators();
        // custom iterator
        let fibs: Vec<u64> = Fibonacci::new().take(10).collect();
        println!("  fibonacci(10): {:?}", fibs);
    }

    section("6. OWNERSHIP & BORROWING");
    ownership_demo();

    section("7. STRUCTS & OPERATOR OVERLOADING");
    {
        let a = Vec2::new(3.0, 4.0);
        let b = Vec2::new(1.0, 2.0);
        println!("  a={a}, b={b}");
        println!("  a+b = {}", a + b);
        println!("  |a| = {:.2}", a.length());
        println!("  a·b = {:.2}", a.dot(&b));
        println!("  â   = {}", a.normalize());
        println!("  default = {}", Vec2::default());
    }

    section("8. SMART POINTERS");
    smart_pointers_demo();

    section("9. TRAIT OBJECTS & DYNAMIC DISPATCH");
    {
        let animals: Vec<Box<dyn Animal>> = vec![
            Box::new(Dog    { name: "Rex".to_string() }),
            Box::new(Cat    { name: "Whiskers".to_string() }),
            Box::new(Parrot { name: "Polly".to_string(), phrase: "Pieces of eight!".to_string() }),
        ];
        make_all_speak(&animals);
    }

    section("10. ERROR HANDLING");
    {
        for input in ["42", "-5", "200", "abc"] {
            match validate(input) {
                Ok(n)  => println!("  '{}' → Ok({})", input, n),
                Err(e) => println!("  '{}' → Err({})", input, e),
            }
        }
    }

    section("11. CUSTOM ITERATOR (Fibonacci)");
    {
        let sum: u64 = Fibonacci::new()
            .take_while(|&n| n < 1000)
            .filter(|n| n % 2 == 0)
            .sum();
        println!("  sum of even fibonacci < 1000 = {sum}");
    }

    section("12. CONCURRENCY");
    concurrency_demo();

    section("13. MACROS");
    {
        println!("  max!(1,9,3,7,2) = {}", max!(1, 9, 3, 7, 2));
        let m = map!["a" => 1, "b" => 2, "c" => 3];
        let mut keys: Vec<&&str> = m.keys().collect();
        keys.sort();
        println!("  map keys: {:?}", keys);
    }

    section("14. STRING HANDLING");
    string_demo();

    section("15. COLLECTIONS");
    collections_demo();

    section("16. TYPE CONVERSIONS (From/Into/TryFrom)");
    {
        let boiling = Celsius(100.0);
        let f: Fahrenheit = boiling.into();
        println!("  {} = {}", boiling, f);
        let body_temp = Fahrenheit(98.6);
        let c: Celsius = body_temp.into();
        println!("  {} = {}", body_temp, c);

        println!("  TryFrom 4  → {:?}", EvenNumber::try_from(4));
        println!("  TryFrom 7  → {:?}", EvenNumber::try_from(7));
    }

    section("17. UNSAFE RUST");
    unsafe_demo();

    section("18. ASSOCIATED TYPES & WHERE CLAUSES");
    {
        let mut stack: Stack<i32> = Stack::new();
        for i in 1..=5 { stack.push(i * 10); }
        print_container(&stack);
        println!("  popped: {:?}", stack.pop());
        print_container(&stack);
    }

    section("19. BUILDER PATTERN");
    {
        let cfg = ConfigBuilder::new()
            .host("example.com")
            .port(443)
            .max_conns(500)
            .tls(true)
            .build();
        println!("  config: {:?}", cfg);
    }

    section("20. ADVANCED PATTERN MATCHING");
    {
        let messages = [
            Message::Quit,
            Message::Move { x: 0, y: 0 },
            Message::Move { x: 3, y: -2 },
            Message::Write("hello".to_string()),
            Message::ChangeColor(255, 128, 0),
        ];
        for m in &messages {
            println!("  {:?} → {}", m, process_message(m));
        }
        advanced_destructuring();
    }

    section("21. CONST GENERICS & STATICS");
    {
        println!("  MAX_POINTS = {MAX_POINTS}");
        println!("  HELLO_WORLD = {HELLO_WORLD}");
        let mut grid: Grid<4, 3> = Grid::new();
        grid.set(1, 2, 9);
        println!("  grid[1][2] = {}", grid.get(1, 2));
        println!("  grid: {:?}", grid);
    }

    section("22. RICH TRAIT IMPLS (Color)");
    {
        let red = Color::new(255, 0, 0);
        let green = Color::new(0, 255, 0);
        let default_color = Color::default();
        println!("  red   = {red}  hex={}", red.to_hex());
        println!("  green = {green}  hex={}", green.to_hex());
        println!("  default = {default_color}");
        println!("  red == red? {}", red == red.clone());
        println!("  red == green? {}", red == green);

        // Use Color as HashMap key (requires Hash + Eq)
        let mut palette: HashMap<Color, &str> = HashMap::new();
        palette.insert(red.clone(), "Red");
        palette.insert(green.clone(), "Green");
        println!("  palette[red] = {:?}", palette.get(&red));
    }

    println!("\n{}", "═".repeat(60));
    println!("  All Rust features demonstrated successfully! 🦀");
    println!("{}", "═".repeat(60));
}

// ─────────────────────────────────────────────────────────────────────────────
// Helper
// ─────────────────────────────────────────────────────────────────────────────
fn section(title: &str) {
    println!("\n{}", "─".repeat(60));
    println!("  {}", title);
    println!("{}", "─".repeat(60));
}
