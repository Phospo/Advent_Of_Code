use std::{fs};
fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    first(&input);
    second(&input);
}

fn first(input: &str){
    let output = input.lines();
    
}

fn second(input: &str){

}
