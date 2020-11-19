#[derive(Debug)]
enum HeadingLevel {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

#[derive(Debug)]
enum CodeKind {
    Inline,
    Block,
}

#[derive(Debug)]
enum TokenKind {
    Heading(HeadingLevel),
    StringVal,
    NewLine,
    Code { kind: CodeKind },
    Blockquote,
    Literal { kind: LiteralKind },
}

#[derive(Debug)]
enum LiteralKind {
    Int,
    Float,
    Char { terminated: bool },
}

#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    literal: String,
}

/// Accepts a file as a stream and parses it to build a vector of tokens
pub fn lex(stream: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    // Split stream into lines and iterate over them
    for line in stream.lines() {
        // Split line into words
        let line = line.to_string();
        let word_vec: Vec<_> = line.split(' ').collect();

        // Iterate over words and tokenise as necessary
        for word in word_vec {
            // Match word to tokens
            let token_kind = match word {
                "#" => TokenKind::Heading(HeadingLevel::H1),
                "##" => TokenKind::Heading(HeadingLevel::H2),
                "###" => TokenKind::Heading(HeadingLevel::H3),
                "####" => TokenKind::Heading(HeadingLevel::H4),
                "#####" => TokenKind::Heading(HeadingLevel::H5),
                "######" => TokenKind::Heading(HeadingLevel::H6),
                ">" => TokenKind::Blockquote,
                "```" => TokenKind::Code {
                    kind: CodeKind::Block,
                },
                _ => {
                    TokenKind::StringVal
                }
            };

            let token = Token {
                kind: token_kind,
                literal: word.to_string(),
            };

            // Save the parsed token.
            tokens.push(token);
        }

        // Add new line token
        tokens.push(Token {
            kind: TokenKind::NewLine,
            literal: "\r\n".to_string(),
        });
    }

    tokens
}

