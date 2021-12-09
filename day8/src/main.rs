use std::{
    fmt::Debug,
    ops::{BitAnd, BitOr, Neg, Sub},
};

fn part1(input: &[&str]) -> usize {
    input
        .into_iter()
        .map(|&s| s.split_whitespace().skip_while(|&s| s != "|").skip(1))
        .flatten()
        .filter(|s| match s.len() {
            2 => true,
            3 => true,
            4 => true,
            7 => true,
            _ => false,
        })
        .count()
}

// 448 too high because I thought a 4 used 5 segments
// 264 correct

type Etype = String;

#[derive(PartialEq, Eq, Clone, Copy)]
struct Segbitmap(u8);

impl Debug for Segbitmap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#09b}", self.0)
    }
}

impl Default for Segbitmap {
    fn default() -> Self {
        Segbitmap(127u8)
    }
}

impl Sub for Segbitmap {
    type Output = Segbitmap;
    fn sub(self, Segbitmap(rhs): Segbitmap) -> Segbitmap {
        let Segbitmap(lhs) = self;
        Segbitmap(lhs & !rhs)
    }
}

impl Neg for Segbitmap {
    type Output = Segbitmap;
    fn neg(self) -> Segbitmap {
        Segbitmap::default() - self
    }
}

impl BitAnd for Segbitmap {
    type Output = Segbitmap;
    fn bitand(self, rhs: Self) -> Self::Output {
        let Segbitmap(lhs) = self;
        let Segbitmap(rhs) = rhs;
        Segbitmap(lhs & rhs)
    }
}

impl BitOr for Segbitmap {
    type Output = Segbitmap;
    fn bitor(self, rhs: Self) -> Self::Output {
        let Segbitmap(lhs) = self;
        let Segbitmap(rhs) = rhs;
        Segbitmap(lhs | rhs)
    }
}

fn seglist_to_segbitmap(l: &str) -> Result<Segbitmap, Etype> {
    let v: Vec<u8> = l.chars().map(|c: char| c as u8 - 'a' as u8).collect();
    if !v.iter().all(|&x| x < 8u8) {
        Err("Character encountered greater than 'g'.".to_string())
    } else {
        Ok(Segbitmap(
            v.into_iter().map(|x| 1 << x).fold(0u8, u8::bitor),
        ))
    }
}

fn displayseglist_to_displayidarr<const CNT: usize>(l: &str) -> Result<[Segbitmap; CNT], Etype> {
    let mut ret = [Segbitmap::default(); CNT];
    let xx: Vec<Segbitmap> = l
        .split_ascii_whitespace()
        .map(seglist_to_segbitmap)
        .collect::<Result<Vec<Segbitmap>, Etype>>()?;
    if xx.len() < CNT {
        return Err("Not enough display entries.".to_string());
    } else if xx.len() > CNT {
        return Err("Too many display entries.".to_string());
    } else {
        for i in 0..CNT {
            ret[i] = xx[i];
        }
        Ok(ret)
    }
}

type DisplayData = (
    Segbitmap,
    Segbitmap,
    Segbitmap,
    [Segbitmap; 3],
    [Segbitmap; 3],
    Segbitmap,
);

fn displayseglist_to_displaysegcats(l: &str) -> Result<DisplayData, Etype> {
    let mut displayidarr: [Segbitmap; 10] = displayseglist_to_displayidarr(l)?;

    displayidarr.sort_unstable_by(|&a, &b| a.0.count_ones().cmp(&b.0.count_ones()));

    Ok((
        displayidarr[0], // 1
        displayidarr[1], // 7
        displayidarr[2], // 4
        displayidarr[3..6]
            .try_into()
            .map_err(|_| "Slice to array conversion error.")?, // 5 segs
        displayidarr[6..9]
            .try_into()
            .map_err(|_| "Slice to array conversion error.")?, // 6 segs
        displayidarr[9], // 8
    ))
}

//  0000
// 5    1
// 5    1
//  6666
// 4    2
// 4    2
//  3333

// display 1 -> 2 segments
// display 7 -> 3 segments
// display 4 -> 4 segments
// display 2 -> 5 segments
// display 3 -> 5 segments
// display 5 -> 5 segments
// display 6 -> 6 segments
// display 0 -> 6 segments
// display 9 -> 6 segments
// display 8 -> 7 segments

fn gen_alphabet(insegchars: &str) -> Result<[Segbitmap; 10], Etype> {
    let (one, seven, four, fivesegs, sixsegs, _) = displayseglist_to_displaysegcats(insegchars)?;

    // println!("{:?}", (one, seven, four, fivesegs, sixsegs, eight));

    let segs_036 = fivesegs
        .iter()
        .fold(Segbitmap::default(), |acc, &x| acc & x);
    let segs_0523 = sixsegs.iter().fold(Segbitmap::default(), |acc, &x| acc & x);

    let disp: [Segbitmap; 7] = [
        seven - one,                  // 0
        one - segs_0523,              // 1
        one - (one - segs_0523),      // 2
        segs_036 & (-(four | seven)), // 3
        -(four | segs_036),           // 4
        segs_0523 - (one | segs_036), // 5
        four & segs_036,              // 6
    ];

    // println!("{:?}", disp);

    Ok([
        -disp[6],                       // 0
        disp[1] | disp[2],              // 1
        -(disp[5] | disp[2]),           // 2
        -(disp[5] | disp[4]),           // 3
        -(disp[0] | disp[3] | disp[4]), // 4
        -(disp[1] | disp[4]),           // 5
        -disp[1],                       // 6
        disp[0] | disp[1] | disp[2],    // 7
        Segbitmap::default(),           // 8
        -disp[4],                       // 9
    ])
}

fn solveline(input: &str) -> Result<usize, Etype> {
    if let Some((insegchars, outsegchars)) = input.split_once(" | ") {
        let outsegs: [Segbitmap; 4] = displayseglist_to_displayidarr(outsegchars)?;
        let alphabet = gen_alphabet(insegchars)?;

        Ok(outsegs
            .into_iter()
            .map(|x| {
                alphabet
                    .iter()
                    .position(|&y| x == y)
                    .ok_or(format!("Outseg {:?} not in alphabet {:?}.", x, alphabet))
            })
            .collect::<Result<Vec<usize>, Etype>>()?
            .into_iter()
            .fold(0, |acc, x| acc * 10 + x))
    } else {
        Err("Bad delimiter.".to_string())
    }
}

fn part2(input: &[&str]) -> Result<usize, Etype> {
    input.into_iter().map(|&s| solveline(s)).sum()
}

// 1063760 correct

fn main() {
    let input: Vec<&str> = include_str!("../input.txt").lines().collect();
    println!("{}", part1(input.as_slice()));
    println!("{:?}", part2(input.as_slice()));
}
