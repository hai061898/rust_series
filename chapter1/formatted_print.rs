// format!: write formatted text to String
// print!: same as format! but the text is printed to the console (io::stdout).
// println!: same as print! but a newline is appended.
// eprint!: same as format! but the text is printed to the standard error (io::stderr).
// eprintln!: same as eprint!but a newline is appended.

fn main() {

    println!("{} days", 31);

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

   
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    println!("{number:>width$}", number=1, width=6);

    println!("{number:0>width$}", number=1, width=6);

    println!("My name is {0}, {1} {0}", "Bond");
 
    #[allow(dead_code)]
    struct Structure(i32);

    println!("This struct `{}` won't print...", Structure(3));
}