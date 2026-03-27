mod lexer;
mod grammar;
use crate::grammar::eval;



fn main() {

    let input = "forward 10 right 90 backward 5 left 45";
    let rules = lexer::lexer_rules();
    let lexemes = santiago::lexer::lex(&rules, &input).unwrap();
    
    println!("lexmes :{:#?}", lexemes); //Affichage des lexemes

    let grammar = grammar::grammar();
    let parse_trees = &santiago::parser::parse(&grammar, &lexemes).expect("syntax error")[0];

    println!("parse tree :{:#?}", parse_trees); //Affichage du parse tree

    println!("AST :{:#?}", parse_trees.as_abstract_syntax_tree()); //Affichage de l'AST

    eval(&parse_trees.as_abstract_syntax_tree()); //Affichage de l'évaluation de l'AST
}
