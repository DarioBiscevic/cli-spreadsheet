use std::fmt;

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

impl std::fmt::Display for ValueType{

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        match self{
            ValueType::Literal(text)   => write!(f, "{}", text),
            ValueType::Numeric(number) => write!(f, "{}", number),
            ValueType::Empty           => write!(f, "{}", 0)
        }
    }
}