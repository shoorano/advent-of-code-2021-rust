use std::io;

fn get_input() -> io::Result<String> {
  let mut buffer = String::new();
  io::stdin().read_line(&mut buffer)?;
  Ok(buffer.trim().to_owned())
}

fn main() {
  let mut all_input = vec![];
  let mut input_count = 0;
  while input_count < 2 {
    match get_input() {
      Ok(words) => {
        all_input.push(words);
        input_count += 1;
      },
      Err(error) => println!("error: {:?}", error),
    }
  }

  for input in all_input {
    println!(
      "Original: {:?}, capitalised: {:?}",
      input,
      input.to_uppercase()
    );
  }
}