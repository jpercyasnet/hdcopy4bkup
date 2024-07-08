pub fn evaldirs(firstent: String, inptprenum: i32, inptprestr: String, outnum: i32, outstr: String) -> (u32, String, String, String) {
    let mut errcode: u32 = 0;
    let mut errstring: String = " ".to_string();
    let mut bolok = true;
    let mut newinput: String = "--".to_string();
    let mut newout: String = "--".to_string();
    if inptprestr == "--".to_string() {
        newinput = firstent.clone();
    } else if inptprenum == 0 {
        newinput = format!("{}{}", inptprestr, firstent);
    } else {
        let inptvec: Vec<&str> = firstent.split("/").collect();
        if inptprenum > inptvec.len() as i32 {
            errstring = format!("input pre number of {} greater than number of subdirectories of {}", inptprenum, inptvec.len());
            errcode = 1;
            bolok = false;
        } else {
            let mut lowersub = firstent.clone();
            for _indl in 0..inptprenum {    
                 match lowersub.split_once('/') {
                    Some((_lsval1, lsval2)) => {
                             lowersub = lsval2.to_string();
                    }
                    None => {
                             errstring = format!("could not split once firstent value {}", lowersub);
                             errcode = 2;
                             bolok = false;
                             break;
                    }
                 }
            }
            newinput = format!("{}/{}", inptprestr, lowersub);
        }
    }

    if bolok {
        if outstr == "--".to_string() {
            newout = outstr;
        } else if outnum == 0 {
            newout = format!("{}{}", outstr, firstent);
        } else {
            let inptvec: Vec<&str> = firstent.split("/").collect();
            if outnum > inptvec.len() as i32 {
                errstring = format!("output indent of {} greater than number of subdirectories of {}", outnum, inptvec.len());
                errcode = 2;
            } else {
                let mut lowersub1 = firstent;
                for _indl in 0..outnum {    
                     match lowersub1.split_once('/') {
                        Some((_lsval3, lsval4)) => {
                             lowersub1 = lsval4.to_string();
                        }
                        None => {
                             errstring = format!("could not split once firstent value {}", lowersub1);
                             errcode = 2;
                             break;
                    }
                 }
                }
                newout = format!("{}/{}", outstr, lowersub1);
            }
        }
    }

    (errcode, errstring, newinput, newout)
}

