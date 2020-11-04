fn main() {
    println!("Hello, world!");

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("ffffffffffffffff")),
        SpreadsheetCell::Float(2.22),
    ];
    
    for i in &row {
        match i {
            SpreadsheetCell::Int(val) => {
                println!("Int : {}", val);
            },
            SpreadsheetCell::Float(val) => {  
                println!("Float : {}", val);
            },
            SpreadsheetCell::Text(val) => {
                println!("Text : {}", val);
            }   
        }
    }
}
enum SpreadsheetCell  { 
    Int(i32),
    Float(f64),
    Text(String),
}