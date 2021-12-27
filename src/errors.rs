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
    Parser(usize, usize, &'static str),
    Scannner(usize, usize, &'static str),
    Interpreter(usize, usize, &'static str),
}

pub fn error(file_name: &str, source: &str, error: &[CompileError]) {
    report(file_name, source, error);
}

pub fn report(file_name: &str, source: &str, errors: &[CompileError]) {
    let file = SimpleFile::new(file_name, source);
    let writer = StandardStream::stderr(ColorChoice::Always);

    for e in errors {
        let diagnostic = match e {
            CompileError::Scannner(l, r, msg) => Diagnostic::error()
                .with_message(*msg)
                .with_labels(vec![Label::primary((), *l..*r)]),

            CompileError::Parser(l, r, msg) => Diagnostic::error()
                .with_message(*msg)
                .with_labels(vec![Label::primary((), *l..*r).with_message("here")]),

            CompileError::Interpreter(l, r, msg) => Diagnostic::error()
                .with_message(*msg)
                .with_labels(vec![Label::primary((), *l..*r)]),
        };

        term::emit(&mut writer.lock(), &Config::default(), &file, &diagnostic).unwrap();
    }
}
