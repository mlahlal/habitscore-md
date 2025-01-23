use std::{fs, path::Path};
use gumdrop::Options;

#[derive(Debug, Options)]
struct MyOptions {
    #[options(free)]
    free: Vec<String>,

    #[options(help = "print help message")]
    help: bool,

    #[options(help = "show the tasks not done many times")]
    stat: bool,
}

fn main() {
    let opts = MyOptions::parse_args_default_or_exit();

    if opts.free.len() > 1 {
        println!("Only one path is accepted");
        return;
    }

    if opts.free.len() <1 {
        println!("Path not provided");
        return;
    }

    let filepath = &opts.free[0];
    let stat = opts.stat;

    if !Path::new(filepath).exists() {
        println!("Invalid path");
        return;
    }

    let contents = fs::read_to_string(filepath)
        .expect("Should have been able to read the file");
    let contents: Vec<&str> = contents.split('\n').collect();
    let mut lines = vec![];
    let mut not_done: Vec<&str> = Vec::new();
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

            if stat && done == " " { not_done.push(&tmp[10..]); }

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

    if stat {
        not_done.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

        struct NotDoneTask<'a> {
            task: &'a str,
            count: i32,
        }

        let mut stats: Vec<NotDoneTask> = Vec::new();

        for el in &not_done {
            let mut count = 0;
            for le in &not_done {
                if le == el { count += 1; }
            }
            if !stats.iter().any(|i| i.task == *el) {
                stats.push(NotDoneTask { task: el, count });
            } else {
                let index = stats.iter().position(|r| r.task == *el).unwrap();
                stats[index].count = count;
            }
        }

        println!("\n\nTask not done many times (>2)\n");
        for el in &stats {
            if el.count >= 2 {
                println!("Task: {} \nCount: {}\n", el.task, el.count);
            }
        }
    }
}
