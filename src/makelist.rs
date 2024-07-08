use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};
use crate::evaldirs;

pub fn makelist (hd_value: String, from_value: String, to_value: String, inptsubdir: i32, inptpre_value: String,
                 outsubdir: i32, outdir_value: String, num_rows: u64) -> (u32, String, String) {
    let mut errcode: u32 = 0;
    let mut errstring: String = "List created".to_string();
    let mut new_mergelist: String = " ".to_string();
    let mut bolok = true;
    let mut bolbsz = true;
    let mut from_int1: i64 = 0;
    let mut to_int1: i64 = 0;
    if from_value.len() == 0 {
         errstring = "********* List: From has no value **********".to_string();
         errcode = 1;
         bolok = false;
    } else {
         let from_int: i64 = from_value.parse().unwrap_or(-99);
         if from_int > 0 {
             bolbsz = false;
             from_int1 = from_int;
         } else if from_int == -99 {
             errstring = "********* List: From is not an integer **********".to_string();
             errcode = 2;
             bolok = false;
         } else {
             errstring = "********* List: From not positive integer **********".to_string();
             errcode = 3;
             bolok = false;
         }
         if !bolbsz {
             bolbsz = true;
             if to_value.len() == 0 {
                 errstring = "********* List: To has no value **********".to_string();
                 errcode = 4;
                 bolok = false;
             } else {
                 let to_int: i64 = to_value.parse().unwrap_or(-99);
                 if to_int > 0 {
                     bolbsz = false;
                     to_int1 = to_int;
                 } else if to_int == -99 {
                     errstring = "********* List: To is not an integer **********".to_string();
                     errcode = 5;
                     bolok = false;
                 } else {
                     errstring = "********* List: To not positive integer **********".to_string();
                     errcode = 6;
                     bolok = false;
                 }
                 if !bolbsz {
                     if to_int1 < from_int1 {
                         errstring = "********* List: From Greater than To **********".to_string();
                         errcode = 7;
                         bolok = false;
                     } else if from_int1 > num_rows as i64 {
                         errstring = "List: From Greater than number of rows".to_string();
                         errcode = 8;
                         bolok = false;
                     }
                 }
             }
         }
    }
    if bolok { 
        if inptsubdir > 0 {
            if !Path::new(&inptpre_value).exists() {
                errcode = 9;
                errstring = format!("input subdirectory {} does not exist", inptpre_value);
                bolok = false
            }
        }
    }
    if bolok {
        if !Path::new(&outdir_value).exists() {
            errcode = 10;
            errstring = format!("output directory {} does not exist", outdir_value);
            bolok = false
       }
    }
// see if filesize exists and is between 4 and 16
    if bolok {
        if !Path::new(&hd_value).exists() {
            errcode = 11;
            errstring = format!("HD list file {} does not exist", hd_value);
//            bolok = false
        } else {
            let file = File::open(hd_value).unwrap();
            let mut reader = BufReader::new(file);
            let mut line = String::new();
            let mut linenum: i64 = 0;
            loop {
                  match reader.read_line(&mut line) {
                      Ok(bytes_read) => {
                                 // EOF: save last file address to restart from this address for next run
                          if bytes_read == 0 {
                              break;
                          }
                          linenum = linenum + 1;
                          if linenum >= from_int1 {
                              if linenum <= to_int1 {
                                  let vecline: Vec<&str> = line.split("|").collect();
                                  if vecline.len() > 2 {
                                      if vecline.len() < 7 {
                                          errcode = 12;
                                          errstring = format!("Hd list row # {} is invalid length: {}", linenum, vecline.len());
//                                          bolok = false;
                                          break;
                                      } else {
                                          let mut inptfilenm: String = vecline[0].to_string();
                                          if inptfilenm[..1].to_string() == '"'.to_string() {
                                              inptfilenm = inptfilenm[1..(inptfilenm.len()-1)].to_string();
                                          }
                                          let newlinelist = format!("{} -- {}", linenum, inptfilenm);
                                          new_mergelist = new_mergelist + &newlinelist + "\n ";
                                          let mut inptdirnm: String = vecline[3].to_string();
                                          if inptdirnm[..1].to_string() == '"'.to_string() {
                                              inptdirnm = inptdirnm[1..(inptdirnm.len()-1)].to_string();
                                          }
                                          new_mergelist = new_mergelist + &inptdirnm + "\n ";
                                          let (errcode1, errstr1, newinput1, newout1) = evaldirs(inptdirnm, inptsubdir.clone(),
                                                                                          inptpre_value.clone(), outsubdir.clone(), outdir_value.clone());
                                          if errcode1 != 0 {
                                              errcode = 13;
                                              errstring = format!("Hd list row # {} is invalid because: {}", linenum, errstr1);
//                                              bolok = false;
                                              break;
                                          } else {
                                             new_mergelist = new_mergelist + &newinput1 + "\n ";
                                             new_mergelist = new_mergelist + &newout1 + "\n " + "\n ";
                                         }
                                     }
                                  }
                              }
                          }
                          line.clear();
                      }
                      Err(_err) => {
                          errcode = 14;
                          errstring = "error reading Hd list ".to_string();
//                          bolok = false;   
                          break;
                      }
                  };
            } 
        }
    }
    (errcode, errstring, new_mergelist)
}

