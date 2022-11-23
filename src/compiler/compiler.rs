use crate::compiler::lexer;

pub fn compile() {
    let mut lexer = lexer::Lexer::new("test.sam");

    loop {
        let token: lexer::Token = lexer.get_token();
        print!("token_type:({})", token.token_type);
        println!(" line..{}", token.line_number);
        if token.token_type == lexer::TokenType::EndOfFile {
            break;
        }
    }    
}