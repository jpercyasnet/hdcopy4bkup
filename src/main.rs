use iced::widget::{button, column, row, text, progress_bar, scrollable, text_input, Space};
use iced::{Alignment, Element, Command, Application, Settings, Color, Length, Size};
use iced::theme::{Theme};
use iced::executor;
use iced::window;
use iced_futures::futures;
use futures::channel::mpsc;
extern crate chrono;
use std::process::Command as stdCommand;
use std::path::{Path};
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::time::Duration as timeDuration;
use std::thread::sleep;
use chrono::Local;
use std::time::Instant as timeInstant;

mod get_winsize;
mod inputpress;
mod diroutpress;
mod evaldirs;
mod makelist;
mod testpress;

use get_winsize::get_winsize;
use inputpress::inputpress;
use diroutpress::diroutpress;
use evaldirs::evaldirs;
use makelist::makelist;
use testpress::testpress;

pub fn main() -> iced::Result {
     let mut widthxx: f32 = 1350.0;
     let mut heightxx: f32 = 750.0;
     let (errcode, errstring, widtho, heighto) = get_winsize();
     if errcode == 0 {
         widthxx = widtho as f32 - 20.0;
         heightxx = heighto as f32 - 75.0;
         println!("{}", errstring);
     } else {
         println!("**ERROR {} get_winsize: {}", errcode, errstring);
     }

     Hdcopy4bkup::run(Settings {
        window: window::Settings {
            size: Size::new(widthxx, heightxx),
            ..window::Settings::default()
        },
        ..Settings::default()
     })
}

struct Hdcopy4bkup {
    hd_value: String,
    inptpre_value: String,
    inptsubdir_value: String,
    outdir_value: String,
    outsubdir_value: String,
    firstentrydir_value: String,
    newinptdir_value: String,
    newoutdir_value: String,
    mess_color: Color,
    msg_value: String,
    scrollheight: f32,
    scroll_value: String,
    liststr_value: String,
    inptsubdir_num: i32,
    outsubdir_num: i32,
    rows_num: u64,
    test_press: bool,
    do_progress: bool,
    from_value: String,
    to_value: String,
    progval: f64,
    tx_send: mpsc::UnboundedSender<String>,
    rx_receive: mpsc::UnboundedReceiver<String>,
}

#[derive(Debug, Clone)]
enum Message {
    HdPressed,
    OutPressed,
    Subinpt(String),
    Subout(String),
    ListPressed,
    NextPressed,
    TestPressed,
    CopyPressed,
    InSubPressed,
    ExecxFound(Result<Execx, Error>),
    FromChanged(String),
    ScrollHChg(String),
    ToChanged(String),
    ProgressPressed,
    ProgRtn(Result<Progstart, Error>),
}

