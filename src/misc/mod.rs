
pub struct MemoryRange {
    pub next:u32,
    pub start:u32,
}

impl MemoryRange {
    fn new(start:u32,next:u32)->MemoryRange {
        MemoryRange {
            start,
            next,
        }
    }
    
    fn size(&self)->i32{
        (self.next-self.start) as i32
    }
}

