pub struct Cell{
    expression: String,
}

impl Cell{
    pub fn new() -> Self{
        Cell{
            expression: String::new()
        }
    }
}