
enum State {
    EMPTY,
    FULL,
}

struct Field {
    u32 length,
    u32 depth,
    u32 ** grid,
    State state,
    u32 numberOfLines,
    u32 level,
    u32 score,
}

fn FieldNew(length: u32, depth: u32) -> Field {
    Field {
        length: length,
        depth: depth,
        numberOfLines: 0,
        level: 0,
        score: 0,
        // TODO define grid
        // grid: 
    }
}
