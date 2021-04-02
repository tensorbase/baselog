use crossbeam_queue::ArrayQueue;
use std::io;
use std::io::{Error, Write};
use std::sync::Mutex;

pub struct ThreadWriter<W: Write> {
    inner: Mutex<Option<W>>,
    // buf: Vec<u8>,
    buf: ArrayQueue<Vec<u8>>,
}

impl<W: Write> ThreadWriter<W> {
    pub fn new(inner: W) -> ThreadWriter<W> {
        ThreadWriter::with_capacity(100, inner)
    }

    pub fn with_capacity(capacity: usize, inner: W) -> ThreadWriter<W> {
        ThreadWriter {
            inner: Mutex::new(Some(inner)),
            buf: ArrayQueue::<Vec<u8>>::new(capacity),
        }
    }

    pub fn flush_buf(&self) -> io::Result<()> {
        let mut counter = self.buf.capacity();

        // @TODO need to find a good capacity..
        // so we don't reallocate to often
        let mut logs = Vec::<u8>::with_capacity(1024 * 8);
        while !self.buf.is_empty() || counter == 0 {
            match self.buf.pop() {
                Some(mut log) => logs.append(&mut log),
                None => break,
            }
            counter -= 1;
        }
//        println!("{}", std::str::from_utf8(&logs).unwrap());

        let wrt = self
            .inner
            .lock()
            .expect("mutex posioned")
            .as_mut()
            .unwrap()
            .write_all(&logs);

        // Currently we don't recover the removed buffer if
        // an error happens.
        match wrt {
//          Ok(0) => {
//                return Err(Error::new(
//                    ErrorKind::WriteZero,
//                    "failed to write the buffered data",
//                ));
//            }
            Ok(_) => Ok(()),
            Err(e) => return Err(e),
        }
    }

    // Buffer some data, if full then the value is returned as an error
    pub fn add(&self, buf: Vec<u8>) -> Result<(), Error> {
        if self.buf.len() >= 99 {
            self.flush_buf().expect("Flush error: ");
        }
        Ok(self.buf.push(buf).expect("ArrayQueue push filed: "))
    }
}
