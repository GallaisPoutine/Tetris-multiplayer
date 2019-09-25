
enum State {
    EMPTY,
    FULL,
}

struct Field {
    u32 length,
    u32 depth,
    // TODO implement attribute "Matrix" grid
    State state,
    u32 numberOfLines,
    u32 level,
    u32 score,
}

impl Field {
    // Field constructor
    fn new(length: u32, depth: u32) -> Field {
        Field {
            length: length,
            depth: depth,
            numberOfLines: 0,
            level: 0,
            score: 0,
            // TODO initialize grid
        }
    }

    // Immutable access --> 
    fn getLength(&self) -> &u32 {
        &self.length
    }
    fn getDepth(&self) -> &u32 {
        &self.depth
    }
    // TODO implement getter for grid
    fn getState(&self) -> &State {
        &self.state
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
}
