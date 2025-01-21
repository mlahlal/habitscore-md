use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filepath = &args[1];

    if filepath.len() <= 0 {
        return;
    }

    let contents = fs::read_to_string(filepath)
        .expect("Should have been able to read the file");
    let contents: Vec<&str> = contents.split('\n').collect();
    let mut lines = vec![];
    let mut day = "";
    let mut tasks: bool = false;
    let mut points: i32 = 0;
    let lvl;

    for line in &contents {
        let tmp = line;

        if tasks && tmp.len() >= 2 && &tmp[..3] == "- [" {
            let tmp_pt = points;
            let done = &tmp[3..4];
            let priority = &tmp[6..7];

            points += match (done, priority) {
                ("X", "H") => 10,
                (_, "H") => -15,
                ("X", "M") => 5,
                (_, "M") => -3,
                ("X", "L") => 1,
                (_, "L") => 0,
                _ => {
                    println!("Invalid priority");
                    0
                }
            };

            if tmp_pt < points { day = ""; }

            continue;
        } else if day.len() > 0 && tasks {
            points -= 10;
            tasks = false;
        }

        if tmp.len() >= 6 && &tmp[..6] == "Punti:" {
            let tmp_pt = tmp.chars().skip_while(|c| !c.is_digit(10) && *c != '-').collect::<String>();
            let tmp_pt = tmp_pt.parse::<i32>().unwrap();
            points = points + tmp_pt;
        }

        if tmp.len() >= 4 && &tmp[0..4] == "####" {
            tasks = true;
            day = tmp;
        }

        lines.push(format!("..{}..", tmp));
    }

    if points < 0 { lvl = -1; }
    else { lvl = points/1000; }

    println!("points => {} \nlevel => {}", points, lvl);
}
