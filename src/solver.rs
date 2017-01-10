use std;
use num::NumCast;
use castutils::CastInt;

pub struct InfLoopSolver {

}

pub enum CellType {
    Empty,
    Peg,
    Line,
    Corner,
    Three,
    Cross
}

macro_rules! sides_known {
    ( $($a:expr),* ) => { [$(Some($a)),*] };
}

fn check_same(a: &[Option<bool>; 4], b: &[Option<bool>; 4]) -> bool {
    for i in 0..4 {
        let a = &a[i];
        let b = &b[i];
        // do options have the same value?
        if a.is_some() && b.is_some() && a != b { return false; }
    }
    true
}

fn peg_solve(sides: &[Option<bool>; 4], peg_val: bool) -> Result<[Option<bool>; 4], ()> {
    let mut single = None;
    let mut possible = [true; 4];

    for i in 0..4 {
        if let Some(v) = sides[i] {
            if v == peg_val {
                if single.is_some() { return Err(()); } // contradiction
                single = Some(i);
            } else {
                possible[i] = false;
            }
        }
    }

    let mut out = [Some(!peg_val); 4];

    if let Some(i) = single {
        out[i] = Some(peg_val);
        Ok(out)
    } else {
        let count = possible.iter().filter(|v| **v).count();
        if count == 0 { Err(()) } // contradiction
        else if count == 1 {
            let mut at = 0;
            for i in 0..4 { if possible[i] { at = i; break; }}

            out[at] = Some(peg_val);
            Ok(out)
        } else { Ok(sides.clone()) }
    }
}

impl CellType {
    pub fn basic_shape(&self) -> [bool; 4] {
        match self {
            &CellType::Empty => [false; 4],
            &CellType::Peg => [true, false, false, false],
            &CellType::Line => [true, false, true, false],
            &CellType::Corner => [true, true, false, false],
            &CellType::Three => [true, true, true, false],
            &CellType::Cross => [true; 4],
        }
    }

    pub fn rot_shape<T: CastInt<usize>>(&self, rot: T) -> [bool; 4] {
        let rot: usize = rot.cast_int() % 4;
        let arr = match self {
            &CellType::Empty => [false; 7],
            &CellType::Peg => [true, false, false, false, true, false, false],
            &CellType::Line => [true, false, true, false, true, false, true],
            &CellType::Corner => [true, true, false, false, true, true, false],
            &CellType::Three => [true, true, true, false, true, true, true],
            &CellType::Cross => [true; 7],
        };
        array_ref!(arr, rot, 4).clone()
    }

    pub fn solve_sides(&self, sides: &[Option<bool>; 4]) -> Result<[Option<bool>; 4], ()> {
        match self {
            &CellType::Empty => {
                if sides.iter().filter_map(Option::as_ref).any(|v| *v == true) { Err(()) }
                else { Ok([Some(false); 4]) }
            },
            &CellType::Peg => {
                peg_solve(sides, true)
            },
            &CellType::Line => {
                let mut at = None;

                for i in 0..4 {
                    if let Some(v) = sides[i] {
                        // one known needed
                        at = Some((i, v));
                        break;
                    }
                }
                
                if let Some((i, v)) = at {
                    let out =
                        if i == 0 || i == 2 { sides_known!(v, !v, v, !v) }
                        else { sides_known!(!v, v, !v, v) };

                    // search for contradiction in input
                    if check_same(sides, &out) { Ok(out) }
                    else { Err(()) }
                } else { Ok(sides.clone()) }
            },
            &CellType::Corner => {
                let mut out = [None; 4];

                for i in 0..4 {
                    if let Some(v) = sides[i] {
                        out[i] = Some(v);

                        // opposite
                        let j = (i + 2) % 4;
                        if sides[j] == Some(v) { return Err(()); } // contradiction check
                        out[j] = Some(!v);
                    }
                }

                Ok(out)
            },
            &CellType::Three => {
                peg_solve(sides, false)
            },
            &CellType::Cross => {
                if sides.iter().filter_map(Option::as_ref).any(|v| *v == false) { Err(()) }
                else { Ok([Some(true); 4]) }
            },
        }
    }
}