// pub fn parse_identifier() {

// }

use crate::compiler::parser::{Expression, Node, NodeInfo};

use super::{
    super::super::lexer::{Token, Tokens},
    arguments::parse_arguments,
    variable::parse_set_variable,
};
impl Tokens {
    pub fn parse_identifer(&mut self) -> String {
        let info = self.advance();

        let token = match &info.token {
            Token::Identifier(string) => return string.clone(),
            token => token.clone(),
        };

        self.throw_error(
            format!("Expected identifier, found '{}'", token),
            "expected identifier",
        )
    }
}

pub fn parse_after_identifier(tokens: &mut Tokens, name: String) -> NodeInfo {
    let info = tokens.expect_tokens(vec![Token::OpenParen, Token::Equals], false);

    let node = match info.token {
        Token::OpenParen => {
            let arguments = parse_arguments(tokens);
            tokens.create_node(Node::Call(name, arguments))
        }
        Token::Equals => parse_set_variable(tokens, name),
        _ => panic!(),
    };

    return node;
}
