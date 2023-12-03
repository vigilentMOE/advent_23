use day01_05_rust::day0_0::zero_entry;

fn main(){
    match zero_entry() {
        Ok(_) => println!("File read successfully"),
        Err(e) => eprintln!("Error reading file: {}", e),
    }}