use num::FromPrimitive;
use num_derive::FromPrimitive;
use sexp::{self, Sexp};
use std::fmt::{self, Debug, Display, Formatter};
use std::str::FromStr;

use crate::sexpr::*;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, FromPrimitive)]
pub enum Dir {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

impl Debug for Dir {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match *self {
            Dir::North => write!(f, "N"),
            Dir::East => write!(f, "E"),
            Dir::South => write!(f, "S"),
            Dir::West => write!(f, "W"),
        }
    }
}

impl Display for Dir {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        Debug::fmt(self, f)
    }
}

impl Sexpable for Dir {
    fn to_sexp(&self) -> Sexp {
        sexp::atom_s(&self.to_string())
    }

    fn from_sexp(sexp: &Sexp) -> Result<Self, SexpError> {
        let s = expect_string(sexp)?;
        Dir::from_str(s).map_err(SexpError::Custom)
    }
}

impl Dir {
    pub fn from_int(i: u8) -> Dir {
        FromPrimitive::from_u8(i).unwrap()
    }

    pub fn opponent_of(&self, other: &Dir) -> bool {
        (*self as u8) | 0x1 != (*other as u8) | 0x1
    }

    pub fn iter(&self) -> IterDir {
        IterDir {
            current: *self,
            last: FromPrimitive::from_u8((*self as u8 + 3) % 4).unwrap(),
        }
    }
}

impl FromStr for Dir {
    type Err = String;

    fn from_str(s: &str) -> Result<Dir, String> {
        match s {
            "N" | "n" => Ok(Dir::North),
            "E" | "e" => Ok(Dir::East),
            "S" | "s" => Ok(Dir::South),
            "W" | "w" => Ok(Dir::West),
            _ => Err(format!("Invalid direction: {}", s)),
        }
    }
}

pub struct IterDir {
    current: Dir,
    last: Dir,
}

impl Iterator for IterDir {
    type Item = Dir;

    fn next(&mut self) -> Option<Dir> {
        if self.current == self.last {
            None
        } else {
            let next = FromPrimitive::from_u8((self.current as u8 + 1) % 4).unwrap();
            self.current = next;
            Some(next)
        }
    }
}

pub const DIRS: [Dir; 4] = [Dir::North, Dir::East, Dir::South, Dir::West];

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, FromPrimitive)]
pub enum Side {
    NS = 0,
    WE = 1,
}

impl Debug for Side {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match *self {
            Side::NS => write!(f, "NS"),
            Side::WE => write!(f, "WE"),
        }
    }
}

impl Display for Side {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        Debug::fmt(self, f)
    }
}

impl Sexpable for Side {
    fn to_sexp(&self) -> Sexp {
        sexp::atom_s(&self.to_string())
    }

    fn from_sexp(sexp: &Sexp) -> Result<Side, SexpError> {
        let s = expect_string(sexp)?;
        Side::from_str(s).map_err(|()| SexpError::InvalidTag(s.to_string()))
    }
}

impl Side {
    pub fn from_int(i: u8) -> Side {
        FromPrimitive::from_u8(i).unwrap()
    }
}

impl FromStr for Side {
    type Err = ();

    fn from_str(s: &str) -> Result<Side, ()> {
        match s {
            "NS" | "ns" => Ok(Side::NS),
            "WE" | "we" => Ok(Side::WE),
            _ => Err(())
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, FromPrimitive)]
pub enum Vulnerability {
    None = 0,
    NS = 1,
    WE = 2,
    Both = 3,
}

impl Debug for Vulnerability {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match *self {
            Vulnerability::None => write!(f, "None"),
            Vulnerability::NS => write!(f, "NS"),
            Vulnerability::WE => write!(f, "WE"),
            Vulnerability::Both => write!(f, "Both"),
        }
    }
}

impl Display for Vulnerability {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
    Debug::fmt(self, f)
    }
}

impl FromStr for Vulnerability {
    type Err = ();

    fn from_str(s: &str) -> Result<Vulnerability, ()> {
        match s {
            "None" | "none" => Ok(Vulnerability::None),
            "NS" | "ns" => Ok(Vulnerability::NS),
            "WE" | "we" => Ok(Vulnerability::WE),
            "Both" | "both" => Ok(Vulnerability::Both),
            _ => Err(())
        }
    }
}
