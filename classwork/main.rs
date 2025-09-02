fn make_change(amount: u32) -> Vec<(u32, u32)> {
    let mut changes: Vec<(u32, u32)> = Vec::new();
    let denominations: [u32; 6] = [100, 50, 20, 10, 5, 1];

    let mut remaining: u32 = amount;
    for &denom in &denominations {
        let count = remaining / denom;
        if count > 0 {
            changes.push((denom, count));
            remaining -= count * denom;
        }
    }
    changes
}

fn main(){
    let amount = 287;
    let change = make_change(amount);
    for (denom, count) in change {
        println!("Coin: {}, Count: {}", denom, count);
    }
}

















/* 
use std::thread;
use std::time::Duration;

fn main() {
    // Allocate a vector on the heap
    let mut vec = Vec::new();

    // Push some elements into the vector
    for i in 0..1000 {
        vec.push(i);
    }

    // Print the vector length and capacity
    println!("Vector length: {}", vec.len());
    println!("Vector capacity: {}", vec.capacity());

    // Spawn a thread to modify the vector
    let handle = thread::spawn(move || {
        for i in 1000..2000 {
            vec.push(i);
        }
        println!("Vector length in thread: {}", vec.len());
    });

    // Wait for the thread to finish
    handle.join().unwrap();

    // Sleep for a while to keep the program running
    thread::sleep(Duration::from_secs(5));
}

*/























/* use std::arch::asm;
#[cfg(target_arch = "x86_64")]
pub fn main(){
    println!("This is x86_64 assembly code");
    let mut x = 0;
    unsafe {
        asm!(
            "mov rdx, 0xA",
            "mov rax, 0x2A",
            "add rax, rdx",
            out("rax") x,
        );
    }
    println!("The value of x is: {}", x);
}



*/












/* use rand::Rng;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

fn generate_random_tuples(n: usize) -> Vec<(i32, i32)> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| (rng.gen_range(1..101), rng.gen_range(1..101))).collect()
}

fn main() {
    let l: Vec<(i32, i32)> = generate_random_tuples(1_000_000);

    // Sequential implementation
    let mut t = 0;
    let start = Instant::now();
    for (a, b) in &l {
        t += a + b;
    }
    let duration = start.elapsed();

    println!("Sequential Sum: {}", t);
    println!("Sequential Time taken: {:?}", duration);

    /* 
    // Naive Thread-based implementation
    let t = Arc::new(Mutex::new(0));
    let start = Instant::now();
    let mut handles = vec![];

    for (a, b) in l.clone() {
        let t = Arc::clone(&t);
        handles.push(thread::spawn(move || {
            let sum = a + b;
            let mut total = t.lock().unwrap();
            *total += sum;
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let duration = start.elapsed();
    println!("Thread-based Sum: {}", *t.lock().unwrap());
    println!("Thread-based Time taken: {:?}", duration);
    */
 
    // Thread-based implementation with configurable number of threads
    let num_threads = 4;
    let t = Arc::new(Mutex::new(0));
    let start = Instant::now();
    let mut handles = vec![];
    let chunk_size = l.len() / num_threads;

    for chunk in l.chunks(chunk_size) {
        let t = Arc::clone(&t);
        let chunk = chunk.to_vec();
        handles.push(thread::spawn(move || {
            let mut local_sum = 0;
            for (a, b) in chunk {
                local_sum += a + b;
            }
            let mut total = t.lock().unwrap();
            *total += local_sum;
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let duration = start.elapsed();
    println!("Configurable Thread-based Sum: {}", *t.lock().unwrap());
    println!("Configurable Thread-based Time taken: {:?}", duration);
    
}


*/





























/*use std::{rc::Rc, sync::{Arc, Mutex}, thread::{self, JoinHandle}};
fn main() {
    let mut handles: Vec<JoinHandle<()>> = vec![];
    let test = Arc::new(Mutex::new(String::from("test")));
    
    for t in 1..=2{
        let sharedClone = Arc::clone(&test);
        handles.push(thread::spawn(move || {
            if let Ok(locked_data) = sharedClone.lock() {
                let data = *locked_data;
                    
                }
            }
            for i in 1..=100000{
                println!("Thread {}: {}", t, i);
            }
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("All threads finished! {}", (*(*test).lock().unwrap()));
}



*/

















/*use std::cell::RefCell;
use std::fmt::{self, Display, Error};

#[derive(Clone)]
struct User<'a, 'b> {
    name: &'a str,
    email: &'b str,
    active: bool,
    age: i8,
}

fn main() {
    let a: &'static str = "bob";
    let b = "alice";
    let user = User{name: a, email: "fake", active: false, age: 12};
}

fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {a} else {b}
}


struct Folder{
    files: Vec<String>,
    folder: Box<Folder>,
}





use std::ops::Add;
use std::rc::{Rc, Weak};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point{
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}





fn identify_by_sound(sound: &str) -> Box<dyn Describable> {
    match sound {
        "bark" => Box::new(Dog),
        "meow" => Box::new(Cat),
        _ => panic!("I don't know that animal sound!")
    }
}

fn give_description<T>(can_be_described: T) -> String
where T:Describable {
    can_be_described.describe()
}
enum Food {
    Chicken,
    Beef,
    Vegetables,
    Fish,
    Bread
}

trait Eats {
    fn eat(&mut self, food: Food);
}

//trait Pettable {
//    fn be_pet_by(&mut self, friend: &Person) -> Result<i8, Error>;
//}
struct Dog;
struct Cat;

impl Describable for Dog {
    fn describe(&self)-> String {
        String::from("Im a dog!")
    }
}

impl Eats for Cat {
    fn eat(&mut self, food: Food) {
        // .... interact with canned food on a plate
    }
}

impl Describable for Cat {}

trait Describable {
    fn describe(&self)-> String {
        String::from("I'm a self-describing object!")
    }
}


enum Food{
    Chicken,
    Beef,
    Fish,
}
trait Eats {
    fn eat(&self, food: Food);
}
struct Dog;

impl Eats for Dog {
    fn eat(&self) {

    }
}
struct Cat;
struct Marker;
struct RGB(i8, i8, i8);

*/

/* fn i_eat_memory<G>(food: G) {}

fn main() {
    {
        let var = String::from("Hello!");
        i_eat_memory(var);
        println!("{}", var);
    };

    {
        let var2 = String::from("Hello!");
        let i_eat_memory_closure = |food: &String| {
            println!("{}", var2);
        };
        i_eat_memory_closure(&var2);
        println!("{}", var2);
    };
    {
        let f = || {};
        assert_eq!(f(), ());
    };
    {
        //let p : ! = panic!("This is a panic!");
    };
    {
        let add_one = Box::new(|x: i32| -> i32 { x + 1 });
        i_eat_memory(add_one);
        let f: Box<dyn Fn(i32) -> i32> = add_one;
        assert_eq!(add_one(2), 3);
    };
    {
        assert_eq!(call_input_fn(|x: i32| x + 1), 2);
    };
}


struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn new(x: i32, y: i32, z: i32) -> Point {
        Point { x, y, z }
    }
}

impl Point {
    fn new_with_pattern((x,y,z):(i32,i32,i32)) -> Point {
        Point { x, y, z }
    }

    fn show_x(&self) -> i32 {
        self.x
    }
}

fn call_input_fn<F>(f: F) -> i32
    where F: Fn(i32) -> i32
{
    f(1)
}

fn call_input_fn_no_return<F>(f: F)
    where F: Fn(i32) -> i32
{
    f(1);
} */
