use colored::Colorize;

pub fn spawn_pizza() {
    println!("{}", r#"            __   
            ""                              
                                               
8b,dPPYba,  88 888888888 888888888 ,adPPYYba,  
88P'    "8a 88      a8P"      a8P" ""     `Y8  
88       d8 88   ,d8P'     ,d8P'   ,adPPPPP88  
88b,   ,a8" 88 ,d8"      ,d8"      88,    ,88  
88`YbbdP"'  88 888888888 888888888 `"8bbdP"Y8  
88                                             
88         "#.color(colored::Color::TrueColor {
    r: 255, g: 200, b: 50
}));
    println!("\nThanks for being a bug tester :)")
}