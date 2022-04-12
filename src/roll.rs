use crate::dice::Die;

pub trait Roll {
    fn roll(self) -> Outcome;
}

#[derive(Debug)]
pub struct Outcome {
    pub dice: Vec<Die>,
    pub outcomes: Vec<u8>,
    pos: usize,
}

impl Outcome {
    pub fn new(dice: Vec<Die>, outcomes: Vec<u8>) -> Self {
        Outcome { dice, outcomes, pos: 0 }
    }
}

impl Iterator for Outcome {
    type Item = (Die, u8);
    fn next(&mut self) -> Option<Self::Item> {
        self.pos += 1;
        Some((*self.dice.get(self.pos - 1)?, self.outcomes[self.pos - 1]))
    }
}

impl Outcome {
    pub fn into_string(self) -> String {
        // let mut s = format!("Dice throw:\n");
        self.map(|(d, n)| format!("{}: {}", d.to_string(), n))
            .collect::<Vec<_>>()
            .join("\n")
        // s
    }
}
