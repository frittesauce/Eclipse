use crate::compiler::{
    lexer::{Token, Tokens},
    parser::{Node, NodeInfo},
};

use super::{expression::parse_expression, types::parse_type};

pub fn parse_variable(tokens: &mut Tokens) -> NodeInfo {
    let mutable = tokens.peek_expect_token(Token::Mutable, true);
    let name = tokens.parse_identifer();

    let data_type = if tokens.peek_expect_token(Token::Colon, true) {
        Some(parse_type(tokens))
    } else {
        None
    };

    tokens.expect_tokens(vec![Token::Equals], false);
    let expression = parse_expression(tokens, true).unwrap();

    tokens.create_node(Node::Variable {
        name,
        mutable,
        data_type,
        expression,
    })
}

pub fn parse_set_variable(tokens: &mut Tokens, name: String) -> NodeInfo {
    let expression = parse_expression(tokens, true).unwrap();
    tokens.create_node(Node::SetVariable { name, expression })
}