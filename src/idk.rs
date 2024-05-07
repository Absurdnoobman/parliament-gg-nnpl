use std::{fs::File, io::{Error, Read}};


use serde::{Deserialize, Serialize};
use serde_json::{self};

#[derive(Serialize, Deserialize)]
struct CommonsCommittees {
    select_committee: Vec<String>
}

fn get_json(file_path: String) -> Result<String, Error> {
    let file: Result<File, std::io::Error> = File::open(file_path);
    let mut raw: String = String::new();
    let _ =  match file {
        Ok(mut f) => f.read_to_string(&mut raw),
        Err(e) => return Err(e)
    };
    Ok(raw)
}

pub fn get_commons_committee_list() -> Result< Vec<String> , Error > {
    let json: Result<String, Error> = get_json("../assets/committees/Commons/commons committee list.json".to_string());
    let result: Result<CommonsCommittees , serde_json::Error> = match json {
        Ok(info) => serde_json::from_str(&info),
        Err(e) => return Err(e)
    };
    match result {
        Ok(list) => Ok(list.select_committee),
        Err(e) => Err(e.into())
    }
}
