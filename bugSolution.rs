fn main() {
    let mut v = vec![1, 2, 3];
    for i in 0..v.len() {
        v[i] = v[i] * 2; // Safe modification
    }
    println!("{:?}", v);
    let mut v2 = vec![1,2,3];
    unsafe{
        let ptr = v2.as_mut_ptr();
        //If you really need to use raw pointers, ensure the data is valid for the lifetime of the pointer.
        let slice = std::slice::from_raw_parts_mut(ptr,v2.len());
        for i in 0..slice.len(){
            slice[i] = slice[i]*2
        }
    }
    println!("{:?}", v2);
} 