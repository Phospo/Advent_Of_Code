use std::fs;
use std::vec;

pub struct Elv {
    name: u32,
    his_calories: u32,
}
fn main() {
    // Get the content of the file.
    let content = fs::read_to_string("./input.txt").unwrap();
    // Split the content into lines, empty lines will be used later (it's not a bug, it's a feature).
    // Also, input file must end with empty line, otherwise second puzzle may break depending on input.
    let elves_list = content.split('\n');

    let mut elv_index: u32 = 1;
    let mut calories: u32 = 0;

    let mut elves = vec![];

    for calories_line in elves_list {
        if !calories_line.is_empty() {
            let cal_tmp: u32 = calories_line.parse().unwrap();
            calories += cal_tmp;
        }

        if calories_line.is_empty(){
            elves.push(Elv {
                name: elv_index,
                his_calories: calories,
            });
            elv_index += 1;
            calories = 0;
        }
    }

    // for x in &elves{
    //     println!("Elf name: {}, Calories: {}", x.name ,x.his_calories)
    // }

    let mut big_elv: Elv = Elv {
        name: 0,
        his_calories: 0,
    };

    for elv in &elves {
        if elv.his_calories > big_elv.his_calories {
            big_elv.name = elv.name;
            big_elv.his_calories = elv.his_calories;
        }
    }

    println!(
        "Big elf calories: {}\nHis Name: {}",
        big_elv.his_calories, big_elv.name
    );

    elves.sort_by_key(|elf| std::cmp::Reverse(elf.his_calories));

    // for x in &elves {
    //     println!("Elf name: {}, Calories: {}", x.name, x.his_calories)
    // }

    let top_elves_calories = {
        let mut ret: u32 = 0;
        (0..3).for_each(|x| {
            ret += elves[x].his_calories;
        });
        ret
    };

    println!("3 top G: {}", top_elves_calories)
}
