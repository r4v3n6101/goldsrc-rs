use std::io;

pub fn validate_range(offset: u32, size: u32, label: &str) -> io::Result<(usize, usize)> {
    let offset = usize::try_from(offset).map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("{label} offset overflow"),
        )
    })?;
    let size = usize::try_from(size).map_err(|_| {
        io::Error::new(io::ErrorKind::InvalidData, format!("{label} size overflow"))
    })?;
    if offset.checked_add(size).is_none() {
        return Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            format!("{label} out of range"),
        ));
    }
    Ok((offset, size))
}
