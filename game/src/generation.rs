use std::fmt::Display;
use std::{fs, str::FromStr, sync::Arc};
use fyrox::rand;
use strum_macros::EnumString;
use rand::prelude::*;
use rand::distributions::WeightedIndex;

#[derive(Debug, EnumString,PartialEq,PartialOrd,Copy,Clone)]
pub(crate) enum Side {
    Water,
    LWater,
    RWater,
    Land,
    ThinLand,
}

#[derive(Debug,Clone)]
pub(crate) struct Tile {
    pub(crate) prefab_path: String,
    north: Side,
    south: Side,
    east: Side,
    west: Side,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.prefab_path)
    }
}

type CollapsingGrid = Vec<Result<Tile,Vec<Tile>>>;

pub(crate) fn import_biome(file_path: &str) -> Result<Vec<Tile>, &str> {
    let contents = fs::read_to_string(file_path).expect(&format!("File not found! {}",file_path));
    Ok(contents.lines().filter(|l| !l.is_empty()).map(|x| import_tile(x).unwrap()).collect())
}
fn import_tile(line: &str) -> Result<Tile, &str> {
    let mut parts = line.split_whitespace().into_iter();
    
    Ok(Tile {
        prefab_path: parts.next().expect(&"weird path?".to_string()).to_owned(),
        north: parts.nth(1).ok_or("North missing").map(|x| Side::from_str(x).expect(&format!("North bad: {}",x))).unwrap(),
        east: parts.next().ok_or("East missing").map(|x| Side::from_str(x).expect(&format!("East bad: {}",x))).unwrap(),
        south: parts.next().ok_or("South missing").map(|x| Side::from_str(x).expect(&format!("South bad: {}",x))).unwrap(),
        west: parts.next().ok_or("West missing").map(|x| Side::from_str(x).expect(&format!("West bad: {}",x))).unwrap(),
    })
}

pub(crate) fn do_wave_function_collapse(basegrid:&mut CollapsingGrid ) -> () {
    // let mut grid = basegrid;
    // , choices:Vec<Tile>
    // let mut entropies = basegrid: Map<i32,Map<i32,f32>>;
    // let lowestEntropyCell = getLowestEntropy(grid);
    // while (lowestEntropyCell.is_some()) {
    for i in 0..=15 {
        for j in 0..=15 {
            collapse_cell(i,j,basegrid);
            
            // lowestEntropyCell = getLowestEntropy(grid);
        }
    }
}

fn getLowestEntropy() {
    todo!()
}

fn collapse_cell(x:i32, y:i32, grid: &mut CollapsingGrid) -> () {
    println!(" collapse {} {}",x,y);
    if let Err(choices) = &grid[(x+y*16) as usize] {
        let weights : Vec<i32> = choices.iter().map(|_x| 1).collect();
        let dist = WeightedIndex::new(&weights).expect(&format!("what? {} {}",x,y));
        let mut rng = thread_rng();
        let a =choices[dist.sample(&mut rng)].clone();
        println!("out of {} i chose {}", choices.len(), a);
        grid[(x+y*16) as usize] = Ok(a);
        propagate_collapse(x,y,100,100,grid)
    }
}

fn propagate_collapse(x:i32, y:i32, origin_x:i32, origin_y:i32, grid: &mut CollapsingGrid) -> () {
    for (nextX,nextY,direction) in [(x,y+1,Direction::North),(x,y-1,Direction::South),(x-1,y,Direction::East),(x+1,y,Direction::West)] {
        if origin_x == nextX && origin_y == nextY {continue;}
        let out_of_bounds = |x| x<=-1 || x>=16;
        if out_of_bounds(nextY) || out_of_bounds(nextX) {continue;}
        if let Err(adjacentPossibles) = &grid[(nextX+nextY*16) as usize] {
            let updated_possibles : Vec<Tile> = adjacentPossibles.iter().filter(|tile| grid[(x+y*16) as usize].clone().map_or_else(|origin_tiles| origin_tiles.iter().any(|origin| connects(origin,tile,direction)), |origin| connects(&origin,&tile,direction))).map(|l| l.clone()).collect();
            // println!("alter");
            // println!("({},{}) from ({},{})",nextX,nextY,x,y);
            // for i in adjacentPossibles {
            //     println!("({},{}) b4:{}",nextX,nextY, i);
            // }
            // println!("walter");
            // for i in updated_possibles.clone() {
            //     println!("({},{}) after:{}",nextX,nextY, i);
            // }
            if updated_possibles.len() == adjacentPossibles.len() {continue;}
            if updated_possibles.len() == 1 {
                grid[(nextX+nextY*16) as usize] = Ok((*updated_possibles.first().unwrap()).clone());
            } else {
                grid[(nextX+nextY*16)  as usize] = Err(updated_possibles);
            }
            propagate_collapse(nextX,nextY,x,y,grid);
        }
    }
}


pub(crate) fn connects(origin :&Tile, connecter:&Tile, direction:Direction) -> bool {
    let (side1,side2) = match direction {
        Direction::North => (origin.north,connecter.south),
        Direction::South => (origin.south,connecter.north),
        Direction::East => (origin.east,connecter.west),
        Direction::West => (origin.west,connecter.east),
    };
    match (side1, side2) {
        (Side::Land, Side::Land) =>    true,
        (Side::ThinLand, Side::Land) => true,
        (Side::Land, Side::ThinLand) => true,
        (Side::Water, Side::Water) => true,
        (Side::LWater, Side::RWater) => true,
        (Side::RWater, Side::LWater) => true,
        _ => false,
    }
}

#[derive(Debug,Copy,Clone)]
pub enum Direction {
    North,
    South,
    East,
    West
}