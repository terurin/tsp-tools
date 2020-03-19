use rand::distributions::Distribution;
use rand::RngCore;
use std::io::{Error, Read, Write};
#[derive(Debug, Clone)]
pub struct Location {
    pub number: usize,
    pub rank: usize,
    pub positions: Vec<Vec<f32>>,
}

impl Location {
    pub fn read<R: Read>(reader: &mut R) -> Result<Location, Error> {
        let mut text = String::new();
        reader.read_to_string(&mut text)?;
        let mut lines = text.trim().split('\n');
        //header
        let (number, rank) = {
            let header = lines.next().unwrap();
            let mut iter = header.split(' ').map(|s: &str| s.parse::<usize>().unwrap());
            (iter.next().unwrap(), iter.next().unwrap())
        };
        //body
        let positions: Vec<Vec<f32>> = lines
            .map(|s: &str| {
                s.trim()
                    .split(' ')
                    .map(|s: &str| s.parse::<f32>().unwrap())
                    .collect()
            })
            .collect();
        Ok(Location {
            rank: rank,
            number: number,
            positions: positions,
        })
    }

    pub fn distance(&self, turns: &[usize]) -> f32 {
        //距離の確認
        let p = turns.iter().map(|&index| &self.positions[index][..]);
        let q = turns
            .iter()
            .cycle()
            .skip(1)
            .map(|&index| &self.positions[index][..]);
        p.zip(q)
            .map(|(x, y): (&[f32], &[f32])| -> f32 {
                x.iter()
                    .zip(y.iter())
                    .map(|(&x, &y)| (x - y).powi(2))
                    .fold(0.0, |acc, a| acc + a)
                    .sqrt()
            })
            .sum()
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        writeln!(writer, "{} {}", self.number, self.rank)?;
        let text = (0..self.number)
            .into_iter()
            .map(|index: usize| {
                (0..self.rank)
                    .into_iter()
                    .map(|j| self.positions[index][j])
                    .fold(String::new(), |acc, a| format!("{} {}", acc, a))
            })
            .fold(String::new(), |acc, a| format!("{}\n{}", acc, a));
        write!(writer, "{}", text)?;
        Ok(())
    }

    pub fn new<R: RngCore + ?Sized>(
        random: &mut R,
        number: usize,
        rank: usize,
        scale: f32,
    ) -> Location {
        let uniform = rand::distributions::Uniform::new(-scale.abs(), scale.abs());

        let positions: Vec<Vec<_>> = (0..number)
            .into_iter()
            .map(|_| {
                (0..rank)
                    .into_iter()
                    .map(|_| uniform.sample(random))
                    .collect()
            })
            .collect();

        Location {
            positions: positions,
            number: number,
            rank: rank,
        }
    }
}
