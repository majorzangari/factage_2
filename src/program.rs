use std::collections::HashSet;

pub struct Program {
    pub grid: Vec<Vec<Space>>,
    pub width: i32,
    pub height: i32,
}

impl Program {
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
        match self.grid[y][x].space_type {
            SpaceType::Conveyor(conveyor_type) => {
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
                self.push_value(self.grid[y][x].value, next_space);
                //TODO: confirm push, remove values
            }
            SpaceType::LogicalConveyor => {
                if y == 0 || y == (self.height - 1) as usize {
                    return;
                }

                let shift_left = match self.grid[y + 1][x].value {
                    ValueType::Integer(i) => i != 0,
                    ValueType::None => return,
                    _ => true,
                };

                let destination = if shift_left {(y, x - 1)} else {(y, x + 1)};
                self.push_value(self.grid[y][x].value, destination);
                //TODO: confirm push, remove values
            }
            SpaceType::Operator(operator_type) => {
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

                self.push_value(ValueType::Integer(result), (y + 1, x));
                //TODO: confirm push, remove values
            }
            SpaceType::Processor(processor_type) => {
                match processor_type {
                    ProcessorType::Delete => self.grid[y][x].value = ValueType::None,
                    ProcessorType::Print => {
                        match self.grid[y][x].value {
                            ValueType::Integer(i) => print!("{i}"),
                            ValueType::Character(c) => print!("{c}"),
                            ValueType::HaltProgram => todo!("stop program at this point"),
                            ValueType::None => {}
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn push_value(&mut self, value: ValueType, destination: (usize, usize)) -> bool {
        todo!("moving value code");
    }
}

#[derive(Clone, Copy)]
pub struct Space {
    pub value: ValueType,
    pub space_type: SpaceType,
}

#[derive(Clone, Copy)]
pub enum ValueType {
    None,
    HaltProgram,
    Integer(i32),
    Character(char),
}

#[derive(Clone, Copy)]
pub enum SpaceType {
    Empty,
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