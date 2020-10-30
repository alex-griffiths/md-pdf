enum HeadingLevel {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

enum CodeKind {
    Inline,
    Block,
}

enum TokenKind {
    Heading(HeadingLevel),
    StringVal,
    Code {
        kind: CodeKind
    },
    Literal {
        kind: LiteralKind,
    },
}

enum LiteralKind {
    Int,
    Float,
    Char {
        terminated: bool,
    },
}

pub struct Token {
    kind: TokenKind,
    literal: String,
}

pub fn lex(stream: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    for line in stream.lines() {
        let line = line.to_string();
        let word_vec: Vec<_> = line.split(' ').collect();

        for word in word_vec {
            println!("{}", word);
        }
    }

    tokens
}
