
// mod tetromino;
//  COmming from tetromino, remove after
const TETROMINO_LENGTH: u32 = 4;
const LENGTH_OF_NEXT_LIST: u32 = 7;

struct Field {
    length: usize,
    depth: usize,
    grid: [[u32; 24]; 10],
    empty: bool,
    numberOfLines: u32,
    level: u32,
    score: u32,
}

impl Field {

    // Field constructor
    fn new(length: usize, depth: usize) -> Field {
        Field {
            length: length,
            depth: depth,
            grid: [[0; 24]; 10],
            empty:false,
            numberOfLines: 0,
            level: 0,
            score: 0,
        }
    }

    // Immutable access --> 
    fn getLength(&self) -> &usize {
        &self.length
    }
    fn getDepth(&self) -> &usize {
        &self.depth
    }
    // TODO implement getter for grid
    fn getState(&self) -> &bool {
        &self.empty
    }
    fn getNumberOfLines(&self) -> &u32 {
        &self.numberOfLines
    }
    fn getLevel(&self) -> &u32 {
        &self.level
    }
    fn getScore(&self) -> &u32 {
        &self.score
    }

    // TODO implement Setters

//  Determine si le joueur a perdu
    fn isInLossZone(&self) -> bool {
        println!("IS IN LOSS ZONE\n");
        let mut loss = false;
        let mut i: usize = 0;
        while i<4 && !loss {
            let mut j:usize = 0;
            while j<self.depth && !loss {
                if self.grid[i][j] != 0    {
                    loss = true;
                    self.empty = true;
                }
                j+= 1;
            }
            i+= 1;
        }
        loss
    }


}
