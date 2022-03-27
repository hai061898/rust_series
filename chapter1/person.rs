#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    let name = "Hai";
    let age = 24;
    let peter = Person { name, age };

    // Pretty print
    println!("{:#?}", peter);
}

// the 'a reads ‘the lifetime a’. Technically, every reference has some lifetime associated with it, but the compiler lets you elide (i.e. omit, see "Lifetime Elision") them in common cases.
// A function can have ‘generic parameters’ between the <>s, of which lifetimes are one kind. The <> is used to declare lifetimes. This says that bar has one lifetime, 'a.

// Rust has two main types of strings: &str and String. The &str are called ‘string slices’. A string slice has a fixed size, and cannot be mutated. It is a reference to a sequence of UTF-8 bytes.