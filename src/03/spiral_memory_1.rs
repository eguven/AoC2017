use std::env;

/// Represents our spiral memory, index 0,0 is top-left so [column][row]
#[derive(Debug)]
struct SpiralMemory {
    target: usize,
    dimension: usize,  // always a square
    x_of_one: usize,
    y_of_one: usize,
    x_of_target: Option<usize>,
    y_of_target: Option<usize>,
    distance_to_target: Option<usize>,
}

impl SpiralMemory {
    /// create a SpiralMemory for given target
    fn new_from_target(target: usize) -> SpiralMemory {
        let dimension = SpiralMemory::dimension_from_target(target);
        let (x_of_one, y_of_one) = SpiralMemory::idx_of_one_from_dimension(dimension);
        SpiralMemory {
            target: target, dimension: dimension,
            x_of_one: x_of_one, y_of_one: y_of_one,
            x_of_target: None, y_of_target: None, distance_to_target: None,
        }
    }

    /// calculate the dimension of the square grid
    fn dimension_from_target(target: usize) -> usize {
        (target as f64).sqrt().ceil() as usize
    }

    /// calculate the x/y of one (center of spiral with value 1)
    fn idx_of_one_from_dimension(dimension: usize) -> (usize, usize) {
        if dimension % 2 == 1 {
            let val = (dimension - 1) / 2;
            (val, val)
        } else {
            let val_y = dimension / 2;
            (val_y - 1, val_y)
        }
    }

    /// set the x, y of target
    fn calculate_idx_of_target(&mut self) {
        let diff_from_max = self.dimension.pow(2) - self.target;
        if  diff_from_max >= self.dimension {
            // this is just a matter of +/- the y dimension diff, not required for puzzle
            unimplemented!();
        }
        // maximum is either top-left (even dimension) or bottom-right (odd dimension)
        if self.dimension % 2 == 1 {
            let (x_of_max, y_of_max) = (self.dimension - 1, self.dimension - 1);
            self.x_of_target = Some(x_of_max - diff_from_max);
            self.y_of_target = Some(y_of_max);  // while above unimplemented
        } else {
            let (x_of_max, y_of_max) = (0, 0);
            self.x_of_target = Some(x_of_max + diff_from_max);
            self.y_of_target = Some(y_of_max);  // while above unimplemented
        }
    }

    /// set the distance_to_target which is hamming distance from one
    fn calculate_distance_to_target(&mut self) {
        let x_distance = (self.x_of_target.unwrap() as isize - self.x_of_one as isize).abs();
        let y_distance = (self.y_of_target.unwrap() as isize - self.y_of_one as isize).abs();
        self.distance_to_target = Some((x_distance + y_distance) as usize);
    }

}

fn main() {
    let args: Vec<_> = env::args().collect();
    let target = usize::from_str_radix(&args[1], 10).unwrap();
    let mut spiral_memory = SpiralMemory::new_from_target(target);
    spiral_memory.calculate_idx_of_target();
    spiral_memory.calculate_distance_to_target();
    println!("{:?}", spiral_memory);
    println!("{}", spiral_memory.distance_to_target.unwrap());
}
