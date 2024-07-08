use std::path::Path;

pub fn testpress (liststr_value: String) -> (u32, String) {
     let mut errstring  = "completed reading list".to_string();
     let mut errcode: u32 = 0;
     let listvec: Vec<&str> = liststr_value[0..].split("\n").collect();
     if listvec.len() < 2 {
         errstring = "List is less than 2 lines".to_string();
         errcode = 1;
     } else {
         let mut lenmg1: usize = listvec.len();
         lenmg1 = lenmg1 - 1;
         let mut indl: usize = 0;
         loop {
              if indl > lenmg1 {
                  break;
              }
              let linestr = listvec[indl];
              let lineparse: Vec<&str> = linestr[0..].split(" -- ").collect();
              if lineparse.len() > 1 {
                  let filename = lineparse[1].to_string();
                  indl = indl + 2;
                  if indl > lenmg1 {
                      errstring = format!("premature end of list for: {}", filename);
                      errcode = 2;
                      break;
                  }
                  let inputdir = listvec[indl].trim().to_string();

                  indl = indl + 1;
                  if indl > lenmg1 {
                      errstring = format!("premature end of list for: {}", filename);
                      errcode = 3;
                      break;
                  }
                  let outdir = listvec[indl].trim().to_string();
                  let fullfrom = inputdir + "/" + &filename;
                  if !Path::new(&fullfrom).exists() {
                      errstring = format!("input file does not exist:{} --{}--", lineparse[0], fullfrom);
                      errcode = 4;
                      break;
                  }
                  let fullto = outdir + "/" + &filename;
                  if Path::new(&fullto).exists() {
                      errstring = format!("output file exists:{} --{}--", lineparse[0], fullfrom);
                      errcode = 5;
                      break;
                  }
              }
              indl = indl + 1;
         }
     }              
     (errcode, errstring)
}
