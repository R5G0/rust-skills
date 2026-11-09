//! Byte contracts, semantic counterexamples, and injected I/O failures.

use std::io::{self, Cursor, Read, Write};

use proptest::prelude::*;
use rust_skills_perf_validation::buffered_io::*;

struct ControlledReader {
    input: Cursor<Vec<u8>>,
    max_chunk: usize,
    calls: usize,
    fail_after: Option<usize>,
}

impl ControlledReader {
    fn new(input: &[u8], max_chunk: usize) -> Self {
        Self {
            input: Cursor::new(input.to_vec()),
            max_chunk,
            calls: 0,
            fail_after: None,
        }
    }
}

impl Read for ControlledReader {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        if self.fail_after.is_some_and(|limit| self.calls >= limit) {
            return Err(io::Error::other("injected read failure"));
        }
        self.calls += 1;
        let length = buffer.len().min(self.max_chunk);
        self.input.read(&mut buffer[..length])
    }
}

struct ControlledWriter {
    bytes: Vec<u8>,
    max_chunk: usize,
    calls: usize,
    fail_after: Option<usize>,
    fail_flush: bool,
    flushes: usize,
}

impl ControlledWriter {
    fn new(max_chunk: usize) -> Self {
        Self {
            bytes: Vec::new(),
            max_chunk,
            calls: 0,
            fail_after: None,
            fail_flush: false,
            flushes: 0,
        }
    }
}

impl Write for ControlledWriter {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        if self.fail_after.is_some_and(|limit| self.calls >= limit) {
            return Err(io::Error::other("injected write failure"));
        }
        self.calls += 1;
        let length = buffer.len().min(self.max_chunk);
        self.bytes.extend_from_slice(&buffer[..length]);
        Ok(length)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.flushes += 1;
        if self.fail_flush {
            Err(io::Error::other("injected flush failure"))
        } else {
            Ok(())
        }
    }
}

#[test]
fn byte_and_text_counters_agree_for_terminated_utf8_lines() {
    for (input, expected) in [
        ("", 0),
        ("\n", 1),
        ("first\r\nsecond\n", 2),
        ("日本語\n", 1),
    ] {
        assert_eq!(
            count_newlines_unbuffered(input.as_bytes()).unwrap(),
            expected
        );
        assert_eq!(
            count_newlines_buffered(input.as_bytes(), 3).unwrap(),
            expected
        );
        assert_eq!(count_text_lines(input.as_bytes()).unwrap(), expected);
    }
}

#[test]
fn original_rule_examples_disagree_on_unterminated_final_line() {
    let input = b"first\nlast";
    assert_eq!(count_newlines_unbuffered(&input[..]).unwrap(), 1);
    assert_eq!(count_newlines_buffered(&input[..], 3).unwrap(), 1);
    assert_eq!(count_text_lines(&input[..]).unwrap(), 2);
}

#[test]
fn original_rule_examples_disagree_on_invalid_utf8() {
    let input = b"\xff\n";
    assert_eq!(count_newlines_unbuffered(&input[..]).unwrap(), 1);
    assert_eq!(count_newlines_buffered(&input[..], 3).unwrap(), 1);
    assert_eq!(
        count_text_lines(&input[..]).unwrap_err().kind(),
        io::ErrorKind::InvalidData
    );
}

#[test]
fn short_reads_and_buffer_boundaries_preserve_all_bytes() {
    for max_chunk in [1, 2, 5] {
        for capacity in [1, 2, 4, 16] {
            let reader = ControlledReader::new(b"a\n\r\nb\nlast", max_chunk);
            assert_eq!(count_newlines_buffered(reader, capacity).unwrap(), 3);
        }
    }
}

#[test]
fn buffering_reduces_underlying_reads_for_one_byte_workload() {
    let input = b"some record\n".repeat(128);
    let mut direct = ControlledReader::new(&input, usize::MAX);
    let mut buffered = ControlledReader::new(&input, usize::MAX);
    assert_eq!(count_newlines_unbuffered(&mut direct).unwrap(), 128);
    assert_eq!(count_newlines_buffered(&mut buffered, 128).unwrap(), 128);
    assert!(buffered.calls < direct.calls);
}

