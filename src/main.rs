use memmap::Mmap;
use rand::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::LineWriter;
use std::vec::Vec;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    inpath: std::path::PathBuf,
    /// The path to the write the new file too
    #[structopt(parse(from_os_str))]
    outpath: std::path::PathBuf,
}

fn main() -> std::io::Result<()> {
    let args = Cli::from_args();

    // Open the input file
    let file = File::open(args.inpath)?;
    let mmap = unsafe { Mmap::map(&file)? };

    // Extract all of the line slices from the input
    let mut lines = Vec::<&[u8]>::new();
    let mut i = 0;
    let mut last: usize = 0;
    while i < mmap.len() {
        if mmap[i] == '\n' as u8 {
            lines.push(&mmap[last..i]);

            i += 1;
            while (i < mmap.len()) && (mmap[i] == '\n' as u8) {
                i += 1;
            }

            last = i;
            continue;
        }

        i += 1;
    }

    if last < mmap.len() {
        lines.push(&mmap[last..mmap.len()]);
    }

    lines.sort_unstable();
    lines.dedup();

    // Create an array of indexs into lines
    let mut idxs = vec![0; lines.len()];
    for i in 0..idxs.len() {
        idxs[i] = i;
    }

    // Shuffle the indexs
    let mut rng = rand::thread_rng();
    // Double permute this just to be safe :)
    idxs.shuffle(&mut rng);
    idxs.shuffle(&mut rng);

    // Create the output file
    let outf = File::create(args.outpath)?;
    let mut outf = LineWriter::new(outf);

    // Write all of the line slices to the output file
    for i in 0..idxs.len() {
        outf.write_all(&lines[idxs[i]])?;
        outf.write(b"\n")?;
    }

    Ok(())
}
