use std::{env, time};
use reqwest::StatusCode;

async fn ping(url: &String) -> u16 {
    let resp = reqwest::get(url).await.unwrap(); // reqwest::blocking::get(url).unwrap();
    let code = resp.status();
    match code {
        StatusCode::OK => println!("OK(200)"),
        code => { let cu16 = code.as_u16(); println!("ERR({cu16})") }
    }
    code.as_u16()
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Expected two paramters: interval (in sec.) and URL to check.");
        return;
    }

    let interval_str = &args[1];
    let url = &args[2];

    let interval_sec = interval_str.parse::<u64>()
        .expect("The first arg. expected to be an integer number (num. of seconds");
    let duration = time::Duration::from_secs(interval_sec);
    println!("Ping. interval={interval_sec} sec.; url={url}");
    let mut interval = tokio::time::interval(duration);

    let mut i: u64 = 0;
    loop {
        // by text of the task the url check should be done inside the loop.
        let parse_result = url::Url::parse(url);
        if let Err(e) = parse_result {
            println!("url parsing error! parse error={e}");
            return;
        }

        i = i + 1;
        print!("{i} :\t");
        ping(url).await;
        interval.tick().await;
    }
}
