#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;
//use anyhow::Ok;

use crate::object::Kind;
use crate::object::Object;

pub(crate) fn CatFile(pretty_print: bool, object_hash: String) -> anyhow::Result<()>{ 
    anyhow::ensure!(pretty_print, "p flag must be there");
    let mut object = Object::read(pretty_print, object_hash)?;
    match object.kind { 
        Kind::Blob => { 
            let mut stdout = std::io::stdout();
            let n = std::io::copy(&mut object.reader, &mut stdout)?;
            anyhow::ensure!(n == object.expected_size, "git/objects are not of expected size : expected {} actual: {n}",
            object.expected_size);
        },
        _ => { 
            anyhow::bail!("not our case to handle for this kind: {}", object.kind);
        }
    }
    Ok(())
}