extern crate arguments;
extern crate temporary;

use std::collections::HashMap;
use std::fmt::Display;

const USAGE: &'static str = "
Usage: cite [options]

Options:
    --bib <FILE>       A bibliography file.
    --ref <NAME>       A reference name.

    --help             Display this message.
";

macro_rules! ok(
    ($result:expr) => (match $result {
        Ok(result) => result,
        Err(error) => raise!(error),
    });
);

macro_rules! raise(
    ($error:expr) => (return Err(Box::new($error)));
    ($($arg:tt)*) => (raise!(format!($($arg)*)));
);

pub type Error = Box<Display>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() {
    start().unwrap_or_else(|error| fail(error));
}

fn start() -> Result<()> {
    let arguments = match arguments::parse(std::env::args()) {
        Ok(arguments) => arguments,
        Err(error) => raise!(error),
    };

    if arguments.get::<bool>("help").unwrap_or(false) {
        help();
    }

    let mut map = HashMap::new();
    map.insert("<bibliography>", "Bibliography!");
    map.insert("<citation>", "Citation!");

    println!("{}", replace(LATEX_TEMPLATE.trim(), &map));

    Ok(())
}

fn help() -> ! {
    println!("{}", USAGE.trim());
    std::process::exit(0);
}

fn fail(error: Error) -> ! {
    use std::io::{stderr, Write};
    let message = format!("Error: {}.\n{}", &*error, USAGE);
    stderr().write_all(message.as_bytes()).unwrap();
    std::process::exit(1);
}

fn replace(text: &str, map: &HashMap<&str, &str>) -> String {
    let mut text = text.to_string();
    for (key, value) in map {
        text = text.replace(key, value);
    }
    text
}

const LATEX_TEMPLATE: &'static str = r#"
\documentclass[journal]{IEEEtran}
\pagestyle{empty}
\renewcommand{\refname}{}

\begin{filecontents}{bibliography.bib}
<bibliography>
\end{filecontents}

\begin{document}
\nocite{<citation>}
\bibliography{IEEEabrv,bibliography.bib}
\bibliographystyle{IEEEtran}
\end{document}
"#;
