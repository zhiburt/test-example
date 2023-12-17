use std::{collections::HashSet, io::Result};

use super::{Code, Target};

pub fn parse<R>(mut file: R) -> Result<Vec<Code>>
where
    R: std::io::Read,
{
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    let tokens = markdown::tokenize(&buf);

    let mut examples = vec![];

    for token in tokens {
        let (head, block) = match token {
            markdown::Block::CodeBlock(Some(head), block) => (head, block),
            _ => continue,
        };

        if !head.starts_with("rust") {
            continue;
        }

        let head = &head[4..];
        let opts = head.split(',').collect::<HashSet<_>>();

        if opts.contains("ignore") {
            continue;
        }

        let compile = if head.contains("no_run") {
            Target::Build
        } else {
            Target::Run
        };

        let code = Code::new(block, vec![], compile);

        examples.push(code);
    }

    Ok(examples)
}
