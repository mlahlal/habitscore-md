use std::fs;

fn main() {
    let contents = fs::read_to_string("/path/to/file")
        .expect("Should have been able to read the file");
    let contents: Vec<&str> = contents.split('\n').collect();
    let mut lines = vec![];
    let mut day = String::new();
    let mut tasks: bool = false;
    let mut points: i32 = 0;
    let lvl;

    for line in &contents {
        let tmp: String = line.to_string();

        if tasks && tmp.chars().count() >= 2 && &tmp[..3] == "- [" {
            let tmp_pt = points;
            match &tmp[6..7] {
                "H" => {
                    points = points + ( if &tmp[3..4] == "X" {10} else {-15} );
                },
                "M" => {
                    points = points + ( if &tmp[3..4] == "X" {5} else {-3} );
                },
                "L" => {
                    points = points + ( if &tmp[3..4] == "X" {1} else {0} );
                },
                _ => println!("Stop The Cap"),
            }
            if tmp_pt < points { String::clear(&mut day); }
            continue;
        } else {
            if day.len() > 0 && tasks { points = points - 10; }
            tasks = false;
        }

        if tmp.chars().count() >= 6 && &tmp[..6] == "Punti:" {
            let tmp_pt = tmp.chars().skip_while(|c| !c.is_digit(10) && *c != '-').collect::<String>();
            let tmp_pt = tmp_pt.parse::<i32>().unwrap();
            points = points + tmp_pt;
        }

        if tmp.chars().count() >= 4 && &tmp[0..4] == "####" {
            tasks = true;
            day = tmp.clone();
        }

        lines.push(format!("..{}..", tmp));
    }

    if points < 0 { lvl = -1; }
    else { lvl = points/1000; }

    println!("points => {} \nlevel => {}", points, lvl);
}
