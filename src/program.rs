use std::collections::HashSet;

pub struct Program {
    pub grid: Vec<Vec<Space>>,
    pub width: i32,
    pub height: i32,
    pub running: bool,
    prev: Option<Vec<Vec<Space>>>,
}

impl Program {
    pub fn new(file_contents: String, width: i32, height: i32) -> Program {
        let mut grid = Vec::new();
        for line in file_contents.lines() {
            let mut curr_line_spaces = Vec::new();
            let mut line_chars = 0;
            for c in line.chars() {
                line_chars += 1;
                let next_space = match c {
                    '0'..='9' => {
                        let int_value = c.to_digit(10).unwrap() as i32;
                        Space::new_value(ValueType::Integer(int_value))
                    },
                    'a'..='z' | 'A'..='Z' => Space::new_value(ValueType::Character(c)),
                    '\"' => Space::new_value(ValueType::Character(' ')),
                    '\\' => Space::new_value(ValueType::Character('\n')),
                    ';' => Space::new_value(ValueType::HaltProgram),
                    '+' => Space::new_space(SpaceType::Operator(OperatorType::Addition)),
                    '-' => Space::new_space(SpaceType::Operator(OperatorType::Subtraction)),
                    '*' => Space::new_space(SpaceType::Operator(OperatorType::Multiplication)),
                    '/' => Space::new_space(SpaceType::Operator(OperatorType::Division)),
                    '%' => Space::new_space(SpaceType::Operator(OperatorType::Modulus)),
                    '&' => Space::new_space(SpaceType::Operator(OperatorType::And)),
                    '|' => Space::new_space(SpaceType::Operator(OperatorType::Or)),
                    '=' => Space::new_space(SpaceType::Operator(OperatorType::Equals)),
                    '<' => Space::new_space(SpaceType::Operator(OperatorType::LessThan)),
                    '>' => Space::new_space(SpaceType::Operator(OperatorType::GreaterThan)),    
                    '!' => Space::new_space(SpaceType::Operator(OperatorType::Invert)),
                    ':' => Space::new_space(SpaceType::Operator(OperatorType::Duplicate)),
                    ' ' => Space::new_space(SpaceType::Conveyor(ConveyorType::Down)),
                    ',' => Space::new_space(SpaceType::Conveyor(ConveyorType::DoubleDown)),
                    '^' => Space::new_space(SpaceType::Conveyor(ConveyorType::Up)),
                    '\'' => Space::new_space(SpaceType::Conveyor(ConveyorType::DoubleUp)),
                    '}' => Space::new_space(SpaceType::Conveyor(ConveyorType::Right)),
                    ']' => Space::new_space(SpaceType::Conveyor(ConveyorType::DoubleRight)),
                    '{' => Space::new_space(SpaceType::Conveyor(ConveyorType::Left)),
                    '[' => Space::new_space(SpaceType::Conveyor(ConveyorType::DoubleLeft)),
                    '?' => Space::new_space(SpaceType::LogicalConveyor),
                    '@' => Space::new_space(SpaceType::Processor(ProcessorType::Print)),
                    '#' => Space::new_space(SpaceType::Processor(ProcessorType::Delete)),
                    '_' => Space::new_space(SpaceType::Wall),
                    _ => continue,
                };
                curr_line_spaces.push(next_space);
            }
            while (line_chars as i32) < width {
                curr_line_spaces.push(Space::new_space(SpaceType::Conveyor(ConveyorType::Down)));
                line_chars += 1;
            }
            grid.push(curr_line_spaces);
        }

        Program {
            grid,
            width,
            height,
            running: true,
            prev: None,
        }
    }

    pub fn run(&mut self) {
        while self.running {
            self.update_board();
        }
        println!("\nProgram has halted");
    }

