use core::ops::{Deref, DerefMut};

pub(crate) struct Ram(Vec<u8>);

// ...implement index instead for dynamic resizing? Need a hashmap for that
// but we can use pages very directly this way.
impl Deref for Ram {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Ram {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Ram {
    fn new() -> Ram {
        let vec_u32: Vec<u32> = Vec::with_capacity(128);
        Ram(unsafe { core::mem::transmute(vec_u32) })
    }
}

