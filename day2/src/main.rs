use std::fs;

fn main(){
    let content = fs::read_to_string("./input.txt").unwrap();
    println!("First: {}", first(&content));
    println!("Second: {}", second(&content));
}

fn first(content: &str) -> u32 {
    let combo_list = content.split('\n');
    let mut points: u32 = 0;

    for x in combo_list{
        if x.starts_with('A'){  // Rock 1
            if x.chars().nth(2).unwrap() == 'X'{ // Rock 1
                points += 3 + 1;
            }
            if x.chars().nth(2).unwrap() == 'Y'{ // Paper 2
                points += 6 + 2;
            }
            if x.chars().nth(2).unwrap() == 'Z'{ // Scissors 3
                points += 3;
            }
        }
        if x.starts_with('B'){  // Paper 2
            if x.chars().nth(2).unwrap() == 'X'{ // Rock 1
                points += 1;
            }
            if x.chars().nth(2).unwrap() == 'Y'{ // Paper 2
                points += 3 + 2;
            }
            if x.chars().nth(2).unwrap() == 'Z'{ // Scissors 3
                points += 6 + 3;
            }
        }
        if x.starts_with('C'){  // Scissors 3
            if x.chars().nth(2).unwrap() == 'X'{ // Rock 1
                points += 6 + 1;
            }
            if x.chars().nth(2).unwrap() == 'Y'{ // Paper 2
                points += 2;
            }
            if x.chars().nth(2).unwrap() == 'Z'{ // Scissors 3
                points += 3 + 3;
            }
        }
    }
    points
}

fn second(content: &str) -> u32 {
    let mut points: u32 = 0;
    let combo_list = content.split('\n');

    for x in combo_list{
        if x.starts_with('A'){  // Rock 1
            if x.chars().nth(2).unwrap() == 'X'{ // Scissors
                points += 3;
            }
            if x.chars().nth(2).unwrap() == 'Y'{ // Rock
                points += 3 + 1;
            }
            if x.chars().nth(2).unwrap() == 'Z'{ // Paper
                points += 6 + 2;
            }
        }
        if x.starts_with('B'){  // Paper 2
            if x.chars().nth(2).unwrap() == 'X'{ // Rock
                points += 1;
            }
            if x.chars().nth(2).unwrap() == 'Y'{ // Paper
                points += 3 + 2;
            }
            if x.chars().nth(2).unwrap() == 'Z'{ // Scissor
                points += 6 + 3;
            }
        }
        if x.starts_with('C'){  // Scissors 3
            if x.chars().nth(2).unwrap() == 'X'{ // Paper
                points += 2;
            }
            if x.chars().nth(2).unwrap() == 'Y'{ // Scissors
                points += 3 + 3;
            }
            if x.chars().nth(2).unwrap() == 'Z'{ // Rock
                points += 6 + 1;
            }
        }
    }

    points
}