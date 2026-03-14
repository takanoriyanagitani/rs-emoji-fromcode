use std::io;

use io::BufWriter;
use io::Write;

use io::BufRead;

use emojis::Emoji;

pub fn code2emoji(code: &str) -> Option<&'static Emoji> {
    emojis::get_by_shortcode(code)
}

pub fn codes2emojis2writer<I, W>(codes: I, mut wtr: W) -> Result<(), io::Error>
where
    I: Iterator<Item = Result<String, io::Error>>,
    W: Write,
{
    for rline in codes {
        let line: String = rline?;
        let oe: Option<&Emoji> = code2emoji(&line);
        if let Some(e) = oe {
            writeln!(&mut wtr, "{e}")?;
        }
    }
    wtr.flush()
}

pub fn stdin2codes2emojis2stdout() -> Result<(), io::Error> {
    let il = io::stdin().lock();
    let lines = il.lines();

    let o = io::stdout();
    let mut ol = o.lock();

    codes2emojis2writer(lines, BufWriter::new(&mut ol))?;

    ol.flush()
}
