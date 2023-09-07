use std::alloc::{alloc,dealloc,handle_alloc_error,Layout};
use std::ops::Deref;
struct RefCount<T>{
    data: T,
    count: usize,
}

struct MyRc<T>{
    sp: *mut RefCount<T>
}

impl<T> MyRc<T>{
    fn new(data:T) -> MyRc<T>{
        let layout = Layout::new::<RefCount<T>>();
        unsafe{
            let sp: *mut RefCount<T> = alloc(layout) as *mut RefCount<T>;
            if sp.is_null(){
                handle_alloc_error(layout);
            }
            (*sp).data = data;
            (*sp).count = 1;
            MyRc{ sp }
        }
    }
    fn count(&self) -> usize{
        unsafe{
            (*self.sp).count
        }
    }   
}

impl<T> Clone for MyRc<T>{
    fn clone(&self) -> MyRc<T>{
        unsafe{
            (*self.sp).count += 1;
        }
        MyRc{
            sp: self.sp
        }
    }
}

impl<T> Deref for MyRc<T>{
    type Target = T;

    fn deref(&self) -> &T{
        unsafe{
            &(*self.sp).data
        }   
    }
}
 impl<T> Drop for MyRc<T>{
    fn drop(&mut self) {
        unsafe{
            (*self.sp).count -= 1;
            if (*self.sp).count == 0{
                dealloc(self.sp as *mut u8, Layout::new::<RefCount<T>>());
            }
        }
    }
 }

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_myrc(){
        let rc = MyRc::new("Hello");
        assert_eq!(1,rc.count());
        {
            let rc2 = rc.clone();
            assert_eq!("Hello",*rc2);
            assert_eq!(2,rc.count());
            assert_eq!(2,rc2.count());
        }
        assert_eq!(1,rc.count());
        assert_eq!("Hello",*rc);
    }
}