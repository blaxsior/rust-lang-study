use core::slice;
static mut COUNTER: u32 = 0;

fn main() {
    {
        let mut num = 5;

        let r1 = &mut num as *mut i32;
        let r2 = &num as *const i32;

        // let addr = 0x012345usize;
        // let r3 = addr as *const i32;

        unsafe {
            println!("r1 = {}", *r1);
            println!("r2 = {}", *r2);

            *r1 += 10;

            println!("r1 = {}", *r1);
            println!("r2 = {}", *r2);
        }
    }

    {
        let mut num = 10;

        unsafe {
            let ptr = get_unsafe_ptr(&mut num);
            println!("{}", *ptr);
        }
    }

    // 추상화
    {
        let mut v = vec![1,2,3,4,5,6];

        let r = &mut v[..];

        let (a,b) = r.split_at_mut(3);
        let (a, b) = split_at_mut(r, 3);
    }

    // extern
    {
        let v = -3;
        unsafe {
            println!("{v}의 절대값 == {}", abs(v));
        }
    }

    {
        unsafe {
            println!("{}", COUNTER);
            COUNTER += 1;
        }
    }
}

unsafe fn get_unsafe_ptr(target: &mut i32) -> *const i32 {
    let ptr = target as *mut i32;
    *ptr += 10;
    return ptr;
}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut[i32], &mut[i32]) {
    let len = values.len();
    // let ptr = values as *mut [i32] as *mut i32;
    let ptr = values.as_mut_ptr();
    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

extern  "C" {
    fn abs(input: i32) -> i32;
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("called rust code from C");
}