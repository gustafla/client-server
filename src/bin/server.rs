use byteorder::{NetworkEndian, WriteBytesExt};
use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
};

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let reader = BufReader::new(stream.try_clone()?);

    for line in reader.lines() {
        let line = line?;
        println!("Received line: {}", line);
        let upper = line.to_uppercase();

        stream.write_u32::<NetworkEndian>(upper.len().try_into().unwrap())?;
        println!("Wrote length: {}", upper.len());
        stream.write_all(upper.as_ref())?;
        println!("Wrote transformed line; {}", upper);
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:32444")?;
    thread::scope(|scope| {
        for stream in listener.incoming() {
            println!("Client connected");
            let stream = stream.unwrap();
            scope.spawn(move || handle_client(stream).unwrap());
        }
    });
    Ok(())
}
