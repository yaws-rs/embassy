//! Basic Blueprint I/O

pub use blueprint::{InBuffer, Left, Right};

/// A/B I/O Left side
pub struct AbIoLeft {
    pub in_bytes: [u8; 4096],
    pub in_bytes_len: usize,
    pub out_bytes: [u8; 4096],
    pub out_bytes_len: usize,
    blocked: bool,
    ready: bool,
    want_read: bool,
    want_write: bool,
    shutdown: bool,
}

impl Default for AbIoLeft {
    fn default() -> Self {
        Self {
            in_bytes: [0; 4096],
            in_bytes_len: 0,
            out_bytes: [0; 4096],
            out_bytes_len: 0,
            blocked: false,
            ready: false,
            want_read: false,
            want_write: false,
            shutdown: false,
        }
    }
}

impl Left for AbIoLeft {
    /// Is Left in blocked?
    fn left_in_blocked(&self) -> bool {
        self.blocked
    }
    /// Set Left in blocked
    fn set_left_in_blocked(&mut self, b: bool) {
        self.blocked = b;
    }
    /// Lengths of Input and Output of Left side
    fn left_lens(&self) -> (usize, usize) {
        (self.in_bytes_len, self.out_bytes_len)
    }
    /// Set the Lengths of Input and Output of Left side
    fn left_set_lens(&mut self, new_len_in: usize, new_len_out: usize) -> () {
        let (cur_len_in, cur_len_out) = self.left_lens();

        if new_len_in < cur_len_in {
            let cut_off = cur_len_in - new_len_in;

            self.in_bytes.rotate_right(cut_off);
            self.in_bytes_len = new_len_in;
        }
        if new_len_out < cur_len_out {
            let cut_off = cur_len_out - new_len_out;
            self.out_bytes.rotate_right(cut_off);
            self.out_bytes_len = new_len_out;
        }
        if new_len_in > cur_len_in {
            self.in_bytes_len = new_len_in;
        }
        if new_len_out > cur_len_out {
            self.out_bytes_len = new_len_out;
        }
    }
    /// Mutable Input and Output bufs of Left side
    fn left_bufs_mut<'d>(&'d mut self) -> (InBuffer<'d>, &'d mut [u8]) {
        let b_buf_in = &mut self.in_bytes[..self.in_bytes_len];
        let b_buf_out = &mut self.out_bytes;
        (InBuffer::Single(b_buf_in), b_buf_out)
    }
    /// Indicates whether the Left side is ready for Right side
    fn is_ready(&self) -> bool {
        self.ready
    }
    /// Set the layer readiness for Right side input and output
    fn set_ready(&mut self, r: bool) -> bool {
        self.ready = r;
        self.ready
    }
    /// Indicates whether the Left side wants Input
    fn left_want_read(&self) -> bool {
        self.want_read
    }
    /// Set the Left side wanting to read
    fn set_left_want_read(&mut self, r: bool) -> () {
        self.want_read = r;
    }
    /// Indicates whether the Left side wants Output
    fn left_want_write(&self) -> bool {
        self.want_write
    }
    /// Set the Left side wanting to write
    fn set_left_want_write(&mut self, w: bool) -> () {
        self.want_write = w;
    }
    /// State machine signals shutdown (e.g. peer signals close)
    fn shutdown(&mut self) -> () {
        self.shutdown = true;
    }
}

/// A/B I/O Right side
pub struct AbIoRight {
    pub in_bytes: [u8; 4096],
    pub in_bytes_len: usize,
    pub out_bytes: [u8; 4096],
    pub out_bytes_len: usize,
    pub want_right_next_in: bool,

    // Passed from Right as Left Intermediary
    pub left_blocked: bool,
    pub left_ready: bool,
    pub left_want_read: bool,
    pub left_want_write: bool,
    pub left_shutdown: bool,
}

impl AbIoRight {
    pub fn impl_left(&mut self) -> impl Left + use<'_> {
        self
    }
}

impl Default for AbIoRight {
    fn default() -> Self {
        Self {
            in_bytes: [0; 4096],
            in_bytes_len: 0,
            out_bytes: [0; 4096],
            out_bytes_len: 0,
            want_right_next_in: false,
            left_blocked: false,
            left_ready: false,
            left_want_read: false,
            left_want_write: false,
            left_shutdown: false,
        }
    }
}

