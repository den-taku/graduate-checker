use std::fs::File;
use std::io::prelude::*;

fn calc(mut values: std::str::SplitWhitespace, buf: &mut String) {
    if values.next() == Some("0") {
        // oom
        buf.push_str("oom\n");
        return;
    }
    if values.next() == Some("0") {
        // time limit
        buf.push_str("time\n");
        return;
    }
    values.next();
    if values.next() == Some("0") {
        // no feasible answer
        buf.push_str("x\n");
    } else if let Some(relocations) = values.next() {
        buf.push_str(&format!("{}\n", relocations));
    }
}

// run `cargo run --release DIRECTORY CASES`
fn main() -> std::io::Result<()> {
    println!("he");
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("invalid number of arguments: {}", args.len());
        println!("{:?}", args);
        std::process::exit(1);
    }
    let size = args[2].parse::<usize>().unwrap();

    let mut buf = String::new();
    buf.push_str(&format!("{} cases\n", size));

    for i in 0..size {
        let path = format!("{}/result{}", &args[1], i);
        let mut file = File::open(&path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let values: std::str::SplitWhitespace = contents.split_whitespace();
        for e in values.clone() {
            println!("{:?}", e);
        }
        calc(values, &mut buf);
    }

    let mut file = File::create(format!("{}/check", args[1]))?;
    file.write_all(buf.as_bytes())
}
