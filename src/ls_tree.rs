#[allow(unused_imports)]
use std::env;
use std::ffi::CStr;
#[allow(unused_imports)]
use std::fs;
use std::io::BufRead;
use std::io::Read;
use std::io::Write;
//use anyhow::Ok;

use anyhow::Context;

use crate::object::Kind;
use crate::object::Object;

pub(crate) fn ls_tree(name_only: bool, object_hash: String) -> anyhow::Result<()>{ 
    let mut object = Object::read(object_hash)?;
    match object.kind {
        Kind::Tree => { 
            let mut hashbuf = [0u8; 20];
            let mut buf = Vec::new();
            let stdout = std::io::stdout();
            let mut stdout = stdout.lock();
            loop { 
                buf.clear();
                let n = object.reader.read_until(0, &mut buf).context("read till null byte")?;
                if n == 0 {
                    break;
                }
                object.reader.read_exact(&mut hashbuf[..]).context("read out hash buf")?;
                let mode_and_name = CStr::from_bytes_with_nul(&buf[..n]).context("invalid tree entry")?;
                let mut bits = mode_and_name.to_bytes().splitn(2, |&b| b == b' ');
                let mode = bits.next().context("mode and name always yiedls one mode definitely")?;
                let name = bits.next().ok_or_else(|| anyhow::anyhow!("tree entry has no file name"))?;
                if name_only { 
                    stdout.write_all(name).context("write tree name to stdout")?;
                } else { 
                    let mode = std::str::from_utf8(mode).context("parse valid mode from utf-8")?;
                    let hash = hex::encode(&hashbuf);
                    let object = Object::read(hash.clone()).with_context( || format!("read the hash {}", hash.clone()))?;
                    let kind = object.kind;
                    write!(stdout, " {mode:0>6} {kind} {hash}   ").context("write out hash").context("write kind and hash")?;
                    stdout.write_all(name).context("write out tree name")?;
                }
                writeln!(stdout, "").context("write new line")?;
            }
        },
        _ => { 
            anyhow::bail!("not our case to handle for this kind: {}", object.kind);
        }
    }
    Ok(())
}