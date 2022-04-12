use std::{
    collections::HashMap,
    ops::{Add, Range},
    io::{Error, ErrorKind},
    sync::mpsc,
    cmp::{Ord, PartialOrd, Ordering},
};
use rug::{
    Integer, Float,
};
use crate::dice::DiceSet;

#[derive(Debug,Eq,PartialEq)]
pub struct Distribution {
    dist: HashMap<usize, Integer>,
    min: usize,
    max: usize,
    size: usize,
    n: usize,
    pos: usize,
}

impl Distribution {
    fn new(dist: HashMap<usize, Integer>, min: usize, max: usize, size: usize, n: usize) -> Self {
        Distribution { dist, min, max, size, n, pos: min }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn probs(&self) -> Vec<Float> {
        let total: Integer = self.dist.values().sum();
        self.dist.values().map(move |count| (count + Float::with_val(10, 0.0f32)) / &total).collect()
    }

    fn get(&self, ind: usize) -> Option<&Integer> {
        self.dist.get(&ind)
    }

    pub fn into_string(self) -> String {
        let mut s = format!("min: {}, max: {}, size: {}, n: {}", self.min, self.max, self.size, self.n);
        let r = self.min..=self.max;
        self.zip(r).for_each(|(n, pos)| s.push_str(&format!("\n{}: {}", pos, n)));
        // self.probs().into_iter().map(|f| f.to_string()).collect::<Vec<_>>().join(" "),
        s
    }
}

impl TryFrom<DiceSet> for Distribution {
    type Error = Error;
    fn try_from(dice: DiceSet) -> Result<Self, Self::Error> {

        match dice.len() {
            0 => Ok(Distribution::new(HashMap::new(), 0, 0, 0, 0)),
            1 => {
                let size = dice.get(0).unwrap().len();
                let mut dist = HashMap::with_capacity(size);
                (1..=size).for_each(|n| {dist.insert(n, Integer::from(1u8));});
                Ok(Distribution::new(dist, 1, size, size, 1))
            },
            _ => {
                // Split the set nicely into two
                let partition = int_log(dice.len());
                let (set1, set2) = dice.partition(partition);

                //
                let (tx, rx) = mpsc::channel();
                let handle = std::thread::spawn(move || { tx.send(Self::try_from(set2)).expect("something happened over here"); } );
                let dist1 = Self::try_from(set1)?;
                let dist2 = rx.recv().map_err(|_e| Error::new(ErrorKind::BrokenPipe, "something went wrong"))??;
                handle.join().map_err(|_e| Error::new(ErrorKind::BrokenPipe, "something different went wrong"))?;

                Ok(dist1 + dist2)
            }
        }
    }
}

impl Distribution {
}

// fn ncr(n: usize, r: usize) -> Integer {

//     let mut a = [r, (n-r)];
//     a.sort();
//     let [smaller, larger] = a;

//     factorial(1+larger, n) / factorial(1,smaller)
// }

// fn factorial(start: usize, stop: usize) -> Integer {
//     let mut product = Integer::from(1usize);
//     (start..=stop).for_each(|n| {product *= Integer::from(n);});
//     product
// }

fn int_log(m: usize) -> usize {
    let mut current = 1;
    let mut next = 2;
    while next < m {
        current = next;
        next *= 2;
    }
    current
}

impl PartialOrd for Distribution {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.size.partial_cmp(&other.size)
    }
}

impl Ord for Distribution {
    fn cmp(&self, other: &Self) -> Ordering {
        self.size.cmp(&other.size)
    }
}

impl Add for Distribution {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {

        let min = self.min + other.min;
        let max = self.max + other.max;
        let size = max - min + 1;
        let n = self.n + other.n;
        let halfway = size / 2 + size % 2;

        let mut arr = [self, other];
        arr.sort();
        let [smaller, larger] = arr;

        let mut dist: HashMap<usize, Integer> = HashMap::with_capacity(size);
        for i in 0..size {
            if i >= halfway {
                dist.insert(
                    i + min,
                    dist.get(&(max - i)).unwrap().clone()
                );
            } else {
                let r: Range<usize> = if i <= smaller.size {
                    0..(i+1)
                } else {
                    0..smaller.size
                };
                dist.insert(
                    min + i,
                    r.map(|j| smaller
                        .get(smaller.min+j)
                        .unwrap_or(&Integer::from(0u8)).clone() * larger
                        .get(min+i-smaller.min-j)
                        .unwrap_or(&Integer::from(0u8)).clone()
                    ).sum::<Integer>() //* ncr(n, smaller.n)
                );
            }
        }
        Distribution::new(dist, min, max, size, n)
    }
}

impl Iterator for Distribution {
    type Item = Integer;
    fn next(&mut self) -> Option<Self::Item> {
        let out = self.dist.get(&self.pos);
        self.pos += 1;
        out.map(|val| val.clone())
    }
}

// impl IntoIterator for Distribution {
//     type Item = Integer;
//     type IntoIter = DistributionIter;
//     fn into_iter(self) -> Self::IntoIter {
//         DistributionIter { dist: self.dist, pos: self.min }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dice::DiceSet;

    #[test]
    fn two_d6() {
        let dice: DiceSet = "2d6".parse().unwrap();
        println!("{:?}", Distribution::try_from(dice).unwrap().into_string());
    }

    #[test]
    fn test_ncr() {
        
    }
}