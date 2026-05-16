#[allow(unused_imports)]
use std::env;
use std::ffi::CStr;
use std::fmt::Display;
#[allow(unused_imports)]
use std::fs;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use anyhow::Context;
//use anyhow::Ok;
use flate2::read::ZlibDecoder;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Kind { 
    Blob,
    Tree,
    Commit
}

impl Display for Kind { 
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self { 
            Self::Blob => write!(f, "blob"),
            Self::Tree => write!(f, "tree"),
            Self::Commit => write!(f, "commit")
        }        
    }   
}
pub struct Object<R> { 
    pub(crate) kind: Kind,
    pub(crate) expected_size: u64,
    pub(crate) reader: R
}
impl Object<()> { 
    pub(crate) fn read(object_hash: String) -> anyhow::Result<Object<impl BufRead>>{ 
        let file = std::fs::File::open(format!(".git/objects/{}/{}", &object_hash[..2], &object_hash[2..])).context("read the hash file")?;
        let reader = BufReader::new(file);
        let z = ZlibDecoder::new(reader);
        let mut decoder_reader = BufReader::new(z);
        let mut buf = Vec::new();
        decoder_reader.read_until(0, &mut buf).context("read from .git/objects")?;
        let header = CStr::from_bytes_until_nul(&buf).expect("there is exactly one null bytes");
        let header = header.to_str().context("not valid utf-8 header")?;
        let Some((kind , size)) = header.split_once(' ') else {
            anyhow::bail!("")
        };
        let kind = match kind { 
            "blob" => Kind::Blob,
            "tree" => Kind::Tree,
            "commit" => Kind::Commit,
            _ =>  { 
                anyhow::bail!("dont know how to print other kinds");
            }
        };


        let size = size.parse::<usize>().context("not valid size")?;
        let z = decoder_reader.take(size as u64);
        Ok(Object { 
            kind,
            expected_size: size as u64,
            reader: z
        })
    }
}