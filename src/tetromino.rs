const TETROMINO_LENGTH: u32 = 4;
const LENGTH_OF_NEXT_LIST: u32 = 7;

// Different forms appearing on screen 
const ZERO: [[u32; 4]; 4] = [[0,0,0,0],[0,0,0,0],[0,0,0,0],[0,0,0,0]];
const CYAN: [[u32; 4]; 4] = [[0,0,1,0],[0,0,1,0],[0,0,1,0],[0,0,1,0]];
const YELLOW: [[u32; 4]; 4] = [[0,0,0,0],[0,2,2,0],[0,2,2,0],[0,0,0,0]];
const GREEN: [[u32; 4]; 4] = [[0,0,0,0],[0,3,3,0],[3,3,0,0],[0,0,0,0]];
const RED: [[u32; 4]; 4] = [[0,0,0,0],[0,4,4,0],[0,0,4,4],[0,0,0,0]];
const ORANGE: [[u32; 4]; 4] = [[0,0,0,0],[0,5,0,0],[0,5,0,0],[0,5,5,0]];
const BLUE: [[u32; 4]; 4] = [[0,0,0,0],[0,0,6,0],[0,0,6,0],[0,6,6,0]];
const MAJENTA: [[u32; 4]; 4] = [[0,0,0,0],[0,7,0,0],[7,7,7,0],[0,0,0,0]];

struct Reserve {
    // TODO implement attribute "Matrix" form
    // "Matrix" form,
    u32 switched,
}

impl Reserve {
    fn new() -> [[]] {
        
    }
}

struct Tetromino {
    u32 x,
    u32 y,
    // TODO implement attribute "Matrix" form
    // "Matrix" form,
    Field &field,
    // TODO implement attribute Watchdog or Timer
    // "Watchdog" watchdog,
    Reserve reserve,
    // TODO implement attribute "Matrix3D" nextform
    // "Matrix3D" nextform,
}

impl Tetromino {
    fn new(field: &Field) -> Tetromino {
        Tetromino {
            x: 0,
            y: field.getDepth() /2 -2,
            field: field,
            
        }
    }
}