use std::{
    env,
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

        stream.write_all(upper.as_ref())?;
        stream.write_all(b"\n")?;
        println!("Wrote transformed line; {}", upper);
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let address = if let Some(arg) = env::args().nth(1) {
        arg
    } else {
        "127.0.0.1:32444".into()
    };

    let listener = TcpListener::bind(address)?;

    thread::scope(|scope| {
        for stream in listener.incoming() {
            println!("Client connected");
            let stream = stream.unwrap();
            scope.spawn(move || handle_client(stream).unwrap());
        }
    });
    Ok(())
}
