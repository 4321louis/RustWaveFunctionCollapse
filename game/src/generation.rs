use std::any;
use std::{env, fs, str::FromStr, sync::Arc};
use fyrox::rand;
use strum_macros::EnumString;
use rand::prelude::*;
use rand::distributions::WeightedIndex;

#[derive(Debug, EnumString,PartialEq,PartialOrd)]
pub(crate) enum Side {
    Water,
    LWater,
    RWater,
    Land,
    ThinLand,
}

#[derive(Debug)]
struct Tile {
    // entity:Entity,
    north: Side,
    south: Side,
    east: Side,
    west: Side,
}

type CollapsingGrid = Vec<Result<Tile,Vec<Tile>>>;

fn import_biome(file_path: &str) -> Result<Vec<Tile>, &str> {
    let contents = fs::read_to_string(file_path).unwrap();
    Ok(contents.lines().filter(|l| !l.is_empty()).map(|x| import_tile(x).unwrap()).collect())
}
fn import_tile(line: &str) -> Result<Tile, &str> {
    let mut parts = line.split_whitespace();
    // if ()
    Ok(Tile {
        north: parts.nth(1).and_then(|x| Side::from_str(x).ok()).ok_or("North bad")?,
        south: parts.nth(2).and_then(|x| Side::from_str(x).ok()).ok_or("South bad")?,
        east: parts.nth(3).and_then(|x| Side::from_str(x).ok()).ok_or("East bad")?,
        west: parts.nth(4).and_then(|x| Side::from_str(x).ok()).ok_or("West bad")?,
        // entity:
    })
}

fn do_wave_function_collapse(basegrid:CollapsingGrid, choices:Vec<Tile> ) -> () {
    let grid = basegrid;
    // let mut entropies = basegrid: Map<u8,Map<u8,f32>>;
    // let lowestEntropyCell = getLowestEntropy(grid);
    // while (lowestEntropyCell.is_some()) {
    for i in 1..10 {
        for j in 1..10 {

            collapseCell(i,j,100,100,grid);
            
            // lowestEntropyCell = getLowestEntropy(grid);
        }
    }
}

fn getLowestEntropy() {
    todo!()
}

fn collapseCell(x:u8, y:u8, origin_x:u8, origin_y:u8, mut grid: CollapsingGrid) -> () {
    
    if let Err(choices) = grid[(x+y*16) as usize] {

        let weights : Vec<u8> = choices.iter().map(|x| 1).collect();
        let dist = WeightedIndex::new(&weights).unwrap();
        let mut rng = thread_rng();
        grid[(x+y*16) as usize] = Ok(choices[dist.sample(&mut rng)]);
        propagateCollapse(x,y,origin_x,origin_y,grid)
    }
}

fn propagateCollapse(x:u8, y:u8, origin_x:u8, origin_y:u8, mut grid: CollapsingGrid) -> () {
    for (nextX,nextY,direction) in [(x+1,y,Direction::North),(x-1,y,Direction::South),(x,y+1,Direction::East),(x,y-1,Direction::West)] {
        if origin_x == x && origin_y == y {continue;}
        if let Err(adjacentPossibles) = grid[(nextX+nextY*16) as usize] {
            let updatedPossibles : Vec<Tile> = adjacentPossibles.iter().filter(|tile| grid[(x+y*16) as usize].iter().any(|origin| connects(*origin,**tile,direction))).map(|l| *l).collect();
            if updatedPossibles.len() == adjacentPossibles.len() {continue;}
            if updatedPossibles.len() == 1 {
                grid[(nextX+nextY*16) as usize] = Ok(*updatedPossibles.first().unwrap());
            }
            grid[(nextX+nextY*16)  as usize] = Err(updatedPossibles);
        }
    }
}


fn connects(origin :Tile, connecter:Tile, direction:Direction) -> bool {
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

enum Direction {
    North,
    South,
    East,
    West
}