#[test]
fn all_read_paths_propagate_injected_errors() {
    for variant in 0..3 {
        let mut reader = ControlledReader::new(b"first\nsecond\n", 1);
        reader.fail_after = Some(2);
        let result = match variant {
            0 => count_newlines_unbuffered(reader),
            1 => count_newlines_buffered(reader, 4),
            _ => count_text_lines(reader),
        };
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::Other);
    }
}

#[test]
fn short_writes_preserve_output_and_buffering_reduces_calls() {
    let records: Vec<_> = ["", "a", "日本語", "x\ny"]
        .into_iter()
        .map(str::to_owned)
        .collect();
    let expected = b"\na\n\xe6\x97\xa5\xe6\x9c\xac\xe8\xaa\x9e\nx\ny\n";
    for max_chunk in [1, 2, 128] {
        let mut direct = ControlledWriter::new(max_chunk);
        let mut buffered = ControlledWriter::new(max_chunk);
        write_records_unbuffered(&mut direct, &records).unwrap();
        write_records_buffered(&mut buffered, &records, 64).unwrap();
        assert_eq!(direct.bytes, expected);
        assert_eq!(buffered.bytes, expected);
        assert_eq!(direct.flushes, 1);
        assert_eq!(buffered.flushes, 1);
        if max_chunk == 128 {
            assert!(buffered.calls < direct.calls);
        }
    }
}

#[test]
fn empty_record_batches_still_flush_explicitly() {
    let mut writer = ControlledWriter::new(8);
    write_records_buffered(&mut writer, &[], 8).unwrap();
    assert!(writer.bytes.is_empty());
    assert_eq!(writer.flushes, 1);
}

#[test]
fn write_failures_and_write_zero_reach_caller() {
    let records = vec!["a large enough record".to_owned()];
    for buffered in [false, true] {
        let mut writer = ControlledWriter::new(2);
        writer.fail_after = Some(1);
        let result = if buffered {
            write_records_buffered(&mut writer, &records, 8)
        } else {
            write_records_unbuffered(&mut writer, &records)
        };
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::Other);

        let mut zero_writer = ControlledWriter::new(0);
        let result = if buffered {
            write_records_buffered(&mut zero_writer, &records, 8)
        } else {
            write_records_unbuffered(&mut zero_writer, &records)
        };
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::WriteZero);
    }
}

#[test]
fn explicit_flush_failures_reach_caller() {
    let records = vec!["record".to_owned()];
    for buffered in [false, true] {
        let mut writer = ControlledWriter::new(8);
        writer.fail_flush = true;
        let result = if buffered {
            write_records_buffered(&mut writer, &records, 8)
        } else {
            write_records_unbuffered(&mut writer, &records)
        };
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::Other);
        assert!(writer.flushes >= 1);
    }
}

#[test]
fn zero_capacity_is_rejected_instead_of_silently_appearing_empty() {
    assert_eq!(
        count_newlines_buffered(&b"a\n"[..], 0).unwrap_err().kind(),
        io::ErrorKind::InvalidInput
    );
    assert_eq!(
        write_records_buffered(Vec::new(), &[], 0)
            .unwrap_err()
            .kind(),
        io::ErrorKind::InvalidInput
    );
}

proptest! {
    #[test]
    fn buffered_byte_counter_matches_reference_for_arbitrary_bytes(input in prop::collection::vec(any::<u8>(), 0..4096), capacity in 1usize..128) {
        let expected = input.iter().filter(|&&byte| byte == b'\n').count();
        prop_assert_eq!(count_newlines_unbuffered(input.as_slice()).unwrap(), expected);
        prop_assert_eq!(count_newlines_buffered(input.as_slice(), capacity).unwrap(), expected);
    }

    #[test]
    fn record_writers_emit_identical_bytes(records in prop::collection::vec("[a-zA-Z0-9 日本語]{0,24}", 0..32), capacity in 1usize..128) {
        let mut direct = Vec::new();
        let mut buffered = Vec::new();
        write_records_unbuffered(&mut direct, &records).unwrap();
        write_records_buffered(&mut buffered, &records, capacity).unwrap();
        let expected: Vec<u8> = records.iter().flat_map(|record| record.bytes().chain(std::iter::once(b'\n'))).collect();
        prop_assert_eq!(&direct, &expected);
        prop_assert_eq!(buffered, expected);
    }
}
