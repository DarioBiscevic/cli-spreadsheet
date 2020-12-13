pub struct Cell{
    expression: String,
    value:      ValueType
}

impl Cell{
    pub fn new() -> Self{
        Cell{
            expression: String::new(),
            value:      ValueType::Empty
        }
    }
}

enum ValueType{
    Literal(String),
    Numeric(f64),
    Empty
}