impl Right for AbIoRight {
    /// Output length of Right side
    fn right_lens(&self) -> (usize, usize) {
        (self.in_bytes_len, self.out_bytes_len)
    }
    /// Indicate processing of Output of Right side
    fn buf_right_out(&self) -> &[u8] {
        &self.out_bytes[..self.out_bytes_len]
    }
    /// Indicate whether Right side wants next input block
    fn wants_right_next_in(&self) -> bool {
        self.want_right_next_in
    }
    /// SM: Indicate Right side to want the next block
    fn set_wants_right_next_in(&mut self, wr: bool) -> () {
        self.want_right_next_in = wr;
    }
    /// SM: Called when all Right side Output was consumed
    fn all_sent_right_out(&mut self) -> () {
        self.out_bytes = [0; 4096];
        self.out_bytes_len = 0;
    }
    /// Add bytes to Right output
    fn add_right_out(&mut self, new_data: &[u8]) -> () {
        let start = self.out_bytes_len;
        let end = start + new_data.len();
        self.out_bytes[start..end].copy_from_slice(new_data);
        self.out_bytes_len += new_data.len();
    }
    /// Add bytes to Right input
    fn add_right_in(&mut self, new_data: &[u8]) -> () {
        let start = self.in_bytes_len;
        let end = start + new_data.len();
        self.in_bytes[start..end].copy_from_slice(new_data);
        self.in_bytes_len += new_data.len();
    }
}

impl Left for &mut AbIoRight {
    /// Is Left in blocked?
    fn left_in_blocked(&self) -> bool {
        self.left_blocked
    }
    /// Set Left in blocked
    fn set_left_in_blocked(&mut self, b: bool) {
        self.left_blocked = b;
    }
    /// Lengths of Input and Output of Left side
    fn left_lens(&self) -> (usize, usize) {
        (self.in_bytes_len, self.out_bytes_len)
    }
    /// Set the Lengths of Input and Output of Left side
    fn left_set_lens(&mut self, new_len_in: usize, new_len_out: usize) -> () {
        let (cur_len_in, cur_len_out) = self.left_lens();

        if new_len_in < cur_len_in {
            let cut_off = cur_len_in - new_len_in;

            self.in_bytes.rotate_right(cut_off);
            self.in_bytes_len = new_len_in;
        }
        if new_len_out < cur_len_out {
            let cut_off = cur_len_out - new_len_out;
            self.out_bytes.rotate_right(cut_off);
            self.out_bytes_len = new_len_out;
        }
        if new_len_in > cur_len_in {
            self.in_bytes_len = new_len_in;
        }
        if new_len_out > cur_len_out {
            self.out_bytes_len = new_len_out;
        }
    }
    /// Mutable Input and Output bufs of Left side
    fn left_bufs_mut<'d>(&'d mut self) -> (InBuffer<'d>, &'d mut [u8]) {
        let b_buf_in = &mut self.in_bytes[..self.in_bytes_len];
        let b_buf_out = &mut self.out_bytes;
        (InBuffer::Single(b_buf_in), b_buf_out)
    }
    /// Indicates whether the Left side is ready for Right side
    fn is_ready(&self) -> bool {
        self.left_ready
    }
    /// Set the layer readiness for Right side input and output
    fn set_ready(&mut self, r: bool) -> bool {
        self.left_ready = r;
        self.left_ready
    }
    /// Indicates whether the Left side wants Input
    fn left_want_read(&self) -> bool {
        self.left_want_read
    }
    /// Set the Left side wanting to read
    fn set_left_want_read(&mut self, r: bool) -> () {
        self.left_want_read = r;
    }
    /// Indicates whether the Left side wants Output
    fn left_want_write(&self) -> bool {
        self.left_want_write
    }
    /// Set the Left side wanting to write
    fn set_left_want_write(&mut self, w: bool) -> () {
        self.left_want_write = w;
    }
    /// State machine signals shutdown (e.g. peer signals close)
    fn shutdown(&mut self) -> () {
        self.left_shutdown = true;
    }
}
