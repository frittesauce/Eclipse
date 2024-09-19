use crate::{
    lexer::{Token, TokensGroup}, BuildError
};

use super::{
    after_identifier::parse_identifer_string, tokens_expected_got, types::parse_type, ASTNode, Node,
};

pub fn parse_enum(tokens: &mut TokensGroup, export: bool) -> Result<ASTNode, BuildError> {
    let name = parse_identifer_string(tokens)?;

    match tokens.advance() {
        Ok(info) => match info.token {
            Token::StartScope => {}
            _ => return Err(tokens_expected_got(tokens, vec![Token::StartScope], info)),
        },
        Err(error) => return Err(error),
    };

    let mut body = Vec::new();

    loop {
        let info = tokens.advance()?;
        let name = match info.token {
            Token::EndScope => break,
            Token::Comma => continue,
            Token::Identifier(name) => name,
            _ => {
                return Err(tokens_expected_got(
                    tokens,
                    vec![Token::Identifier(String::from("enum"))],
                    info,
                ))
            }
        };
        let mut types = Vec::new();

        let info = tokens.peek()?;
        match info.token {
            Token::OpenParen => {
                tokens.advance()?;
                loop {
                    types.push(parse_type(tokens)?);
                    let info = tokens.advance()?;
                    match info.token {
                        Token::Comma => continue,
                        Token::CloseParen => break,
                        _ => return Err(tokens_expected_got(tokens, vec![Token::Comma], info)),
                    }
                }
            }
            _ => {}
        }

        body.push((name, types))
    }

    return Ok(tokens.generate(Node::Enum {
        export,
        name,
        generics: Vec::new(),
        body: body,
    })?);
}
