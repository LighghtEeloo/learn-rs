pub fn main() {
    // First, check that moving Box is low cost.
    let b1 = Builder {
        buf: Box::new([0, 0, 0, 1, 1, 2, 3, 5]),
    };
    print_ptr(b1.buf.as_ptr());
    let b2 = b1;
    print_ptr(b2.buf.as_ptr()); // no extra clone
    println!("Above should be equal.");

    // Second, check that we can gather all the data.
    // messing around with more builders...
    let b3 = b2.clone();
    let mut b4 = b2.clone();
    b4.buf.reverse();
    let mut b5 = b2.clone();
    b5.buf.rotate_left(1);
    println!("b3 - b5 are:");
    println!("{:?}", b3);
    println!("{:?}", b4);
    println!("{:?}", b5);
    // ... and collect them!
    let mut buffer = Buffer::new();
    buffer.take(b3);
    buffer.take(b4);
    buffer.take(b5);
    println!("And buffer is:");
    println!("{:?}", buffer);
}

#[derive(Debug, Clone)]
struct Builder {
    buf: Box<[u8]>,
}

#[derive(Debug)]
struct Buffer {
    all: Vec<u8>,
}

impl Buffer {
    fn new() -> Self {
        Self {
            // 256 is just something big.
            all: Vec::with_capacity(256),
        }
    }
    fn take(&mut self, b: Builder) {
        self.all.extend(b.buf.iter())
    }
}

// used to check the memory status
fn print_ptr<T>(ptr: *const T) {
    println!("{}", ptr as usize);
}