    fn update_board(&mut self) {
        self.prev = Some(self.grid.clone());
        let mut updated: HashSet<(usize, usize)> = HashSet::new();
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                match self.grid[y][x].space_type {
                    SpaceType::Operator(_) | SpaceType::Processor(_) => {
                        if !updated.contains(&(y, x)) {
                            self.update_space(y, x, &mut updated);
                            updated.insert((y, x));
                        }
                    }
                    _ => {}
                }
            }
        }

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
            SpaceType::Conveyor(conveyor_type) => self.update_conveyor(y, x, updated, conveyor_type),
            SpaceType::LogicalConveyor => self.update_logical_conveyor(y, x, updated),
            SpaceType::Operator(operator_type) => self.update_operator(y, x, updated, operator_type),
            SpaceType::Processor(processor_type) => self.update_processor(y, x, processor_type),
            SpaceType::Wall => {},
        }
    }

    fn update_conveyor(&mut self, y: usize, x: usize, updated: &mut HashSet<(usize, usize)>, conveyor_type: ConveyorType) {

        let next_space: (i32, i32) = match conveyor_type {
            ConveyorType::Up => (y as i32 - 1, x as i32),
            ConveyorType::Down => (y as i32 + 1, x as i32),
            ConveyorType::Left => (y as i32, x as i32 - 1),
            ConveyorType::Right => (y as i32, x as i32 + 1),
            ConveyorType::DoubleUp => (y as i32 - 2, x as i32),
            ConveyorType::DoubleDown => (y as i32 + 2, x as i32),
            ConveyorType::DoubleLeft => (y as i32, x as i32 - 2),
            ConveyorType::DoubleRight => (y as i32, x as i32 + 2),
        };
        match self.grid[y][x].value {
            ValueType::Integer(_) | ValueType::Character(_)  | ValueType::HaltProgram => {
                if self.push_value(self.grid[y][x].value, next_space, updated) {
                    self.grid[y][x].value = ValueType::None;
                }
            }
            _ => {}
        }
    }

    fn update_logical_conveyor(&mut self, y: usize, x: usize, updated: &mut HashSet<(usize, usize)>) {
        if y == 0 || y == (self.height - 1) as usize {
            return;
        }
        let shift_left = match self.prev {
            Some(ref prev) => {
                match prev[y + 1][x].value {
                    ValueType::Integer(i) => i != 0,
                    ValueType::None => return,
                    _ => true,
                }
            }
            None => return,
        };

        let destination = if shift_left {(y as i32, (x as i32) - 1)} else {(y as i32, (x as i32) + 1)};
        
        if self.push_value(self.grid[y - 1][x].value, destination, updated) {
            self.grid[y - 1][x].value = ValueType::None;
        }
    }

    fn update_operator(&mut self, y: usize, x: usize, updated: &mut HashSet<(usize, usize)>, operator_type: OperatorType) {
        match operator_type {
            OperatorType::Invert | OperatorType::Duplicate => {
                if x == 0 {
                    return;
                }

                let left_value = match self.prev {
                    Some(ref prev) => {
                        match prev[y][x - 1].value {
                            ValueType::Integer(i) => i,
                            ValueType::Character(c) => c as i32,
                            _ => return,
                        }
                    }
                    None => return,
                };

                let left_space_temp = self.grid[y][x - 1].value;

                let result = match operator_type {
                    OperatorType::Invert => {
                        self.grid[y][x - 1].value = ValueType::None;
                        if left_value == 0 {
                            ValueType::Integer(1)
                        } else {
                            ValueType::Integer(0)
                        }
                    },
                    OperatorType::Duplicate => left_space_temp,
                    _ => unreachable!("Addition, Subtraction, Multiplication, Division, Modulus, And, Or, Equals, LessThan, GreaterThan should have been handled earlier"),
                };

                if self.push_value(result, (y as i32, (x + 1) as i32), updated) {
                    match operator_type {
                        OperatorType::Invert => self.grid[y][x - 1].value = ValueType::None,
                        OperatorType::Duplicate => self.grid[y][x - 1].value = left_space_temp,
                        _ => unreachable!("Addition, Subtraction, Multiplication, Division, Modulus, And, Or, Equals, LessThan, GreaterThan should have been handled earlier"),
                    }
                } else {
                    self.grid[y][x - 1].value = left_space_temp;
                }
            }
            _ => {
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

                let left_space_temp = self.grid[y][x - 1].value;
                let right_space_temp = self.grid[y][x + 1].value;
                self.grid[y][x - 1].value = ValueType::None;
                self.grid[y][x + 1].value = ValueType::None;

                let result = match operator_type {
                    OperatorType::Addition => left_value + right_value,
                    OperatorType::Subtraction => left_value - right_value,
                    OperatorType::Multiplication => left_value * right_value,
                    OperatorType::Division => left_value / right_value,
                    OperatorType::Modulus => left_value % right_value,
                    OperatorType::And => left_value & right_value,
                    OperatorType::Or => left_value | right_value,
                    OperatorType::Equals => if left_value == right_value {1} else {0},
                    OperatorType::LessThan => if left_value < right_value {1} else {0},
                    OperatorType::GreaterThan => if left_value > right_value {1} else {0},
                    _ => unreachable!("Invert and Duplicate should have been handled earlier"),
                };

                 if self.push_value(ValueType::Integer(result), ((y + 1) as i32, x as i32), updated) {
                    self.grid[y][x - 1].value = ValueType::None;
                    self.grid[y][x + 1].value = ValueType::None;
                 } else {
                    self.grid[y][x - 1].value = left_space_temp;
                    self.grid[y][x + 1].value = right_space_temp;
                 }
            }
        }
    }

    fn update_processor(&mut self, y: usize, x: usize, processor_type: ProcessorType) {
        match processor_type {
            ProcessorType::Print => {
                match self.grid[y][x].value {
                    ValueType::Integer(i) => print!("{i}"),
                    ValueType::Character(c) => print!("{c}"),
                    ValueType::HaltProgram => self.running = false,
                    _ => {}
                }
            }
            ProcessorType::Delete => {}
        }
        self.grid[y][x].value = ValueType::None
    }

    fn push_value(&mut self, new_value: ValueType, destination: (i32, i32), updated: &mut HashSet<(usize, usize)>) -> bool {
        let (dest_y, dest_x) = destination;
        if dest_y < 0 || dest_y >= self.height {
            return true;
        }
        if dest_x < 0 || dest_x >= self.width {
            return true;
        }

        match self.grid[dest_y as usize][dest_x as usize].value {
            ValueType::None => {
                self.update_space(dest_y as usize, dest_x as usize, updated);
                self.grid[dest_y as usize][dest_x as usize].value = new_value;
                updated.insert((dest_y as usize, dest_x as usize));
                return true;
            }
            ValueType::Character(_) | ValueType::Integer(_) | ValueType::HaltProgram => {
                if !updated.contains(&(dest_y as usize, dest_x as usize)) {
                    self.update_space(dest_y as usize, dest_x as usize, updated);
                    match self.grid[dest_y as usize][dest_x as usize].value {
                        ValueType::None => {
                            self.grid[dest_y as usize][dest_x as usize].value = new_value;
                            updated.insert((dest_y as usize, dest_x as usize));
                            return true;
                        }
                        _ => {
                            return false;
                        }
                    }
                }
            }
            ValueType::CannotHoldValue => {
                return false;
            }
        }

        false
    }

    fn print_board(&self) {
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                match self.grid[y][x].value {
                    ValueType::Integer(i) => {
                        if i < 10 && i >= 0 {
                            print!("{i}");
                        } else {
                            print!("n");
                        }
                    },
                    ValueType::Character(c) => {
                        if c == '\n' {
                            print!("\\");
                        } else {
                            print!("{c}");
                        }
                    },
                    ValueType::HaltProgram => print!(";"),
                    _ => {
                        match self.grid[y][x].space_type {
                            SpaceType::Conveyor(conveyor_type) => {
                                match conveyor_type {
                                    ConveyorType::Up => print!("^"),
                                    ConveyorType::Down => print!(" "),
                                    ConveyorType::Left => print!("<"),
                                    ConveyorType::Right => print!(">"),
                                    ConveyorType::DoubleUp => print!("\""),
                                    ConveyorType::DoubleDown => print!("\'"),
                                    ConveyorType::DoubleLeft => print!("["),
                                    ConveyorType::DoubleRight => print!("]"),
                                }
                            }
                            SpaceType::LogicalConveyor => print!("?"),
                            SpaceType::Operator(operator_type) => {
                                match operator_type {
                                    OperatorType::Addition => print!("+"),
                                    OperatorType::Subtraction => print!("-"),
                                    OperatorType::Multiplication => print!("*"),
                                    OperatorType::Division => print!("/"),
                                    OperatorType::Modulus => print!("%"),
                                    OperatorType::And => print!("&"),
                                    OperatorType::Or => print!("|"),
                                    OperatorType::Equals => print!("="),
                                    OperatorType::LessThan => print!("<"),
                                    OperatorType::GreaterThan => print!(">"),
                                    OperatorType::Invert => print!("!"),
                                    OperatorType::Duplicate => print!(":"),
                                }
                            }
                            SpaceType::Processor(processor_type) => {
                                match processor_type {
                                    ProcessorType::Print => print!("@"),
                                    ProcessorType::Delete => print!("#"),
                                }
                            }
                            SpaceType::Wall => print!("_"),
                        }
                    }
                }
            }
            println!();
        }
    }
}

#[derive(Clone, Copy)]
pub struct Space {
    pub value: ValueType,
    pub space_type: SpaceType,
}

impl Space {
    fn new_value(value: ValueType) -> Self {
        Self {
            value,
            space_type: SpaceType::Conveyor(ConveyorType::Down),
        }
    }

    fn new_space(space_type: SpaceType) -> Self {
        Self {
            value: match space_type {
                SpaceType::Conveyor(_) | SpaceType::Processor(_) => ValueType::None,
                _ => ValueType::CannotHoldValue,
            },
            space_type,
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    Invert,
    Duplicate,
}

#[derive(Clone, Copy)]
pub enum ProcessorType {
    Print,
    Delete,
}