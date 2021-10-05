pub fn error(line: usize, message: &str) {
    report(line, "", message);
}

fn report(line: usize, ctx: &str, message: &str) {
    // TODO use this https://github.com/brendanzab/codespan
    dbg!(line, ctx, message);
}
