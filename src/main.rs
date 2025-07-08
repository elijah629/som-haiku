use som_haiku::process;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> std::io::Result<()> {
    let stdin = std::io::stdin();
    let reader = stdin.lock();

    let stdout = std::io::stdout();
    let writer = std::io::BufWriter::new(stdout.lock());

    process(reader, writer)

    /*for line in handle.lines() {
        let line_content = line?;

        let strwords = format(&line_content);
        let words = strwords.split_ascii_whitespace().map(est::estimate);

        if is_haiku(words) {
            writeln!(writer, "{line_content}")?;
        }
        /*for (left, right) in compute_haiku_padding(sum, &words) {
            for _ in 0..left {
                write!(writer, "a ")?;
            }

            // Push main content
            write!(writer, "{}", &line_content)?;

            for _ in 0..right {
                write!(writer, " a")?;
            }

            writeln!(writer)?;
        }*/
    }

    writer.flush()?;

    Ok(())*/
}
