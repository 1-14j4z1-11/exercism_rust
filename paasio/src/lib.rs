use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    reader: R,
    bytes: usize,
    reads: usize,
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats{reader:wrapped, bytes:0, reads:0}
    }

    pub fn get_ref(&self) -> &R {
        &self.reader
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn reads(&self) -> usize {
        self.reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let result = self.reader.read(buf);

        if let Ok(bytes) = result {
            self.bytes += bytes;
            self.reads += 1;
        }

        result
    }
}

pub struct WriteStats<W> {
    writer: W,
    bytes: usize,
    writes: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats{writer:wrapped, bytes:0, writes:0}
    }

    pub fn get_ref(&self) -> &W {
        &self.writer
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn writes(&self) -> usize {
        self.writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let result = self.writer.write(buf);

        if let Ok(bytes) = result {
            self.bytes += bytes;
            self.writes += 1;
        }

        result
    }

    fn flush(&mut self) -> Result<()> {
        self.writer.flush()
    }
}
