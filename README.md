# Cite [![Version][version-img]][version-url] [![Status][status-img]][status-url]

The package provides a tool for formatting citations.

## Requirements

The tool relies on the following commands:

* [bibtex][1],
* [dvipdf][2],
* [latex][3], and
* [pdftotext][4].

## Usage

```
Usage: cite [options]

Options:
    --bib <FILE>       A bibliography file. If unspecified, the content is read
                       from the standard input.

    --ref <NAME>       A reference name. If unspecified, the first found
                       reference is taken.

    --tex <FILE>       A template file. If unspecified, the built-in template is
                       used, which is based on IEEE’s journal document style.

    --help             Display this message.
```

## Contributing

1. Fork the project.
2. Implement your idea.
3. Open a pull request.

[1]: http://www.bibtex.org
[2]: http://linux.die.net/man/1/dvipdf
[3]: http://www.latex-project.org
[4]: http://www.foolabs.com/xpdf

[version-img]: https://img.shields.io/crates/v/cite.svg
[version-url]: https://crates.io/crates/cite
[status-img]: https://travis-ci.org/IvanUkhov/cite.svg?branch=master
[status-url]: https://travis-ci.org/IvanUkhov/cite
