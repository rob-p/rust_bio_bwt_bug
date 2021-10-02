use bio::alphabets;
use bio::data_structures::bwt::{bwt, less, Occ};
use bio::data_structures::fmindex::{FMIndex, FMIndexable};
use bio::data_structures::suffix_array::suffix_array;
use bio::io::fasta;
use bio::io::fasta::FastaRead;
use std::env;

fn main() {
    // Create an FM-Index for the given text.
    let args: Vec<String> = env::args().collect();
    let ref_file = std::fs::File::open(&args[1]).unwrap();
    let mut reader = fasta::Reader::new(ref_file);
    let mut record = fasta::Record::new();
    let mut text: Vec<u8> = vec![];

    reader.read(&mut record).expect("Failed to parse record");
    while !record.is_empty() {
        let check = record.check();
        if check.is_err() {
            panic!("I got a rubbish record!")
        }
        // obtain sequence
        let seq = record.seq();
        text = seq.to_vec();
        text.push(b'$');
        reader.read(&mut record).expect("Failed to parse record");
    }

    // instantiate an alphabet
    let alphabet = alphabets::dna::iupac_alphabet();
    // calculate a suffix array
    let sa = suffix_array(&text[..]);
    // calculate the Burrows-Wheeler-transform
    let bwt = bwt(&text[..], &sa);
    // calculate the vectors less and Occ (occurrences)
    let less = less(&bwt, &alphabet);
    let occ = Occ::new(&bwt, 3, &alphabet);
    // set up FMIndex
    let fmindex = FMIndex::new(&bwt, &less, &occ);

    // Iterate over a FASTQ file, use the alphabet to validate read
    // sequences and search for exact matches in the FM-Index.
    let positions: Vec<usize> = vec![];

    // create FASTQ reader
    let query_file = std::fs::File::open(&args[2]).unwrap();
    let mut query_reader = fasta::Reader::new(query_file);
    query_reader
        .read(&mut record)
        .expect("Failed to parse record");

    while !record.is_empty() {
        let check = record.check();
        if check.is_err() {
            panic!("I got a rubbish record!")
        }
        // obtain sequence
        let seq = record.seq();
        println!("seq : {:?}", seq);
        // check, whether seq is in the expected alphabet
        if alphabet.is_word(seq) {
            let interval = fmindex.backward_search(seq.iter());
            println!("interval = {:?}", interval);
            let positions = interval.occ(&sa);
            for p in positions {
                println!("found pattern at position : {}", p)
            }
        }
        reader.read(&mut record).expect("Failed to parse record");
    }
}