impl Application for Hdcopy4bkup {
    type Message = Message;
    type Theme = Theme;
    type Flags = ();
    type Executor = executor::Default;
    fn new(_flags: Self::Flags) -> (Hdcopy4bkup, iced::Command<Message>) {
        let (tx_send, rx_receive) = mpsc::unbounded();
//        let mut heightxx: f32 = 190.0;
//        let (errcode, errstring, _widtho, heighto) = get_winsize();
//        if errcode == 0 {
//            heightxx = 190.0 + ((heighto as f32 - 768.0) / 2.0);
//            println!("{}", errstring);
//        } else {
//         println!("**ERROR {} get_winsize: {}", errcode, errstring);
//        }
        ( Self { hd_value: "--".to_string(), inptpre_value: "--".to_string(), msg_value: "no message".to_string(),
               rows_num: 0, mess_color: Color::from([0.0, 1.0, 0.0]), outdir_value: "--".to_string(),
               do_progress: false, progval: 0.0, tx_send, rx_receive, inptsubdir_value: "0".to_string(),
               outsubdir_value: "0".to_string(), from_value: "1".to_string(), newinptdir_value: "--".to_string(),
               to_value: "16".to_string(), firstentrydir_value: "--".to_string(), newoutdir_value: "--".to_string(),
               inptsubdir_num: 0, outsubdir_num: 0,     liststr_value:" List button not pressed \n \
                ".to_string(), scrollheight: 170.0, scroll_value: "170.0".to_string(), test_press: false,
          },
          Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("HD file list copy -- iced")
    }


    fn update(&mut self, message: Message) -> Command<Message>  {
        match message {
            Message::HdPressed => {
               self.test_press = false;
               let inputstr: String = self.hd_value.clone();
               let (errcode, errstr, newinput) = inputpress(inputstr);
               self.msg_value = errstr.to_string();
               if errcode == 0 {
                   if Path::new(&newinput).exists() {
                       self.mess_color = Color::from([0.0, 1.0, 0.0]);
                       self.hd_value = newinput.to_string();
                       self.rows_num = 0;
                       let mut bolok = true;
                       let file = File::open(newinput).unwrap();
                       let mut reader = BufReader::new(file);
                       let mut line = String::new();
                       let mut linenum: u64 = 0;
                       loop {
                          match reader.read_line(&mut line) {
                             Ok(bytes_read) => {
                                 // EOF: save last file address to restart from this address for next run
                                 if bytes_read == 0 {
                                     break;
                                 }
                                 linenum = linenum + 1;
                                 if linenum < 2 {
                                     let vecline: Vec<&str> = line.split("|").collect();
                                     if vecline.len() < 7 {
                                         self.msg_value = format!("Hd list row is invalid length: {}", vecline.len());
                                         self.mess_color = Color::from([0.0, 1.0, 0.0]);
                                     } else {
                                         let mut inptdirnm: String = vecline[3].to_string();
                                          if inptdirnm[..1].to_string() == '"'.to_string() {
                                              inptdirnm = inptdirnm[1..(inptdirnm.len()-1)].to_string();
                                          }
                                         self.firstentrydir_value = inptdirnm;
                                         let (errcode1, errstr1, newinput1, newout1) = 
                                            evaldirs(self.firstentrydir_value.clone(), self.inptsubdir_num.clone(),
                                                     self.inptpre_value.clone(), self.outsubdir_num.clone(),
                                                     self.outdir_value.clone());
                                         if errcode1 == 0 {
                                            self.newinptdir_value = newinput1;
                                            self.newoutdir_value = newout1;
                                         } else {
                                            self.msg_value = errstr1;
                                            self.mess_color = Color::from([1.0, 0.0, 0.0]);
                                            bolok = false;
                                            break;
                                         }
                                     }
                                 }
                                 line.clear();
                             }
                             Err(_err) => {
                                 self.msg_value = "error reading Hd list ".to_string();
                                 self.mess_color = Color::from([1.0, 0.0, 0.0]);
                                 bolok = false;   
                                 break;
                             }
                          };
                       }
                       if bolok {
                           self.rows_num = linenum;

                           self.mess_color = Color::from([0.0, 1.0, 0.0]);
                           self.msg_value = "got HD list file and retrieved its number of rows".to_string();
                       } 
                   } else {
                       self.mess_color = Color::from([1.0, 0.0, 0.0]);
                       self.msg_value = format!("HD list file does not exist: {}", newinput);
                   }
               } else {
                   self.mess_color = Color::from([1.0, 0.0, 0.0]);
               }
               Command::none()
           }
            Message::OutPressed => {
               self.test_press = false;               
               let (errcode, errstr, newdir) = diroutpress(self.outdir_value.clone());
               if errcode == 0 {
                   self.mess_color = Color::from([0.0, 1.0, 0.0]);
                   self.msg_value = "got out Directory".to_string();
                   self.outdir_value = newdir.to_string();
                   let (errcode1, errstr1, newinput1, newout1) = 
                              evaldirs(self.firstentrydir_value.clone(), self.inptsubdir_num.clone(),
                                       self.inptpre_value.clone(), self.outsubdir_num.clone(),
                                       self.outdir_value.clone());
                   if errcode1 == 0 {
                       self.newinptdir_value = newinput1;
                       self.newoutdir_value = newout1;
                   } else {
                       self.msg_value = errstr1;
                       self.mess_color = Color::from([1.0, 0.0, 0.0]);
                   }
               } else {
                   self.mess_color = Color::from([1.0, 0.0, 0.0]);
                   self.msg_value = errstr.to_string();
               }
               Command::none()
           }
            Message::InSubPressed => {
               self.test_press = false;
               let (errcode, errstr, newdir) = diroutpress(self.inptpre_value.clone());
               if errcode == 0 {
                   let isd_int: i32 = self.inptsubdir_value.parse().unwrap_or(-99);
                   if isd_int > 0 {
                       self.inptsubdir_num = isd_int;
                   }
                   self.mess_color = Color::from([0.0, 1.0, 0.0]);
                   self.msg_value = "got out Directory".to_string();
                   self.inptpre_value = newdir.to_string();
                   let (errcode1, errstr1, newinput1, newout1) = 
                              evaldirs(self.firstentrydir_value.clone(), self.inptsubdir_num.clone(),
                                       self.inptpre_value.clone(), self.outsubdir_num.clone(),
                                       self.outdir_value.clone());
                   if errcode1 == 0 {
                       self.newinptdir_value = newinput1;
                       self.newoutdir_value = newout1;
                   } else {
                       self.msg_value = errstr1;
                       self.mess_color = Color::from([1.0, 0.0, 0.0]);
                   }
               } else {
                   self.mess_color = Color::from([1.0, 0.0, 0.0]);
                   self.msg_value = errstr.to_string();
               }
               Command::none()
           }
            Message::TestPressed => {
               let (errcode, errstr) = testpress(self.liststr_value.clone());
               self.msg_value = errstr.to_string();
               if errcode == 0 {
                   self.test_press = true;
                   self.mess_color = Color::from([0.0, 1.0, 0.0]);
               } else {
                   self.test_press = false;
                   self.mess_color = Color::from([1.0, 0.0, 0.0]);
               }
               Command::none()
            }
            Message::CopyPressed => {
               if self.test_press {
                   let (errcode, errstr) = testpress(self.liststr_value.clone());
                   self.test_press = false;
                   if errcode == 0 {
                       self.msg_value = "Copying just started".to_string();
                       self.mess_color = Color::from([0.0, 0.0, 1.0]);
                       Command::perform(Execx::copyit(self.liststr_value.clone(), self.tx_send.clone()), Message::ExecxFound)
                   } else {
                       self.msg_value = errstr.to_string();
                       self.mess_color = Color::from([1.0, 0.0, 0.0]);
                       Command::none()
                   }
               } else {
                   self.msg_value = "Test button was not pressed".to_string();
                   self.mess_color = Color::from([1.0, 0.0, 0.0]);
                   Command::none()
               }
            }
            Message::ExecxFound(Ok(exx)) => {
               self.msg_value = exx.errval.clone();
               if exx.errcd == 0 {
                   self.mess_color = Color::from([0.0, 1.0, 0.0]);
               } else {
                   self.mess_color = Color::from([1.0, 0.0, 0.0]);
               }
               Command::none()
            }
            Message::Subinpt(value) => {
               self.test_press = false;
               self.inptsubdir_value = value;
               if self.inptsubdir_value.len() == 0 {
                   self.inptsubdir_num = 0;
                   self.msg_value = "input subdir not integer".to_string();
                   self.mess_color = Color::from([1.0, 0.0, 0.0]);
               } else {
                   let isd_int: i32 = self.inptsubdir_value.parse().unwrap_or(-99);
                   if isd_int == -99 {
                       self.inptsubdir_num = 0;
                       self.msg_value = "input subdir not an integer".to_string();
                       self.mess_color = Color::from([1.0, 0.0, 0.0]);
                   } else if isd_int < 0 {
                       self.inptsubdir_num = 0;
                       self.msg_value = "input subdir not positive integer".to_string();
                       self.mess_color = Color::from([1.0, 0.0, 0.0]);
                   } else {
                       if self.inptpre_value == "--".to_string() {
                           self.inptsubdir_num = 0;
                           self.msg_value = "no input pre subdir value specified: number will be ignored".to_string();
                           self.mess_color = Color::from([1.0, 0.0, 0.0]);
                       } else {
                           self.inptsubdir_num = isd_int;
                           self.msg_value = "input subdir is good integer".to_string();
                           self.mess_color = Color::from([0.0, 1.0, 0.0]);
                           let (errcode1, errstr1, newinput1, newout1) = 
                              evaldirs(self.firstentrydir_value.clone(), self.inptsubdir_num.clone(),
                                       self.inptpre_value.clone(), self.outsubdir_num.clone(),
                                       self.outdir_value.clone());
                           if errcode1 == 0 {
                               self.newinptdir_value = newinput1;
                               self.newoutdir_value = newout1;
                           } else {
                               self.msg_value = errstr1;
                               self.mess_color = Color::from([1.0, 0.0, 0.0]);
                           }
                       }
                   }
               }
               Command::none()
            }
            Message::Subout(value) => {
               self.test_press = false;
               self.outsubdir_value = value;
               if self.outsubdir_value.len() == 0 {
                   self.outsubdir_num = 0;
                   self.msg_value = "output subdir not integer".to_string();
                   self.mess_color = Color::from([1.0, 0.0, 0.0]);
               } else {
                   let osd_int: i32 = self.outsubdir_value.parse().unwrap_or(-99);
                   if osd_int == -99 {
                       self.outsubdir_num = 0;
                       self.msg_value = "output subdir not an integer".to_string();
                       self.mess_color = Color::from([1.0, 0.0, 0.0]);
                   } else if osd_int < 0 {
                       self.outsubdir_num = 0;
                       self.msg_value = "output subdir not positive integer".to_string();
                       self.mess_color = Color::from([1.0, 0.0, 0.0]);
                   } else {
                       self.outsubdir_num = osd_int;
                       self.msg_value = "ouput subdir is good integer".to_string();
                       self.mess_color = Color::from([0.0, 1.0, 0.0]);
                       let (errcode1, errstr1, newinput1, newout1) = 
                              evaldirs(self.firstentrydir_value.clone(), self.inptsubdir_num.clone(),
                                       self.inptpre_value.clone(), self.outsubdir_num.clone(),
                                       self.outdir_value.clone());
                       if errcode1 == 0 {
                           self.newinptdir_value = newinput1;
                           self.newoutdir_value = newout1;
                       } else {
                           self.msg_value = errstr1;
                           self.mess_color = Color::from([1.0, 0.0, 0.0]);
                       }
                   }
               }
               Command::none()
            }
            Message::FromChanged(value) => {
               self.test_press = false;
               self.from_value = value;
               Command::none()
            }
            Message::ToChanged(value) => {
               self.test_press = false;
               self.to_value = value;
               Command::none()
            }
            Message::ScrollHChg(value) => {
               self.test_press = false;
               let height_flt: f32 = value.parse().unwrap_or(-99.0);
               if height_flt > 170.0 {
                   self.scrollheight = height_flt;
                   self.msg_value = "Good scroll height value".to_string();
                   self.mess_color = Color::from([0.0, 1.0, 0.0]);
               } else if height_flt == -99.0 {
                   self.msg_value = "Scroll height value is not a floating point number".to_string();
                   self.mess_color = Color::from([1.0, 0.0, 0.0]);
               } else {
                   self.msg_value = "Scroll height value is less than 170.0".to_string();
                   self.mess_color = Color::from([1.0, 0.0, 0.0]);
               }
               self.scroll_value = value;
               Command::none()
            }
            Message::ListPressed => {
               self.test_press = false;
               let (errcd, errstr, listitems) =
                    makelist(self.hd_value.clone(), self.from_value.clone(), self.to_value.clone(),
                               self.inptsubdir_num.clone(), self.inptpre_value.clone(), self.outsubdir_num.clone(),
                               self.outdir_value.clone(), self.rows_num.clone());
               if errcd == 0 {
                   self.liststr_value  = listitems;
                   self.mess_color = Color::from([0.0, 1.0, 0.0]);
               } else {
                   self.mess_color = Color::from([1.0, 0.0, 0.0]);
               }
               self.msg_value = errstr.to_string();
               Command::none()
            }
            Message::NextPressed => {
               self.test_press = false;
               let mut from_int1: i64 = 0;
               let mut to_int1: i64 = 0;
               let mut bolok = true;
               if self.from_value.len() == 0 {
                   self.msg_value = "From has no value".to_string();
                   self.mess_color = Color::from([1.0, 0.0, 0.0]);
                   bolok = false;
               } else {
                   let from_int: i64 = self.from_value.parse().unwrap_or(-99);
                   if from_int > 0 {
                       from_int1 = from_int;
                       if self.to_value.len() == 0 {
                           self.msg_value = "To has no value".to_string();
                           self.mess_color = Color::from([1.0, 0.0, 0.0]);
                           bolok = false;
                       } else {
                           let to_int: i64 = self.to_value.parse().unwrap_or(-99);
                           if to_int > 0 {
                               to_int1 = to_int;
                           } else if to_int == -99 {
                               self.msg_value = "To is not an integer".to_string();
                               self.mess_color = Color::from([1.0, 0.0, 0.0]);
                               bolok = false;
                           } else {
                               self.msg_value = "From is not positive integer".to_string();
                               self.mess_color = Color::from([1.0, 0.0, 0.0]);
                               bolok = false;
                           }
                       }
                   } else if from_int == -99 {
                        self.msg_value = "From is not an integer".to_string();
                        self.mess_color = Color::from([1.0, 0.0, 0.0]);
                        bolok = false;
                   } else {
                        self.msg_value = "From is not positive integer".to_string();
                        self.mess_color = Color::from([1.0, 0.0, 0.0]);
                        bolok = false;
                   }
               }
               if bolok {
                   if to_int1 < from_int1 {
                        self.msg_value = "From Greater than To".to_string();
                        self.mess_color = Color::from([1.0, 0.0, 0.0]);
                        bolok = false;
                   } else {
                        let newfrom_int = to_int1 + 1;
                        if newfrom_int > self.rows_num as i64 {
                            self.msg_value = "No more rows to list".to_string();
                            self.mess_color = Color::from([1.0, 0.0, 0.0]);
                            bolok = false;
                        } else {
                            to_int1 = newfrom_int + to_int1 - from_int1;
                            from_int1 = newfrom_int;
                        }
                   }
               }
               if bolok {
                   self.to_value = format!("{}", to_int1);
                   self.from_value = format!("{}", from_int1);
                   let (errcd, errstr, listitems) =
                        makelist(self.hd_value.clone(), self.from_value.clone(), self.to_value.clone(),
                               self.inptsubdir_num.clone(), self.inptpre_value.clone(), self.outsubdir_num.clone(),
                               self.outdir_value.clone(), self.rows_num.clone());
                   if errcd == 0 {
                       self.liststr_value  = listitems;
                       self.mess_color = Color::from([0.0, 1.0, 0.0]);
                   } else {
                       self.mess_color = Color::from([1.0, 0.0, 0.0]);
                   }
                   self.msg_value = errstr.to_string();
               }
               Command::none()
            }
            Message::ExecxFound(Err(_error)) => {
               self.msg_value = "error in copyx copyit routine".to_string();
               self.mess_color = Color::from([1.0, 0.0, 0.0]);
               Command::none()
            }
            Message::ProgressPressed => {
                   self.do_progress = true;
                   Command::perform(Progstart::pstart(), Message::ProgRtn)
            }
            Message::ProgRtn(Ok(_prx)) => {
              if self.do_progress {
                let mut inputval  = " ".to_string();
                let mut bgotmesg = false;
                let mut b100 = false;
                while let Ok(Some(input)) = self.rx_receive.try_next() {
                   inputval = input;
                   bgotmesg = true;
                }
                if bgotmesg {
                    let progvec: Vec<&str> = inputval[0..].split("|").collect();
                    let lenpg1 = progvec.len();
                    if lenpg1 == 4 {
                        let prog1 = progvec[0].to_string();
                        if prog1 == "Progress" {
                            let num_flt: f64 = progvec[1].parse().unwrap_or(-9999.0);
                            if num_flt < 0.0 {
                                println!("progress numeric not numeric: {}", inputval);
                            } else {
                                let dem_flt: f64 = progvec[2].parse().unwrap_or(-9999.0);
                                if dem_flt < 0.0 {
                                    println!("progress numeric not numeric: {}", inputval);
                                } else {
                                    self.progval = 100.0 * (num_flt / dem_flt);
                                    if dem_flt <= num_flt {
                                        b100 = true;
                                    } else {
                                        self.msg_value = format!("progress: {:.3} of {:.3} {}", (num_flt/1000000000.0), (dem_flt/1000000000.0), progvec[3]);
                                        self.mess_color = Color::from([0.0, 0.0, 1.0]);
                                    }
                                }
                            }
                        } else {
                            println!("message not progress: {}", inputval);
                        }
                    } else {
                        println!("message not progress: {}", inputval);
                    }
                } 
                if b100 {
                    Command::none()   
                } else {         
                    Command::perform(Progstart::pstart(), Message::ProgRtn)
                }
              } else {
                Command::none()
              }
            }
            Message::ProgRtn(Err(_error)) => {
                self.msg_value = "error in Progstart::pstart routine".to_string();
                self.mess_color = Color::from([1.0, 0.0, 0.0]);
               Command::none()
            }

        }
    }

    fn view(&self) -> Element<Message> {
        column![
            row![text("Message:").size(25),
                 text(&self.msg_value).size(25).style(*&self.mess_color),
            ].align_items(Alignment::Center).spacing(10).padding(5),
            row![text(format!("# of rows: {}", self.rows_num)).size(15),
                 button("HD list input").on_press(Message::HdPressed),
                 text(&self.hd_value).size(15).width(1000)
                ].align_items(Alignment::Center).spacing(10).padding(5),
            row![text(format!("first entry directory: {}", self.firstentrydir_value)).size(15)
                ].align_items(Alignment::Center).spacing(10).padding(5),
            row![text(" sub-dir indent #: "),
                 text_input("0", &self.inptsubdir_value).on_input(Message::Subinpt).padding(5).size(15).width(60),
                 button("input prefix dir").on_press(Message::InSubPressed),
                 text(&self.inptpre_value).size(15).width(1000)
                ].align_items(Alignment::Center).spacing(10).padding(5),
            row![text(format!("new input entry directory: {}", self.newinptdir_value)).size(15)
                ].align_items(Alignment::Center).spacing(10).padding(5),
            row![text(" sub-dir indent #: "),
                 text_input("0", &self.outsubdir_value).on_input(Message::Subout).padding(5).size(15).width(60),
                 button("output dir").on_press(Message::OutPressed),
                 text(&self.outdir_value).size(15).width(1000)
                ].align_items(Alignment::Center).spacing(10).padding(5),
            row![text(format!("new output entry directory: {}", self.newoutdir_value)).size(15)
                ].align_items(Alignment::Center).spacing(10).padding(5),
            row![text("Scroll Height: "),
                 text_input("170.0", &self.scroll_value).on_input(Message::ScrollHChg).padding(5).size(15),
                 text("                 From: "),
                 text_input("1", &self.from_value).on_input(Message::FromChanged).padding(5).size(15),
                 text("                    To: "),
                 text_input("16", &self.to_value).on_input(Message::ToChanged).padding(5).size(15)
                ].align_items(Alignment::Center).spacing(10).padding(5),
            row![Space::with_width(Length::Fixed(50.0)),
                 button("List Button").on_press(Message::ListPressed),
                 Space::with_width(Length::Fixed(50.0)),
                 button("Next Button").on_press(Message::NextPressed),
            ].align_items(Alignment::Center).spacing(10).padding(5),
            scrollable(
                column![
                        text(format!("{}",&self.liststr_value))
                ].width(Length::Fill),
            ).height(Length::Fixed(self.scrollheight)),
            row![Space::with_width(Length::Fixed(50.0)),
                 button("Test Button").on_press(Message::TestPressed),
                 Space::with_width(Length::Fixed(50.0)),
                 button("Copy Button").on_press(Message::CopyPressed),
            ].align_items(Alignment::Center).spacing(10).padding(5),
            row![button("Start Progress Button").on_press(Message::ProgressPressed),
                 progress_bar(0.0..=100.0,self.progval as f32),
                 text(format!("{}%", &self.progval)).size(15),
            ].align_items(Alignment::Center).spacing(5).padding(5),
         ]
        .padding(5)
        .align_items(Alignment::Start)
        .into()
    }

    fn theme(&self) -> Theme {
         Theme::Dracula
              
    }

}

#[derive(Debug, Clone)]
struct Execx {
    errcd: u32,
    errval: String,
}
impl Execx {

