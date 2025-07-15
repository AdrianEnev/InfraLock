use std::path::Path;
use std::io::{self, Read};
use std::fs;
use sha2::{Sha256, Digest};

/// Calculate the SHA256 hash of a file.
pub fn file_sha256<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let mut file = fs::File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192];
    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 { break; }
        hasher.update(&buffer[..n]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

/// Compare two files by SHA256 hash. Returns true if they are different.
pub fn files_differ<P: AsRef<Path>>(a: P, b: P) -> io::Result<bool> {
    let hash_a = file_sha256(a)?;
    let hash_b = file_sha256(b)?;
    Ok(hash_a != hash_b)
}

/// Atomically replace a file with another.
pub fn atomic_replace<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> io::Result<()> {
    fs::rename(src, dst)?;
    Ok(())
} 