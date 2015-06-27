extern crate arguments;
extern crate temporary;

use std::collections::HashMap;
use std::fmt::Display;
use std::fs::File;
use std::io::{Read, Write};
use std::process::Command;
use temporary::Directory;

const USAGE: &'static str = "
Usage: cite [options]

Options:
    --bib <FILE>       A bibliography file (required).
    --ref <NAME>       A reference name (required).

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

    let bibliography = match arguments.get::<String>("bib") {
        Some(bibliography) => bibliography,
        _ => raise!("a bibliography file is required"),
    };
    let reference = match arguments.get::<String>("ref") {
        Some(reference) => reference,
        _ => raise!("a reference name is required"),
    };

    process(TEMPLATE.trim(), &bibliography, &reference)
}

fn process(template: &str, bibliography: &str, reference: &str) -> Result<()> {
    let root = ok!(Directory::new("cite"));

    macro_rules! cmd(
        ($command:expr) => (Command::new($command));
    );
    macro_rules! run(
        ($command:expr) => ({
            let status = ok!($command.current_dir(&root).status());
            if !status.success() {
                raise!(status);
            }
        });
    );

    {
        let mut map = HashMap::new();
        map.insert("<bibliography>", bibliography);
        map.insert("<reference>", reference);
        let tex = replace(template, &map);

        let mut file = ok!(File::create(root.join("paper.tex")));
        ok!(file.write_all(tex.as_bytes()));
    }

    run!(cmd!("latex").arg("paper.tex"));
    run!(cmd!("bibtex").arg("paper"));
    run!(cmd!("latex").arg("paper.tex"));
    run!(cmd!("latex").arg("paper.tex"));
    run!(cmd!("dvipdf").arg("paper.dvi"));
    run!(cmd!("pdftotext").arg("paper.pdf"));

    let mut buffer = Vec::new();
    {
        let mut file = ok!(File::open(root.join("paper.txt")));
        ok!(file.read_to_end(&mut buffer));
    }

    let content = String::from_utf8_lossy(&buffer);
    println!("{}", content.trim());

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

const TEMPLATE: &'static str = r#"
\documentclass[journal]{IEEEtran}
\pagestyle{empty}
\renewcommand{\refname}{}

\begin{document}
\nocite{<reference>}
\bibliography{IEEEabrv,<bibliography>}
\bibliographystyle{IEEEtran}
\end{document}
"#;
