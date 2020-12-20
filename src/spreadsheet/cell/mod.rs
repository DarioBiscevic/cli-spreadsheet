#[derive(Debug)]
pub struct Cell{
    pub coords:     (usize, usize),
    pub expression: String,
    pub value:      ValueType
}

impl Cell{
    pub fn new(coords: (usize, usize)) -> Self{
        Cell{
            coords,
            expression: String::new(),
            value:      ValueType::Empty
        }
    }

    pub fn set(&mut self, expression: &str){
        self.expression = String::from(expression);
    }
}

#[derive(Debug)]
pub enum ValueType{
    Literal(String),
    Numeric(f64),
    Empty
}