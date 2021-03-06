
#![feature(mixed_integer_ops)]

use std::{
    error::Error,
    fmt::Debug,
    ops::{
        Index,
        IndexMut,
    },
    str::FromStr,
};
use simple_error::{
    SimpleError,
    simple_error,
};

const TEST_INPUT: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
";

#[derive(Debug, Clone)]
struct Grid {
    height: usize,
    width: usize,
    grid: Vec<u8>,
}

impl Grid {
    fn step(&mut self) -> usize {
        for o in &mut self.grid {
            *o += 1;
        }
        loop {
            let mut changed = false;
            for y in 0..self.height {
                for x in 0..self.width {
                    let mut flash = false;
                    for y_off in -1..=1 {
                        let yi = match y.checked_add_signed(y_off) {
                            Some(yi) if yi < self.height => yi,
                            _ => continue,
                        };
                        for x_off in -1..=1 {
                            let xi = match x.checked_add_signed(x_off) {
                                Some(xi) if xi < self.width => xi,
                                _ => continue,
                            };
                            // 10 -> it will flash
                            // 11 -> it has given energie to its neighbors
                            if 10 == self[(x, y)] && 9 >= self[(xi, yi)] {
                                self[(xi, yi)] += 1;
                                changed = true;
                                flash = true;
                            }
                        }
                    }
                    if flash {
                        self[(x, y)] += 1;
                    }
                }
            }
            if !changed {
                break;
            }
        }
        let mut flashes = 0;
        for o in &mut self.grid {
            if *o > 9 {
                *o = 0;
                flashes += 1;
            }
        }
        flashes
    }
    fn steps(&mut self, n: usize) -> usize {
        (1..=n).fold(0, |flashes, _| {
            flashes + self.step()
        })
    }
    fn is_all_flash(&self) -> bool {
        for o in &self.grid {
            if 0 != *o {
                return false;
            }
        }
        true
    }
    fn all_flash(&mut self) -> usize {
        let mut step = 0;
        loop {
            self.step();
            step += 1;
            if self.is_all_flash() {
                return step;
            }
        }
    }
    fn ansi_print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if 0 == self[(x, y)] {
                    print!("\x1b[1m0\x1b[0m");
                } else {
                    print!("\x1b[2m{}\x1b[0m", self[(x, y)]);
                }
            }
            println!("");
        }
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = u8;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.grid[x + y * self.width]
    }
}

impl IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.grid[x + y * self.width]
    }
}

impl FromStr for Grid {
    type Err = SimpleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (grid, (width, _)/*, height*/) = s.chars()
            .try_fold((Vec::new(), (0,0)/*, 0*/), |(mut grid, (width, xi)/*, yi*/), c| {
                if '\n' == c {
                    if width != 0 && width != xi {
                        return Err(simple_error!("line lenght not consitent"));
                    }
                    Ok((grid, (xi, 0)/*, yi + 1*/))
                } else {
                    match c.to_digit(10) {
                        Some(d) => {
                            grid.push(d as u8);
                            Ok((grid, (width, xi + 1)/*, yi*/))
                        },
                        None => Err(simple_error!("{} is not a number"))
                    }
                }
            })?;
        Ok(Grid {
            height: grid.len()/width,
            width: width,
            grid: grid,
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    //let input = TEST_INPUT;
    let input = include_str!("input.txt");
    let g = Grid::from_str(input)?;

    let flashes = g.clone().steps(100);
    let steps_to_all_flash = g.clone().all_flash();
    println!("{}", flashes);
    println!("{}", steps_to_all_flash);

    Ok(())
}
