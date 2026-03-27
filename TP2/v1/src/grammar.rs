use santiago::grammar::Grammar;

#[derive(Debug, PartialEq)]
pub enum AST {
    Empty,
    Program(Vec<AST>),
    Command(Vec<AST>),
    Order(Vec<AST>),
    Forward,
    Backward,
    Left,
    Right,
    Number(i32),
}

pub fn grammar() -> Grammar<AST> {
    santiago::grammar!(
        //règles pour commande et programme
        "program" => rules "command" "program" =>AST::Program;
        "program" => empty => |_|AST::Empty;
        
        "command" => rules "order" "num"=> AST::Command;

        //règles pour les ordres
        "order" => lexemes "FORWARD" => |_lexemes|AST::Order(vec![AST::Forward]);
        "order" => lexemes "BACKWARD"=> |_lexemes|AST::Order(vec![AST::Backward]);
        "order" => lexemes "LEFT"    => |_lexemes|AST::Order(vec![AST::Left]);
        "order" => lexemes "RIGHT"   => |_lexemes|AST::Order(vec![AST::Right]);

        "num" => lexemes "number"=> |lexemes| {
            let value = str::parse::<i32>(&lexemes[0].raw).unwrap();
            AST::Number(value)
        };
    )
}

pub fn eval(value: &AST) -> () {
    match value {
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
