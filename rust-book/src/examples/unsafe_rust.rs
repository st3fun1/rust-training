use std::ptr::addr_of;
use std::slice;

pub fn unsafe_rust_access_address() {
    let num = 32;
    let borrow = &num;

    println!("Address: {:?}", addr_of!(borrow));

 let mut num = 5;
 let r1 = &raw const num;
 let r2 = &raw mut num;
   unsafe {
     println!("r1 is: {}", *r1);
     println!("r2 is: {}", *r2);
   };
}


pub fn split_list() {
  fn split_at_mut(values:&mut [i32], mid: usize) -> (&mut [i32], &mut[i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
      (
        slice::from_raw_parts_mut(ptr, mid),
        slice::from_raw_parts_mut(ptr.add(mid), len - mid)
      )
    }
  }

  let mut v = vec![1,2,3,4,5];
  let r = &mut v[..];

  let result = split_at_mut(r, 2);
  println!("{:?} - {:?}", result.0, result.1)
}

static mut COUNTER: u32 = 0;

unsafe fn add_to_count(inc: u32) {
  unsafe {
    COUNTER += inc;
  }
}

pub fn run_unsafe_addition_to_static_var() {
  unsafe {
    add_to_count(3);
    println!("COUNTER: {}", *(&raw const COUNTER))
  }
}