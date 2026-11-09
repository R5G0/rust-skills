//! Buffering workloads and the original rule's two different line contracts.

use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};

/// Count newline bytes with one-byte reads, as in the rule's slow example.
///
/// This accepts arbitrary bytes and does not count an unterminated final line.
pub fn count_newlines_unbuffered<R: Read>(mut reader: R) -> io::Result<usize> {
    let mut count = 0;
    let mut byte = [0u8; 1];
    loop {
        match reader.read(&mut byte) {
            Ok(0) => return Ok(count),
            Ok(_) => count += usize::from(byte[0] == b'\n'),
            Err(error) => return Err(error),
        }
    }
}

/// Count newline bytes with buffered reads while preserving the byte contract.
///
/// Returns `InvalidInput` for zero capacity and propagates reader failures.
pub fn count_newlines_buffered<R: Read>(reader: R, capacity: usize) -> io::Result<usize> {
    if capacity == 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "buffer capacity must be positive",
        ));
    }
    let mut reader = BufReader::with_capacity(capacity, reader);
    let mut count = 0;
    loop {
        let buffer = reader.fill_buf()?;
        if buffer.is_empty() {
            return Ok(count);
        }
        count += buffer.iter().filter(|&&byte| byte == b'\n').count();
        let consumed = buffer.len();
        reader.consume(consumed);
    }
}

/// Count UTF-8 text lines, as in the rule's fast `lines()` example.
///
/// Unlike newline-byte counting, this includes an unterminated final line and
/// rejects invalid UTF-8. It also allocates a `String` for each yielded line.
pub fn count_text_lines<R: Read>(reader: R) -> io::Result<usize> {
    BufReader::new(reader)
        .lines()
        .try_fold(0, |count, line| line.map(|_| count + 1))
}

/// Write each record followed by a newline, then explicitly flush the writer.
///
/// The explicit flush gives both benchmark variants the same completion scope.
pub fn write_records_unbuffered<W: Write>(mut writer: W, records: &[String]) -> io::Result<()> {
    for record in records {
        writer.write_all(record.as_bytes())?;
        writer.write_all(b"\n")?;
    }
    writer.flush()
}

/// Buffer record writes, including an explicit flush that exposes flush errors.
///
/// Returns `InvalidInput` for zero capacity and propagates write/flush failures.
pub fn write_records_buffered<W: Write>(
    writer: W,
    records: &[String],
    capacity: usize,
) -> io::Result<()> {
    if capacity == 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "buffer capacity must be positive",
        ));
    }
    let mut writer = BufWriter::with_capacity(capacity, writer);
    for record in records {
        writer.write_all(record.as_bytes())?;
        writer.write_all(b"\n")?;
    }
    writer.flush()
}
