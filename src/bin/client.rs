use byteorder::{NetworkEndian, ReadBytesExt};
use std::{
    io::{self, BufRead, BufReader, BufWriter, Read, Write},
    net::TcpStream,
};

fn main() -> std::io::Result<()> {
    let stdin = io::stdin().lock();
    let stdout = io::stdout().lock();
    let stdin_reader = BufReader::new(stdin);
    let mut stdout_writer = BufWriter::new(stdout);

    let mut stream = TcpStream::connect("127.0.0.1:32444")?;

    let mut buf = Vec::new();
    for line in stdin_reader.lines() {
        println!("Read new line");
        let line = line?;
        stream.write_all(line.as_ref())?;
        stream.write(&[b'\n'])?;
        println!("Wrote line to TcpStream");

        let transformed_len = stream.read_u32::<NetworkEndian>()?;
        println!("Reply has len {}", transformed_len);
        buf.resize(transformed_len.try_into().unwrap(), 0u8);
        stream.read_exact(&mut buf)?;

        println!("Read response from TcpStream: {:?}", buf);
        stdout_writer.write_all(&buf)?;
        stdout_writer.write(&[b'\n'])?;
        stdout_writer.flush()?;
    }

    Ok(())
}
