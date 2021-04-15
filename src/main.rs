#![feature(destructuring_assignment)]

#[cfg(feature = "chrono")]
use chrono::{Local, Timelike};
#[cfg (not(feature = "chrono"))]
use std::convert::TryInto;

use std::time::*;



///returns current time in utc+0, (seconds minutes hours)
#[cfg(not (feature = "chrono"))]
fn time_now() -> (u32, u32, u32)
{
    fn unix_time_to_smh(mut seconds: u64) -> (u32, u32, u32)
    {
        let mut  minutes = seconds/60;
        let mut  hours   = minutes/60;
        minutes%=60;
        hours%=24;
        seconds%=60;

        (seconds.try_into().unwrap(), minutes.try_into().unwrap(), hours.try_into().unwrap())
    }
    let now = SystemTime::now();
    match now.duration_since(UNIX_EPOCH)
    {
        Ok(val) =>  unix_time_to_smh(val.as_secs()),
        Err(_) => unimplemented!(),
    }
}
///returns current local time. seconds minutes hours
#[cfg(feature = "chrono")]
fn time_now()  -> (u32, u32, u32)
{
    let time = Local::now().naive_local().time();
    (time.second(), time.minute(), time.hour())
}


fn pretty_digits_table(ch:char) -> String
    {
        match ch
        {
            '0' =>
                String::from("   __     \n")+
                             "  /  \\    \n"+ //i can't use  native rust multiline strings because they not preserve left whitespaces
                             " | () |   \n"+
                             "  \\__/    \n",
            '1' =>
                String::from("  _       \n")+
                             " / |      \n"+
                             " | |      \n"+
                             " |_|      \n",
            '2' =>
                String::from("   ___    \n")+
                             "  |_  )   \n"+
                             "   / /    \n"+
                             "  /___|   \n",
            '3' =>
                String::from("   ____   \n")+
                             "  |__ /   \n"+
                             "   |_ \\   \n"+
                             "  |___/   \n",
            '4' =>
                String::from("  _ _     \n")+
                             " | | |    \n"+
                             " |_  _|   \n"+
                             "   |_|    \n",
            '5' =>
                String::from("  ___     \n")+
                             " | __|    \n"+
                             " |__ \\    \n"+
                             " |___/    \n",
            '6' =>
                String::from("   __     \n")+
                             "  / /     \n"+
                             " / _ \\    \n"+
                             " \\___/    \n",
            '7' =>
                String::from("  ____    \n")+
                             " |__  |   \n"+
                             "   / /    \n"+
                             "  /_/     \n",
            '8' =>
                String::from("  ___     \n")+
                             " ( _ )    \n"+
                             " / _ \\    \n"+
                             " \\___/    \n",
            '9' =>
                String::from("  ___     \n")+
                             " / _ \\    \n"+
                             " \\_. /    \n"+
                             "  /_/     \n",
            ':' =>
                String::from("  _       \n")+
                             " (_)      \n"+
                             "  _       \n"+
                             " (_)      \n",
            _ =>  unimplemented!(),

        }

    }

struct PrettyTimeStamp
{
    hours:u32,
    minutes:u32,
    seconds:u32
}
impl PrettyTimeStamp
{
    fn  to_str(&self) -> String
    {
        format!("{:02}:{:02}:{:02}",self.hours,self.minutes,self.seconds)
    }

    fn to_pretty_str(&self) -> String //this code is kinda sucks
    {
        let string = self.to_str();
        let mut vec: Vec<String> = Vec::new();
        let mut split_vec :Vec<Vec<String>> = Vec::new();

        for i in string.chars()
        {
            let pretty_symbol = pretty_digits_table(i);
            split_vec.push(pretty_symbol.split('\n').map(String::from).collect());
        }
        let mut j =0;
        for i in 0..split_vec[j].len()
        {
            vec.push(String::from(""));
            for _ in 0..split_vec.len()
            {
                vec[i]+=&split_vec[j][i];
                j+=1;
            }
            j=0;
        }


        let mut result_str = String::new();
        for i in vec.iter()
        {
            result_str+=&i;
            result_str+="\n"
        }

        result_str
    }


    fn new(hours:u32, minutes:u32, seconds:u32) -> PrettyTimeStamp
    {
        PrettyTimeStamp{hours, minutes, seconds}
    }
    fn now() -> PrettyTimeStamp
    {
        let (seconds, minutes, hours) = time_now();
        PrettyTimeStamp::new(hours, minutes, seconds)

    }

}
impl std::fmt::Display for PrettyTimeStamp
{
    fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "{}", self.to_pretty_str())
    }

}

fn main() {
    loop
    {
        println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n"); //dont mind me
        println!("{}", PrettyTimeStamp::now());
        std::thread::sleep(Duration::new(1, 0));

    }
}
