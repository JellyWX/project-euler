struct Spiral {
    pub start_num: usize,
    pub end_num: usize,
    pub row: usize,
    pub side_length: usize,
}

impl Spiral {
    pub fn new() -> Self {
        Spiral { row: 0, start_num: 1, end_num: 1, side_length: 1 }
    }

    pub fn next(&self) -> Self {
        let current_row = self.row + 1;
        let start_num = self.end_num + 1;

        let side_length = current_row * 2 + 1;

        let delta = (side_length - 1) * 4;

        Spiral { 
            row: current_row,
            start_num,
            end_num: start_num + delta - 1,
            side_length,
        }
    }

    pub fn diagonals_sum(&self) -> usize {
        if self.row == 0 {
            1
        }
        else {
            self.end_num + 
            (self.end_num - (self.side_length - 1)) + 
            (self.end_num - (self.side_length - 1) * 2) +
            (self.end_num - (self.side_length - 1) * 3)
        }
    }
}

fn main() {
    let mut spiral = Spiral::new();
    let mut count = 0;

    for _ in 0..501 {
        count += spiral.diagonals_sum();

        spiral = spiral.next();
    }

    println!("{}", count);
}
