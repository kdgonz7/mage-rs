// mage vm test
// this implements a simple bytecode vm
// with paging n stuff
//

use std::fmt::Error;

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

    fn read_from(&self, offset: usize, amount_of_bytes: usize) -> Result<Vec<i16>, i32> {
        let mut all_bytes: Vec<i16> = Vec::new();

        if (offset + amount_of_bytes) > self.p_frame.len() {
            Err(0)
        } else {
            for p in offset..offset + amount_of_bytes {
                all_bytes.push(self.p_frame[p]);
            }

            Ok(all_bytes)
        }
    }

    fn write_i32(&mut self, offset: usize, int: i32) {
        let req_size = size_of::<i32>();
        
        if offset + req_size > self.p_frame.len() {
            panic!("{} is bigger than {}, can not continue", offset + req_size, self.p_frame.len())
        }

        let writing: Vec<u8> = int.to_le_bytes().to_vec();

        let mut cur = 0;

        for i in offset..offset+req_size {
            self.p_frame[i] = writing[cur].try_into().unwrap();

            cur = cur + 1
        }
    }
}

fn main() {
    let mut page: MagePage = MagePage::new(2048);
    page.write_i32(0, 355);
    let bytes_read = page.read_from(0, 4).unwrap();

    println!("{:#?}", bytes_read)
}
