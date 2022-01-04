use codespan_reporting::{
    diagnostic::{Diagnostic, Label},
    files::SimpleFile,
    term::{
        self,
        termcolor::{ColorChoice, StandardStream},
        Config,
    },
};

#[derive(Debug)]
pub enum CompileError {
    Parser((usize, usize), String),
    Scanner((usize, usize), String),
    Interpreter((usize, usize), String),
}

pub fn error(file_name: &str, source: &str, errors: &[CompileError]) {
    let file = SimpleFile::new(file_name, source);
    let writer = StandardStream::stderr(ColorChoice::Always);

    for e in errors {
        let diagnostic = match e {
            CompileError::Scanner(span, msg) => Diagnostic::error()
                .with_message(format!("Error while scanning: {msg}"))
                .with_labels(vec![Label::primary((), span.0..span.1)]),

            CompileError::Parser(span, msg) => Diagnostic::error()
                .with_message(format!("Error while parsing: {msg}"))
                .with_labels(vec![Label::primary((), span.0..span.1).with_message("here")]),

            CompileError::Interpreter(span, msg) => Diagnostic::error()
                .with_message(format!("Runtime error: {msg}"))
                .with_labels(vec![Label::primary((), span.0..span.1)]),
        };

        term::emit(&mut writer.lock(), &Config::default(), &file, &diagnostic).unwrap();
    }
}
