mod lexer;

fn main() {
    let code = String::from("# I'm just a comment
+-{}() ++ -- \"ameer\"  \"jhan\" 123.258 0.2 \"jhan\"
first_variable = 3.14
");
    let mut lex = lexer::Lexer::new();
    
    let tokens = lex.lex(&code);

    println!("{:?}", tokens);
}
