use std::{net::TcpStream, error::Error, io::{Write, Read, self, ErrorKind}, u32, usize};

pub struct Header {
    length: u32,
    data_type: String,
}

fn parse_header(data_header: String) -> Result<Header, io::Error> {
    if data_header.is_empty() {
        return Err(io::Error::new(
            ErrorKind::InvalidData,
            "An error occurred while trying to parse AVTP header, header is empty!",
        ));
    }

    let parts: Vec<&str> = data_header.splitn(2, '\n').collect();
    if parts.len() != 2 {
        return Err(io::Error::new(
            ErrorKind::InvalidData,
            "Header does not contain expected format",
        ));
    }

    let length = match parts[0].parse::<u32>() {
        Ok(l) => l,
        Err(_) => {
            return Err(io::Error::new(
                ErrorKind::InvalidData,
                "Failed to parse length from header",
            ))
        }
    };

    let data_type = parts[1].to_owned();

    Ok(Header {
        length,
        data_type,
    })
}

pub fn send_data(mut stream: TcpStream, data: &[u8], data_type: &str) -> Result<(), Box<dyn Error>> {
    let header = format!("Length: {}\nType: {}\n\n", data.len(), data_type);
    stream.write_all(header.as_bytes())?;
    stream.write_all(data)?;
    Ok(())
}

pub fn receive_data(mut stream: TcpStream) -> Result<(Vec<u8>, Header), Box<dyn Error>> {
    let mut data_header = String::new();
    stream.read_to_string(&mut data_header)?;
    
    let header = parse_header(data_header)?;

    let mut data = vec![0u8; header.length as usize];
    stream.read_exact(&mut data)?;

    Ok((data, header))
}

fn main() {}
