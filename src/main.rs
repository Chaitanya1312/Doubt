#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_parens)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_mut)]

mod builder;
mod compiler;
mod language;
mod parser;
mod readback;
mod rulebook;
mod runtime;

fn main() -> std::io::Result<()> {
  //run_cli()?;
  run_example()?;
  return Ok(());
}

fn run_cli() -> std::io::Result<()> {
  let mut args: Vec<String> = std::env::args().collect();

  if args.len() <= 1 {
    show_help();
    return Ok(());
  }

  let cmd = &args[1];

  if cmd == "run" && args.len() == 3 {
    let file = &args[2];
    return run_code(&load_file_code(&file));
  }

  if cmd == "compile" && args.len() == 3 {
    let file = &args[2];
    return compile_code(&load_file_code(&file), &file);
  }

  println!("Invalid arguments: {:?}.", args);
  return Ok(());
}

fn show_help() {
  println!("High-Order Virtual Machine ({})", env!("CARGO_PKG_VERSION"));
  println!("==========================");
  println!("");
  println!("To run a file, interpreted:");
  println!("");
  println!("  hovm run file.hovm");
  println!("");
  println!("To compile a file to C:");
  println!("");
  println!("  hovm compile file.hovm");
  println!("");
  println!("More info: https://github.com/kindelia/hovm");
  println!("");
}

fn run_code(code: &str) -> std::io::Result<()> {
  println!("Reducing.");
  let (norm, cost, size, time) = builder::eval_code("Main", code);
  println!("Rewrites: {} ({:.2} MR/s)", cost, (cost as f64) / (time as f64) / 1000.0);
  println!("Mem.Size: {}", size);
  println!("");
  println!("{}", norm);
  return Ok(());
}

fn compile_code(code: &str, name: &str) -> std::io::Result<()> {
  let name = format!("{}.out.c", name);
  compiler::compile_code_and_save(code, &name)?;
  println!("Compiled to '{}'.", name);
  return Ok(());
}

fn load_file_code(file_name: &str) -> String {
  return std::fs::read_to_string(file_name)
    .expect(&format!("Error reading file: '{}'.", file_name));
}

fn run_example() -> std::io::Result<()> {
  // Source code
  let code = "(Main) = (λf λx (f (f x)) λf λx (f (f x)))";

  let code = "
    (Fn 0) = 1
    (Fn n) = (+ (Fn (- n 1)) (Fn (- n 1)))
    (Main) = (Fn 20)
  ";

  let code = "
    // Applies a function to all elements in a list
    (Map fn (Nil))            = (Nil)
    (Map fn (Cons head tail)) = (Cons (fn head) (Map fn tail))

    // Increments all numbers on [1,2,3]
    (Main) = (Map λx(+ x 1) (Cons 1 (Cons 2 (Cons 3 (Nil)))))
  ";

  let code = "
    (Filter fn (Cons x xs)) = (Filter_Cons (fn x) fn x xs)
      (Filter_Cons 1 fn x xs) = (Cons x (Filter fn xs))
      (Filter_Cons 0 fn x xs) = (Filter fn xs)
    (Filter fn (Nil)) = (Nil)

    (Concat (Nil) b)        = b
    (Concat (Cons ah at) b) = (Cons ah (Concat at b))

    (Quicksort (Nil)) = (Nil)
    (Quicksort (Cons h t)) =
      let min = (Filter λx(< x h) t)
      let max = (Filter λx(> x h) t)
      (Concat (Quicksort min) (Cons h (Quicksort max)))

    (Main) = (Quicksort (Cons 3 (Cons 1 (Cons 2 (Cons 4 (Nil))))))
  ";

  let code = "
    (Foo arg) = λx (arg (λx x) x)
    (Main)    = 42
  ";

  let code = "
    (Main) = (λx(x 4 5) λa λb b)
  ";

  let code = "
    (Main) =
      (
        λxs λnil λcons (xs nil λx λxs (cons x xs))
        λn λc (c 0 (c 0 n))
      )
  ";

  // Compiles to C and saves as 'main.c'
  compiler::compile_code_and_save(code, "main.c")?;
  println!("Compiled to 'main.c'.");

  // Evaluates with interpreter
  println!("Reducing with interpreter.");
  let (norm, cost, size, time) = builder::eval_code("Main", code);
  println!("Rewrites: {} ({:.2} MR/s)", cost, (cost as f64) / (time as f64) / 1000.0);
  println!("Mem.Size: {}", size);
  println!("");
  println!("{}", norm);
  println!("");

  return Ok(());
}
