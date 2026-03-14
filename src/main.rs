use std::io;
use std::process::ExitCode;

use rs_emoji_fromcode::stdin2codes2emojis2stdout;

fn sub() -> Result<(), io::Error> {
    stdin2codes2emojis2stdout()
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
