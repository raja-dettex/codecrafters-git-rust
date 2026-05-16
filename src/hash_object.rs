use std::path::PathBuf;
use std::io::Write;
use sha1::{Sha1, Digest};
use flate2::write::ZlibEncoder;
use flate2::Compression;

struct HashWriter<W> { 
    writer: W,
    hasher: Sha1
}

impl<W> Write for HashWriter<W> 
where W: Write 
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let n = self.writer.write(&buf)?;
        self.hasher.update(&buf[..n]);
        Ok(n)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}


pub fn hash_object(file: &PathBuf, write: bool) -> std::io::Result<()>{ 
    fn write_blob<W>(file: &PathBuf, mut writer: W) -> std::io::Result<String>
    where W: Write 
    { 
        let mut encoder = ZlibEncoder::new(writer, Compression::default());
        let mut writer = HashWriter { 
            writer: encoder,
            hasher: Sha1::new()
        };
        let stat = std::fs::metadata(file).expect("file stat");
        write!(writer, "blob ")?;
        write!(writer, "{}\0", stat.len())?;
        let mut file = std::fs::File::open(file)?;
        std::io::copy(&mut file, &mut writer)?;
        let _  = writer.writer.finish()?;
        let hash = writer.hasher.finalize();
        Ok(hex::encode(hash))

    }
    let hash = if write { 
        let temp  = "temp";
        let hash = write_blob(&file, std::fs::File::create(temp)?)?;
        std::fs::create_dir_all(format!(".git/objects/{}", &hash[..2]))?;
        std::fs::rename(temp, format!(".git/objects/{}/{}", &hash[..2], &hash[2..]))?;
        hash
    } else { 
        write_blob(&file, std::io::sink())?
    };
    println!("{hash}");
    Ok(())
}