    async fn copyit(liststr_value: String, tx_send: mpsc::UnboundedSender<String>,) -> Result<Execx, Error> {
     let mut errstring  = "completed copy".to_string();
     let mut errcode: u32 = 0;
     let listvec: Vec<&str> = liststr_value[0..].split("\n").collect();
     let mut lenmg1: usize = listvec.len();
     lenmg1 = lenmg1 - 1;
     let start_time = timeInstant::now();
     let mut indl: usize = 0;
     loop {
           if indl > lenmg1 {
               let diffx = start_time.elapsed();
               let minsx: f64 = diffx.as_secs() as f64/60 as f64;
               let datexx = Local::now();
               let msgx = format!("Progress|{}|{}| elapsed time {:.1} mins at {} {} of {}", indl, lenmg1, minsx, datexx.format("%H:%M:%S"), indl, lenmg1);
               tx_send.unbounded_send(msgx).unwrap();
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
                    errstring = format!("input file does not exist: --{}--",fullfrom);
                    errcode = 4;
                    break;
               }
               let fullto = outdir.clone() + "/" + &filename;
               if Path::new(&fullto).exists() {
                   errstring = format!("output file exists: {}",fullfrom);
                   errcode = 5;
                   break;
               }
               if !Path::new(&outdir).exists() {
                   let _output1 = stdCommand::new("mkdir")
                                         .arg("-p")
                                         .arg(&outdir)
                                        .output()
                                         .expect("failed to execute process");
               }
               let _output2 = stdCommand::new("cp")
                                         .arg("-p")
                                         .arg(&fullfrom)
                                         .arg(&fullto)
                                         .output()
                                         .expect("failed to execute process");
               let diffx = start_time.elapsed();
               let minsx: f64 = diffx.as_secs() as f64/60 as f64;
               let datexx = Local::now();
               let msgx = format!("Progress|{}|{}| elapsed time {:.1} mins at {} {} of {}", indl, lenmg1, minsx, datexx.format("%H:%M:%S"), indl, lenmg1);
               tx_send.unbounded_send(msgx).unwrap();
           }
           indl = indl + 1;
     }
     Ok(Execx {
            errcd: errcode,
            errval: errstring,
        })
    }
}
#[derive(Debug, Clone)]
pub enum Error {
//    APIError,
//    LanguageError,
}

// loop thru by sleeping for 5 seconds
#[derive(Debug, Clone)]
pub struct Progstart {
//    errcolor: Color,
//    errval: String,
}

impl Progstart {

    pub async fn pstart() -> Result<Progstart, Error> {
//     let errstring  = " ".to_string();
//     let colorx = Color::from([0.0, 0.0, 0.0]);
     sleep(timeDuration::from_secs(3));
     Ok(Progstart {
//            errcolor: colorx,
//            errval: errstring,
        })
    }
}
