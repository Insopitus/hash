use std::io::{self, Read};

// use blake2::{Blake2b, Blake2s};
use clap::{Parser, ValueEnum};
use digest::{Digest, DynDigest};
use md5::Md5;
use sha1::Sha1;
use sha2::{Sha256, Sha512};
fn main() {
    let config = Config::parse();
    // let source = std::fs::File::open(config.path);
    // match config.path {
    //     Some(path) => {}
    //     None => {
    //         println!("Usage: ")
    //     }
    // }
    let algo = config.algorithm;
    let mut hasher = algo.hasher();
    let result = if config.string {
        hash_string(&mut *hasher, &config.input)
    } else {
        hash_file(&mut *hasher, std::fs::File::open(&config.input).unwrap()).unwrap()
    };
    let hex = HexSlice(&result, config.lowercase);
    println!("{}: \n{}", algo, hex);
}

struct HexSlice<'a>(&'a [u8], bool);
impl<'a> std::fmt::Display for HexSlice<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.1 {
            for &byte in self.0 {
                write!(f, "{:0>2x}", byte)?;
            }
        } else {
            for &byte in self.0 {
                write!(f, "{:0>2X}", byte)?;
            }
        }
        Ok(())
    }
}

trait Hasher: DynDigest + io::Write {}
impl<T: DynDigest + io::Write> Hasher for T {}

#[derive(clap::Parser)]
struct Config {
    #[arg(short,long,default_value_t = Algorithm::Sha256)]
    algorithm: Algorithm,
    /// whether you want to treat the input as a string and hash
    /// it directly; note that the input string would be recognized
    /// as UTF-8 encoded.
    #[arg(short, long, default_value_t = false)]
    string: bool,
    #[arg(short, long, default_value_t = true)]
    lowercase: bool,
    /// path of the file you want to hash
    input: String,
}

#[derive(Parser, Copy, Clone, ValueEnum)]
enum Algorithm {
    Md5,
    Sha1,
    Sha256,
    Sha512,
    // Blake2s,
    // Blake2b,
}
impl Algorithm {
    fn hasher(self) -> Box<dyn Hasher> {
        match self {
            Algorithm::Md5 => Box::new(Md5::new()),
            Algorithm::Sha1 => Box::new(Sha1::new()),
            Algorithm::Sha256 => Box::new(Sha256::new()),
            Algorithm::Sha512 => Box::new(Sha512::new()),
            // Algorithm::Blake2s => Box::new(Blake2s::new()),
            // Algorithm::Blake2b => Box::new(Blake2b::new()),
        }
        // Md5::new()
    }
}

impl std::fmt::Display for Algorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Algorithm::Md5 => "md5",
            Algorithm::Sha1 => "sha1",
            Algorithm::Sha256 => "sha256",
            Algorithm::Sha512 => "sha512",
            // Algorithm::Blake2s => "blake2s",
            // Algorithm::Blake2b => "blake2b",
        };
        write!(f, "{}", s)
    }
}

fn hash_file(mut hasher: &mut dyn Hasher, mut file: impl Read) -> std::io::Result<Vec<u8>> {
    io::copy(&mut file, &mut hasher)?;
    let n = hasher.finalize_reset();
    Ok(n.to_vec())
}
fn hash_string(hasher: &mut dyn Hasher, s: &str) -> Vec<u8> {
    hasher.update(s.as_bytes());
    let n = hasher.finalize_reset();
    n.to_vec()
}
