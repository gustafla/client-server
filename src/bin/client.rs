use std::{
    env,
    io::{self, BufRead, BufReader, BufWriter, Write},
    net::TcpStream,
};

fn main() -> std::io::Result<()> {
    let stdin = io::stdin().lock();
    let stdout = io::stdout().lock();
    let stdin_reader = BufReader::new(stdin);
    let mut stdout_writer = BufWriter::new(stdout);

    let address = if let Some(arg) = env::args().nth(1) {
        arg
    } else {
        "127.0.0.1:32444".into()
    };

    let mut stream = TcpStream::connect(address)?;
    let mut stream_reader = BufReader::new(stream.try_clone()?);

    let mut buf = String::new();
    for line in stdin_reader.lines() {
        println!("Read new line");
        let line = line?;
        stream.write_all(line.as_ref())?;
        stream.write_all(b"\n")?;
        println!("Wrote line to TcpStream");

        buf.clear();
        stream_reader.read_line(&mut buf)?;
        println!("Read response from TcpStream: {:?}", buf);

        stdout_writer.write_all(buf.as_bytes())?;
        stdout_writer.flush()?;
    }

    Ok(())
}
