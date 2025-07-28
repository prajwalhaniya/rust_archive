use std::fs::File;
use std::io::Error;
use std::rc::Rc;
use std::sync::Arc;
use std::thread;
use std::cell::RefCell;

// Unused variables

fn unused_variables() {
    let x = 5;

    // for unused variables use _
    let _y = &x;

    println!("x: {}", x);
}

/**
    Why to use lifetimes?
        The lifetimes are important because they make sure that the borrowed 
        data outlives its usage.
        
        - They ensure borrowed references are valid when used
        - They prevent use-after-free errors
        - They're compile time checks

    --

    What are use-after-free errors?
        The use-after-free errors happen when you try to access memory that has already been freed/deallocated.
        
        Real-world analogy:
            1. You rent an apartment (allocate memory)
            2. You give your friend the address (create a pointer/reference)
            3. You move out and the landlord rents it to someone else (memory gets freed/reused)
            4. Your friend shows up with your address expecting to find you (use-after-free)
            5. Instead they find strangers or an empty room (garbage data or crash)

    Rust cataches the lifetime errors at the compile time. To prevent the crash in producation.

*/
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn understanding_lifetimes() {
    /**
        Cannot be modified, for modification use String::from (you will own the data on heap)
        More expensive to use String::from, this is mutable.

        let string1 = "Hello" 
        -> Here string1 is borrowed, it cannot be modified, it is cheap
    */
    let string1 = "Helloo"; 
    let string2 = "World";

    let result = longest(string1, string2);
    println!("The longest string is {}", result);

}


/**
    Traits are similar to interfaces. 
    It defines a set of methods that types can implement.

*/

trait Drawable {
    fn draw(&self);
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a circle with radius {}", self.radius);
    }

    fn area(&self) -> f64 {
        3.142 * self.radius * self.radius
    }
}

impl Drawable for Rectangle {
    fn draw(&self) {
        println!("Drawing a rectangle with width {} and height {}", self.width, self.height);
    }

    fn area(&self) -> f64 {
        self.width * self.height
    }
}

fn understanding_traits() {
    let circle = Circle { radius: 5.0 };
    circle.draw();
    println!("Area: {:.2}", circle.area());

    let rect = Rectangle { width: 10.0, height: 5.0 };
    rect.draw();
    println!("Area: {:.2}", rect.area());
}

/**
    Pattern matching in rust allows you destructure data
*/

enum Direction {
    North,
    South,
    East,
    West
}

fn match_direction(dir: Direction) {
    match dir {
        Direction::North => println!("Moving north"),
        Direction::South => println!("Moving south"),
        Direction::East => println!("Moving east"),
        Direction::West => println!("Moving west"),
    }
}

fn understanding_pattern_matching() {
    let dir = Direction::West;
    match_direction(dir);
}

/**
    Error handling
    
    Rust uses two main types to represents operations that may fail
    Result<T, E> and Option<T>.

    Option<T> Represents a value that might or might not exisit.
    Some(T) - Contains a value of type T
    None - represents the absense of value
*/

fn find_user(id: u32) -> Option<String> {
    if id == 1 {
        Some("Arjun".to_string())
    } else {
        None
    }
}

fn understanding_option() {
    match find_user(1) {
        Some(user) => println!("Found user: {}", user),
        None => println!("User not found"),
    }
}

fn open_file() -> Result<File, Error> {
    File::open("data.txt")
}

fn understanding_result() {
    match open_file() {
        Ok(_file) => println!("File opened successfully"),
        Err(e) => println!("Error opening file: {}", e),
    }
}

fn understanding_error_handling() {
    understanding_option();
    understanding_result();
}

/**
    Smart pointers

    Smart pointers are data structres that act like traditional pointers but provide
    additional metadata and capabilities.

    They own the data that they point to and can control how that data is accessed, shared
    or cleaned up.

    They implement: Dref and Drop traits.

    Deref allows them to be used like regular references & Drop provides custom cleanup logic when
    they go out of scope.

    The smart pointers are usually used as a combination of multiple smart pointers to
    achieve specific ownership patterns.
*/

fn implement_box_t() {
    let boxed_int = Box::new(54);
    println!("Boxed integer: {}", boxed_int);

    // useful for the large data that would overflow in a stack.
    let large_array = Box::new([0; 1000000]);

    // enable recursive data structures
    #[derive(Debug)]
    enum List {
        Node(i32, Box<List>),
        Nil
    }

    let list = List::Node(1, Box::new(List::Nil));

    println!("List: {:?}", list);
}

/**
    Rc<T> allows mutliple owners of the same data. 
    It keeps track of how many references exisit and cleans up
    when the count reaches zero. It only works on single threaded
    context.

*/
fn implement_Rc_t() {
    let data = Rc::new(String::from("Hello"));

    let ref1 = Rc::clone(&data);
    let ref2 = Rc::clone(&data);

    println!("Ref count: {}", Rc::strong_count(&data));
    println!("Data: {}", data);
    println!("Ref1: {}", ref1);
    println!("Ref2: {}", ref2);
}

/**
    Arc<T> is thread safe version of Rc<T>. It uses atomic operations 
    to manage the reference count, making it safe to share between threads.
*/
fn implement_Arc_t() {
    let data = Arc::new(String::from("Hello"));
    let mut handles = vec![];
    let mut count = 0;

    for i in 0..3 {
        count += 1;
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            println!("Thread {} has data: {}", i, data_clone);
            println!("Count: {}", count);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

/**
    Refcell<T> provides interior mutability - the ability to mutate data 
    even when there are immutable references to it.

    It enforces borrowing rules at runtime instead of compile time.

*/
fn implement_RefCell_t() {
    let data = RefCell::new(vec![1, 2, 3]);

    data.borrow_mut().push(4);
    println!("Data: {:?}", data.borrow());
}

fn understanding_smart_pointers() {
    implement_box_t();
    implement_Rc_t();
    implement_Arc_t();
    implement_RefCell_t();
}

fn main() {
    unused_variables();
    understanding_lifetimes();
    understanding_traits();
    understanding_pattern_matching();
    understanding_error_handling();
    understanding_smart_pointers();
}