use logos::Logos;

#[derive(Default, Debug, PartialEq, Clone)]
pub enum LexingError {
    #[default]
    UnexpectedToken,
    ExpectedToken,
}

#[derive(Logos, Debug, PartialEq)]
#[logos(extras = LexingError)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    #[regex("//[^\n]*|/\\*([^*]|\\*[^/])*\\*/")]
    Comment,

    #[token("let")]
    Let,

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifer,

    #[token("=")]
    Assign,

    #[regex("[0-9]+")]
    Number,

    #[regex(r#""[^"]*""#)]
    String,

    #[regex("true|false")]
    Boolean,
    
    #[token("(")]
    LParen,

    #[token(")")]
    RParen,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asing_str() {
        let mut lexer = Token::lexer("let x = \"test\"");
        assert_eq!(lexer.next(), Some(Ok(Token::Let)));
        assert_eq!(lexer.next(), Some(Ok(Token::Identifer)));
        assert_eq!(lexer.next(), Some(Ok(Token::Assign)));
        assert_eq!(lexer.next(), Some(Ok(Token::String)));
    }

    #[test]
    fn asing_number() {
        let mut lexer = Token::lexer("let x = 10");
        assert_eq!(lexer.next(), Some(Ok(Token::Let)));
        assert_eq!(lexer.next(), Some(Ok(Token::Identifer)));
        assert_eq!(lexer.next(), Some(Ok(Token::Assign)));
        assert_eq!(lexer.next(), Some(Ok(Token::Number)));
    }

    #[test]
    fn expr() {
        let mut lexer = Token::lexer("print(\"Hello, world!\")");
        
        for token in lexer {
            println!("{:?}", token);
        }

    }
}