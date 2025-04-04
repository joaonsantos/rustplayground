use std::process::exit;

use yafp::Parser;

pub struct Args {
    pub pattern: String,
    pub number: bool,
    pub context: (u32, u32),
}

pub fn parse_args() -> Args {
    let mut parser = Parser::from_env();
    parser.bool_flag(
        "number",
        "this is used to determine if the line number should be printed",
    );
    parser.optional_flag(
        "before-context",
        "number of lines to include before the match",
    );
    parser.optional_flag(
        "after-context",
        "number of lines to include after the match",
    );
    parser.optional_flag(
        "context",
        "number of lines to include before and after the match",
    );

    // Check if there is an error parsing.
    let result = parser.finalize();
    let remaining = match result {
        Ok(remaining) => remaining,
        Err(e) => {
            println!("{}: {}", parser.command, e);
            exit(1);
        }
    };

    let pattern_result = remaining.iter().next();
    let pattern = match pattern_result {
        Some(pat) => pat.clone(),
        None => {
            println!("{}", parser.help());
            exit(1);
        }
    };

    let before_context = parser.get_value("before-context").unwrap_or(0);
    let after_context = parser.get_value("after-context").unwrap_or(0);
    let context = match parser.get_value("context") {
        Some(c) => (c, c),
        None => (before_context, after_context),
    };
    Args {
        pattern,
        number: parser.get_value("number").unwrap_or_default(),
        context,
    }
}
