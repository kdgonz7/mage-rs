// mage vm test
// this implements a simple bytecode vm
// with paging n stuff
//

pub struct MagePage {
    p_frame: Vec<i16>,    // holds the info
}

#[derive(Debug)]
pub enum MageError {
    MRanOutOfSpaceError
}

impl MagePage {
    fn new(p_pagesize: isize) -> MagePage {
        let mut page_ret = MagePage {
            p_frame: Vec::new(),
        };

        for _ in 0..p_pagesize {
            page_ret.p_frame.push(-1);
        };

        page_ret
    }

    fn set_offset(&mut self, offset: usize, bytes: Vec<i16>) {
        let end: usize = offset + bytes.len();
        
        if end >= self.p_frame.len().try_into().unwrap() {
            panic!("too much data in set_offset");
        }

        let mut cursor: usize = 0;

        for i in offset..end {
            self.p_frame[i] = bytes[cursor];
            cursor = cursor + 1;
        }
    }

    fn read_from(&self, offset: usize, amount_of_bytes: usize) -> Result<Vec<i16>, MageError> {
        if (offset + amount_of_bytes) > self.p_frame.len() {
            return Err(MageError::MRanOutOfSpaceError)
        }

        Ok(self.p_frame[offset..(offset+amount_of_bytes)].to_vec())
    }

    fn write_i32(&mut self, offset: usize, int: i32) -> Result<(), MageError> {
        let req_size = size_of::<i32>();
        
        if offset + req_size > self.p_frame.len() {
            return Err(MageError::MRanOutOfSpaceError)
        }

        let writing: Vec<i16> = int.to_le_bytes().into_iter().map(|b| b as i16).collect();
        self.set_offset(offset, writing);

        Ok(())      
    }
}

fn main() {
    let mut page: MagePage = MagePage::new(2048);
    let write_355 = page.write_i32(0, 355);

    match write_355 {
        Ok(_) => {
            println!("wrote");
        }
        Err(_) => {
            println!("failed to write 355");
        }
    }

    let bytes_read = page.read_from(0, 4).unwrap();

    println!("{:#?}", bytes_read)
}
