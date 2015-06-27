extern crate arguments;
extern crate temporary;

use std::collections::HashMap;
use std::fmt::Display;
use std::fs::File;
use std::io::{stderr, stdin, stdout, Read, Write};
use std::path::Path;
use std::process::Command;
use temporary::Directory;

const USAGE: &'static str = "
Usage: cite [options]

Options:
    --bib <FILE>       A bibliography file.
    --ref <NAME>       A reference name.
    --tex <FILE>       A template file.

    --help             Display this message.
";

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

    let root = ok!(Directory::new("cite"));

    let bibliography = match arguments.get::<String>("bib") {
        Some(bibliography) => bibliography,
        _ => try!(create_bibliography(&root)),
    };
    let reference = match arguments.get::<String>("ref") {
        Some(reference) => reference,
        _ => raise!("a reference name is required"),
    };

    match arguments.get::<String>("tex") {
        Some(ref template) => {
            let mut file = ok!(File::open(template));
            let mut template = String::new();
            ok!(file.read_to_string(&mut template));
            process(&template, &bibliography, &reference, &root)
        },
        _ => {
            process(TEMPLATE, &bibliography, &reference, &root)
        },
    }
}

fn process(template: &str, bibliography: &str, reference: &str, root: &Path) -> Result<()> {
    macro_rules! run(
        ($program:expr, $argument:expr) => ({
            let mut command = Command::new($program);
            command.arg($argument).current_dir(&root);
            let output = match command.output() {
                Ok(output) => output,
                Err(error) => raise!(format!("`{}` has failed: {}", $program, error)),
            };
            if !output.status.success() {
                let _ = stdout().write_all(&output.stdout);
                let _ = stderr().write_all(&output.stderr);
                raise!(format!("`{}` has failed", $program));
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

    run!("latex", "paper.tex");
    run!("bibtex", "paper");
    run!("latex", "paper.tex");
    run!("latex", "paper.tex");
    run!("dvipdf", "paper.dvi");
    run!("pdftotext", "paper.pdf");

    let mut buffer = Vec::new();
    {
        let mut file = ok!(File::open(root.join("paper.txt")));
        ok!(file.read_to_end(&mut buffer));
    }

    let content = String::from_utf8_lossy(&buffer);
    println!("{}", content.trim());

    Ok(())
}

fn create_bibliography(root: &Path) -> Result<String> {
    println!("Paste a bibliography content and press Ctrl-D:");
    let mut content = String::new();
    ok!(stdin().read_to_string(&mut content));

    let mut file = ok!(File::create(root.join("paper.bib")));
    ok!(file.write_all(content.as_bytes()));

    Ok(String::from("paper.bib"))
}

fn replace(text: &str, map: &HashMap<&str, &str>) -> String {
    let mut text = text.to_string();
    for (key, value) in map {
        text = text.replace(key, value);
    }
    text
}

fn help() -> ! {
    println!("{}", USAGE.trim());
    std::process::exit(0);
}

fn fail(error: Error) -> ! {
    let message = format!("Error: {}.\n{}", &*error, USAGE);
    let _ = stderr().write_all(message.as_bytes());
    std::process::exit(1);
}
