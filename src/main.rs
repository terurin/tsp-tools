#[macro_use]
extern crate clap;

use rand::RngCore;
use rand::SeedableRng;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::{stdin, stdout, BufReader, BufWriter, Error, Read, Write};
mod cli;
mod location;

fn main() {
    let matches = cli::build_cli().get_matches();

    let parse_u32 = |x: Option<&str>| x.map(|s: &str| s.parse::<u32>().unwrap()).unwrap();
    let parse_f32 = |x: Option<&str>| x.map(|s: &str| s.parse::<f32>().unwrap()).unwrap();
    //生成
    if let Some(g) = matches.subcommand_matches("generate") {
        //引数
        let filename = g.value_of("output");
        let rank = parse_u32(g.value_of("rank"));
        let number = parse_u32(g.value_of("number"));
        let scale = parse_f32(g.value_of("scale"));

        //出力先準備
        let mut writer: Box<dyn Write> = if let Some(filename) = filename {
            Box::new(BufWriter::new(File::create(filename).unwrap()))
        } else {
            Box::new(stdout())
        };

        let mut random: Box<dyn RngCore> =
            if let Some(seed) = g.value_of("seed").map(|s: &str| s.parse().unwrap()) {
                Box::new(rand_xoshiro::Xoshiro256StarStar::seed_from_u64(seed))
            } else {
                Box::new(rand::thread_rng())
            };
        let location = location::Location::new(&mut random, number as usize, rank as usize, scale);
        location.write(&mut writer).unwrap();
    }

    //demo
    if let Some(d) = matches.subcommand_matches("demo") {
        //入力受付
        let input = d.value_of("location");
        let output = d.value_of("output");
        let mode = d.value_of("mode").unwrap();
        //出力先
        let mut writer: Box<dyn Write> = if let Some(filename) = output {
            Box::new(BufWriter::new(File::create(filename).unwrap()))
        } else {
            Box::new(stdout())
        };
        //入力元
        let mut reader: Box<dyn Read> = if let Some(filename) = input {
            Box::new(BufReader::new(File::open(filename).unwrap()))
        } else {
            Box::new(stdin())
        };

        demo(&mut writer, &mut reader, mode).unwrap();
    }

    //check
    if let Some(c) = matches.subcommand_matches("check") {
        //入力受付
        let location_name = c.value_of("location").unwrap();

        //入力元
        let mut turn: Box<dyn Read> = if let Some(path) = c.value_of("path") {
            Box::new(BufReader::new(File::open(path).unwrap()))
        } else {
            println!("a");
            Box::new(stdin())
        };
        let mut location = BufReader::new(File::open(location_name).unwrap());
        check(&mut turn, &mut location).unwrap();
    }
}

fn demo<W: Write, R: Read>(writer: &mut W, reader: &mut R, _mode: &str) -> Result<(), Error> {
    let mut text = String::new();
    reader.read_to_string(&mut text)?;
    let iter = text.chars();
    let number: u32 = iter
        .take_while(|&c| '0' <= c && c <= '9')
        .collect::<String>()
        .parse()
        .unwrap();
    //真面目に解く必要はない
    for i in 0..number - 1 {
        writeln!(writer, "{}", i)?;
    }
    writeln!(writer, "{}", number - 1)?;
    Ok(())
}
//basic? basicじゃないよ
fn iif<T>(flag: bool, a: T, b: T) -> T {
    if flag {
        a
    } else {
        b
    }
}

fn check<R1: Read, R2: Read>(path_reader: &mut R1, location_reader: &mut R2) -> Result<(), Error> {
    let mut turn_text = String::new();
    path_reader.read_to_string(&mut turn_text)?;
    let path: Vec<usize> = turn_text
        .trim()
        .split('\n')
        .map(|s: &str| s.parse().unwrap())
        .collect();
    //location
    let location = location::Location::read(location_reader).unwrap();
    //整合性チェック(すべてのノードが使われているか)
    let mut flags: BTreeSet<usize> = (0..location.number).into_iter().collect();
    for turn in path.iter() {
        flags.remove(turn);
    }

    println!("path test:{}", iif(flags.is_empty(), "good", "bad"));

    println!("distance :{}", location.distance(&path));
    Ok(())
}
