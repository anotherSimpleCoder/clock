use std::io::Read;
use std::process::{Command, Stdio};
use rocket::tokio::time::{self, Duration};
use rocket::response::stream::ByteStream;


pub fn run(video_url: String) -> ByteStream![Vec<u8>] {
    let mut yt_dlp = Command::new("yt-dlp")
        .arg(video_url)
        .arg("-o")
        .arg("-")
        .stdout(Stdio::piped())
        .spawn()
        .expect("FatalError: Couldn't launch yt-dlp!");

    let byte_stream = ByteStream! {
        let mut interval = time::interval(Duration::from_millis(100));
        let mut buffer = vec![0; 8192];
        let stdout = &mut yt_dlp.stdout.take().unwrap();

        loop {
            match stdout.read(&mut buffer) {
                Ok(0) => break,
                Ok(bytes_read) => {
                    yield buffer[..bytes_read].to_vec();
                },
                Err(_) => {
                    break;
                }
            }
        }

        interval.tick().await;
    };

    byte_stream
}
