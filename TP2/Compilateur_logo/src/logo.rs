pub struct Logo {
    position: (f64, f64),
    orientation: f64,
    isWriting: bool,
    svgFile: String,
}

impl compile(&mut self, ast, ast: &AST) -> String {
    let mut logo = Logo {
        position: (0.0, 0.0),
        orientation: 0.0,
        isWriting: false,
        svgFile: BeginSvg{w: 300.0, h: 200.0}.to_string(), // Initialisation du SVG
    };

    match commands {
        AST::Empty => println!("Stop."),
        AST::Program(commands) => {
            for command in commands {
                eval(command);
            }
        }
        AST::Command(order) => {
            for o in order {
                eval(o);
            }
            println!("units.")
        }
        AST::Order(orders) => {
            for order in orders {
                eval(order);
            }        
        }
        AST::Number(num) => {
            print!("{} ", num);
        }
        AST::Forward => print!("Moving forward "),
        AST::Backward => print!("Moving backward "),
        AST::Left => print!("Turning left "),
        AST::Right => print!("Turning right "),
    }

}
    