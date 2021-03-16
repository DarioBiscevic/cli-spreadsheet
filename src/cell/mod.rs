use std::fmt;

///Holds the name, coordinates, expression and value of a cell; its data is then used for
///computation or for saving the data inside a file.
#[derive(Debug)]
pub struct Cell{
    pub name:       Option<String>,
    pub coords:     (usize, usize),
    pub expression: String,
    pub value:      ValueType
}

impl Cell{
    pub fn new(name: Option<String>, coords: (usize, usize)) -> Self{
        Cell{
            name,
            coords,
            expression: String::new(),
            value:      ValueType::Empty
        }
    }

    pub fn set_expr(&mut self, expression: &str){
        self.expression = String::from(expression);
    }

    pub fn set_name(&mut self, name: &str){
        self.name = Some(String::from(name));
    }
}


/*
A cell's value can be of three types only:
    - Literal: the output value is text
    - Numeric: the output value is a number
    - Empty:   the output value doesn't actually exist (not yet evaluated/empty cell)

*/
#[derive(Debug)]
pub enum ValueType{
    Literal(String),
    Numeric(f64),
    Empty
}

//Display implementation of ValueType
impl std::fmt::Display for ValueType{

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self{
            ValueType::Literal(text)   => write!(f, "{}", text),
            ValueType::Numeric(number) => write!(f, "{}", number),
            ValueType::Empty           => write!(f, "{}", 0)
        }
    }
}
