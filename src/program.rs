use std::collections::HashSet;

pub struct Program {
    pub grid: Vec<Vec<Space>>,
    pub width: i32,
    pub height: i32,
}

impl Program {
    pub fn new(file_contents: String) {
        let mut grid = Vec::new();
        for line in file_contents.lines() {
            let mut curr_line_spaces = Vec::new();
            for c in line.chars() {
                let next_space = match c {
                    '0'..='9' => {
                        let int_value = c.to_digit(10).unwrap() as i32;
                        Space::new_value(ValueType::Integer(int_value))
                    },
                    'a'..='z' | 'A'..='Z' => Space::new_value(ValueType::Character(c)),
                    '\"' => Space::new_value(ValueType::Character(' ')),
                    '\\' => Space::new_value(ValueType::Character('\n')),
                    ';' => Space::new_value(ValueType::HaltProgram),
                    
                };
                curr_line_spaces.push(next_space);
            }
            grid.push(curr_line_spaces);
        }

        
    }

    fn find_longest_line_and_count_lines(file_contents: &str) -> (usize, usize) {
        let mut max_length = 0;
        let mut num_lines = 0;
    
        for line in file_contents.lines() {
            num_lines += 1;
            let line_length = line.len();
            if line_length > max_length {
                max_length = line_length;
            }
        }
        
        (max_length, num_lines)
    }

    fn update_board(&mut self) {
        let mut updated: HashSet<(usize, usize)> = HashSet::new();
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                if !updated.contains(&(y, x)) {
                    self.update_space(y, x, &mut updated);
                }
            }
        }
    }


    fn update_space(&mut self, y: usize, x: usize, updated: &mut HashSet<(usize, usize)>) {
        if updated.contains(&(y, x)) {
            return;
        }

        match self.grid[y][x].space_type {
            SpaceType::Conveyor(conveyor_type) => { 
                if self.grid[y][x].in_operable_range.0 {
                    self.update_space(y, x, updated);
                }
                if self.grid[y][x].in_operable_range.1 {
                    self.update_space(y, x, updated);
                }
                let next_space = match conveyor_type {
                    ConveyorType::Up => (y - 1, x),
                    ConveyorType::Down => (y + 1, x),
                    ConveyorType::Left => (y, x - 1),
                    ConveyorType::Right => (y, x + 1),
                    ConveyorType::DoubleUp => (y - 2, x),
                    ConveyorType::DoubleDown => (y + 2, x),
                    ConveyorType::DoubleLeft => (y, x - 2),
                    ConveyorType::DoubleRight => (y, x + 2),
                };
                 if self.push_value(self.grid[y][x].value, next_space, updated) {
                    self.grid[y][x].value = ValueType::None;
                 }
            }
            SpaceType::LogicalConveyor => { // Cannot hold value
                if y == 0 || y == (self.height - 1) as usize {
                    return;
                }

                let shift_left = match self.grid[y + 1][x].value {
                    ValueType::Integer(i) => i != 0,
                    ValueType::None => return,
                    _ => true,
                };

                let destination = if shift_left {(y, x - 1)} else {(y, x + 1)};
                if self.push_value(self.grid[y - 1][x].value, destination, updated) {
                    self.grid[y - 1][x].value = ValueType::None;
                }
            }
            SpaceType::Operator(operator_type) => { // Cannot hold value
                if x == 0 || x == (self.width - 1) as usize {
                    return;
                }

                let left_value = match self.grid[y][x - 1].value {
                    ValueType::Integer(i) => i,
                    ValueType::Character(c) => c as i32,
                    _ => return,
                };
                let right_value = match self.grid[y][x + 1].value {
                    ValueType::Integer(i) => i,
                    ValueType::Character(c) => c as i32,
                    _ => return,
                };

                self.grid[y][x - 1].value = ValueType::None;
                self.grid[y][x + 1].value = ValueType::None;

                let result = match operator_type {
                    OperatorType::Addition => left_value + right_value,
                    OperatorType::Subtraction => left_value - right_value,
                    OperatorType::Multiplication => left_value * right_value,
                    OperatorType::Division => left_value / right_value,
                    OperatorType::Modulus => left_value % right_value,
                    OperatorType::And => if left_value != 0 && right_value != 0 {1} else {0},
                    OperatorType::Or => if left_value != 0 || right_value != 0 {1} else {0},
                    OperatorType::Equals => if left_value == right_value {1} else {0},
                    OperatorType::LessThan => if left_value < right_value {1} else {0},
                    OperatorType::GreaterThan => if left_value > right_value {1} else {0},
                };

                 if self.push_value(ValueType::Integer(result), (y + 1, x), updated) {
                    self.grid[y][x - 1].value = ValueType::None;
                    self.grid[y][x + 1].value = ValueType::None;
                 }
            }
            SpaceType::Processor(processor_type) => { // Cannot hold value
                match processor_type {
                    ProcessorType::Delete => self.grid[y][x].value = ValueType::None,
                    ProcessorType::Print => {
                        match self.grid[y][x].value {
                            ValueType::Integer(i) => print!("{i}"),
                            ValueType::Character(c) => print!("{c}"),
                            ValueType::HaltProgram => todo!("stop program at this point"),
                            _ => {}
                        }
                    }
                }
            }
            SpaceType::Wall => {},
        }
    }

    fn push_value(&mut self, new_value: ValueType, destination: (usize, usize), updated: &mut HashSet<(usize, usize)>) -> bool {
        let (y, x) = destination;
        if y >= (self.height - 1) as usize {
            return true;
        }
        if x >= (self.width - 1) as usize {
            return true;
        }

        match self.grid[y][x].value {
            ValueType::None => {
                self.grid[y][x].value = new_value;
                updated.insert(destination);
                return true;
            }
            ValueType::Character(_) | ValueType::Integer(_) | ValueType::HaltProgram => {
                if !updated.contains(&destination) {
                    self.update_space(y, x, updated);
                    return self.push_value(new_value, destination, updated);
                }
            }
            ValueType::CannotHoldValue => {
                return false;
            }
        }

        false
    }
}

#[derive(Clone, Copy)]
pub struct Space {
    pub value: ValueType,
    pub space_type: SpaceType,
    pub in_operable_range: (bool, bool),
}

impl Space {
    fn new_value(value: ValueType) -> Self {
        Self {
            value,
            space_type: SpaceType::Conveyor(ConveyorType::Down),
            in_operable_range: (false, false),
        }
    }

    fn new_space(space_type: SpaceType) -> Self {
        Self {
            value: match space_type {
                SpaceType::Conveyor(_) | SpaceType::Processor(_) => ValueType::None,
                _ => ValueType::CannotHoldValue,
            },
            space_type,
            in_operable_range: (false, false,),
        }
    }
}

#[derive(Clone, Copy)]
pub enum ValueType {
    CannotHoldValue,
    None,
    HaltProgram,
    Integer(i32),
    Character(char),
}

#[derive(Clone, Copy)]
pub enum SpaceType {
    Wall,
    Conveyor(ConveyorType),
    LogicalConveyor,
    Operator(OperatorType),
    Processor(ProcessorType),
}

#[derive(Clone, Copy)]
pub enum ConveyorType {
    Up,
    Down,
    Left,
    Right,
    DoubleUp,
    DoubleDown,
    DoubleLeft,
    DoubleRight,
}

#[derive(Clone, Copy)]
pub enum OperatorType {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulus,
    And,
    Or,
    Equals,
    LessThan,
    GreaterThan,
}

#[derive(Clone, Copy)]
pub enum ProcessorType {
    Print,
    Delete,
}