#![no_main]
use std::io::prelude::*;
use std::io::Cursor;

use arbitrary::{Arbitrary, Unstructured};
use libfuzzer_sys::fuzz_target;
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipWriter};

const MIN_SIZE: usize = 5;
const MAX_SIZE: usize = 8192;

#[derive(Arbitrary, Debug)]
pub struct FuzzInput<'a> {
    pub large_file: bool,
    #[arbitrary(with = |num_files: &mut Unstructured| num_files.int_in_range(2..=10))]
    pub num_files: usize,
    pub data: &'a [u8],
}

fn compress(input: FuzzInput) -> Result<(), Box<dyn std::error::Error>> {
    let mut result = Vec::new();
    let mut writer = ZipWriter::new(Cursor::new(&mut result));

    let options = SimpleFileOptions::default()
        .compression_method(CompressionMethod::Stored)
        .large_file(input.large_file);

    for i in 0..input.num_files {
        writer.start_file(i.to_string(), options)?;
        writer.write_all(input.data)?;
    }

    writer.finish()?;

    Ok(())
}

fuzz_target!(|input: FuzzInput| {
    if (MIN_SIZE..MAX_SIZE).contains(&input.data.len()) {
        let _ = compress(input);
    }
});
