pub enum ValueType {
    None,
    Integer(i32),
    Character(i32),
}

pub enum SpaceType {
    Conveyor(ConveyorType),
    Empty,
    LogicalConveyor,
    Operator,
    Processor,
    Wall,
}

pub enum ConveyorType {
    Up,
    Down,
    Left,
    Right,
}

pub enum OperatorType {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    And,
    Or,
    Equals,
    LessThan,
    GreaterThan,
}

pub struct Space {
    pub value: ValueType,
    pub square_type: SquareType,
}

impl Space {
    fn update_space(&self) {
        match square_type {
            SpaceType::Conveyor(conveyor_type) => {
                match conveyor_type {
                    
                }
            }
            SpaceType::LogicalConveyor => {

            }
            SpaceType::Operator => {

            }
            SpaceType::Processor => {

            }
        }
    }
}