# Unexpected result for fm-index lookup

First build the minimal example

```
cargo build --release
```

Then run the query, this is what I see.

```
$ ./target/release/rust_bio_bwt_bug data/sars_cov2.fa data/reads.fa
seq : [71, 67, 65, 65, 67, 67, 65, 65, 65, 84, 71, 84, 71, 67, 67, 84, 84, 84, 67, 65, 65, 67, 84, 67, 84, 67, 65, 84, 71, 65, 65, 71, 84, 71, 84, 71, 65, 84, 67, 65, 84, 84, 71, 84, 71, 71, 84, 71, 65, 65, 65, 67, 84, 84, 67, 65, 84]
interval = Interval { lower: 1494, upper: 1495 }
found pattern at position : 1248
```

However, this is *not* where the pattern occurs in the reference.  We can verify this with a simple linear 
search using BioPython:

```
❯ python3
Python 3.7.2 (default, Dec 27 2018, 07:35:52) 
[Clang 10.0.0 (clang-1000.11.45.5)] on darwin
Type "help", "copyright", "credits" or "license" for more information.
>>> import Bio.SeqIO
>>> g = list(Bio.SeqIO.parse('data/sars_cov2.fa', 'fasta'))
>>> r = list(Bio.SeqIO.parse('data/reads.fa', 'fasta'))
>>> g[0].seq.find(r[0].seq)
1199
```

`grep` agrees with BioPython:

```
❯ grep -n "GCAACCAAATGTGCCTTTCAACTCTCATGAAGTGTGATCATTGTGGTGAAACTTCAT" data/sars_cov2.fa
19:CAAATGAATGCAACCAAATGTGCCTTTCAACTCTCATGAAGTGTGATCATTGTGGTGAAACTTCATGGCA
```

the file has 70 character long lines, and the pattern appears 9 characters into line 19 (indexing starting at 1), so 17*70+9 = 1199 (it's 17 because the first line is the header).

So, I am not sure how to explain the RustBio result.
