use rand::{Rng, thread_rng};
use crate::roll::{Roll, Outcome};
use std::{
    io::{Error as IoError, ErrorKind},
    str::FromStr
};

#[derive(Eq,PartialEq,Debug,Ord,PartialOrd,Copy,Clone)]
pub struct Die { pub n: u8 }

impl Die {
    pub fn new(n: u8) -> Self {
        Die { n }
    }

    pub fn len(&self) -> usize {
        self.n as usize
    }
}

impl From<Die> for u8 {
    fn from(d: Die) -> Self {
        d.n
    }
}


impl From<&Die> for u8 {
    fn from(d: &Die) -> Self {
        d.n
    }
}

impl From<u8> for Die {
    fn from(d: u8) -> Self {
        Die::new(d)
    }
}


impl ToString for Die {
    fn to_string(&self) -> String {
        format!("d{}", self.n)
    }
}

impl FromStr for Die {
    type Err = IoError;
    fn from_str(s: &str) -> Result<Self,Self::Err> {
        s[(s.starts_with('d') as usize)..].parse::<u8>()
            .map(Die::new)
            .map_err(|_| IoError::new(ErrorKind::InvalidData, "Number of sides must be an integer"))
    }
}


#[derive(Eq,PartialEq,Debug)]
pub struct DiceSet {
    dice: Vec<Die>
}

impl DiceSet {
    pub fn new(mut dice: Vec<Die>) -> Self {
        dice.sort();
        DiceSet { dice }
    }

    pub fn from_die(d: Die) -> Self {
        Self::new(vec![d])
    }

    pub fn from_u8(n: u8) -> Self {
        Self::new(vec![Die::new(n)])
    }

    pub fn len(&self) -> usize {
        self.dice.len()
    }

    pub fn into_inner(self) -> Vec<Die> {
        self.dice
    }
}

impl DiceSet 
{
    pub fn get(&self, ind: usize) -> Option<&Die>
    {
        self.dice.get(ind)
    }

    pub fn partition(mut self, ind: usize) -> (Self, Self) {
        (DiceSet::new(self.dice.drain(..ind).collect()), DiceSet::new(self.dice))
    }
}

impl Default for Die {
    fn default() -> Self {
        Die::new(6)
    }
}

impl Default for DiceSet {
    fn default() -> Self {
        DiceSet::from_die(Die::default())
    }
}

impl FromStr for DiceSet {
    type Err = IoError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let mut dice = Vec::new();

        s.split(' ')
            .map(
                |ss| {
                    let mut iter = ss.split('d');
                    (iter.next(), iter.next()) 
                }
            ).try_for_each(
                |tup| {
                    match tup {
                        (Some(m), Some(n)) => {
                            dice.append(&mut vec![n.parse()?;
                                        m.parse()
                                        .map_err(|_| IoError::new(ErrorKind::InvalidData, "Number of dice must be an integer"))?]);
                            Ok(())
                        },
                        _ => Err(IoError::new(ErrorKind::InvalidData, "Incorrect string format")),
                    }
                }
            )?;

        Ok(DiceSet::new(dice))
    }
}


impl ToString for DiceSet {
    fn to_string(&self) -> String {
        self.dice
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }
}

impl Roll for DiceSet {
    fn roll(self) -> Outcome {
        let outcomes = self.dice
            .iter()
            .map(|n| thread_rng().gen_range(1..=n.into()))
            .collect();
        Outcome::new(self.into_inner(), outcomes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use regex::Regex;

    #[test]
    fn roll_single() {
        let die = DiceSet::from_u8(10);
        die.roll();
    }

    #[test]
    fn roll_multiple() {
        let dice = DiceSet::new(vec![6.into(); 10]);
        println!("{:?}", dice.roll().into_string());
    }

    #[test]
    fn roll_from_str() {
        let dice: DiceSet = "2d6 3d8 8d4".parse().unwrap();
        println!("{}", dice.roll().into_string());
    }
}
