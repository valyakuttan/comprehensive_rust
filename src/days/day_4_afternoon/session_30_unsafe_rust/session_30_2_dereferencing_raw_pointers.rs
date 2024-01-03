/// # 30.2 Creating and Dereferencing Raw Pointers
///
/// ## 30.2.1 Raw pointers
///
/// Raw Pointers are pointers without safety or liveness guarantees. Raw pointers
/// are written as *const T or *mut T. For example *const i32 means a raw pointer
/// to a 32-bit integer. Copying or dropping a raw pointer has no effect on the
/// lifecycle of any other value. Dereferencing a raw pointer is an unsafe operation.
///
/// This can also be used to convert a raw pointer to a reference by reborrowing it
/// (&* or &mut *). Raw pointers are generally discouraged; they exist to support
/// interoperability with foreign code, and writing performance-critical or
/// low-level functions.
///
/// When comparing raw pointers they are compared by their address, rather than by
/// what they point to. When comparing raw pointers to dynamically sized types
/// they also have their additional data compared.
///
/// Raw pointers can be created directly using `core::ptr::addr_of!` for *const
/// pointers and `core::ptr::addr_of_mut!` for *mut pointers.
///
/// Creating pointers is safe, but dereferencing them requires `unsafe`:
///
/// It is good practice (and required by the Android Rust style guide) to write a
/// comment for each `unsafe` block explaining how the code inside it satisfies
/// the safety requirements of the `unsafe` operations it is doing.
///
/// In the case of pointer dereferences, this means that the pointers must be valid, i.e.:
///
/// - The pointer must be non-null.
///
/// - The pointer must be dereferenceable (within the bounds of a single allocated object).
///
/// - The object must not have been deallocated.
///
/// - There must not be concurrent accesses to the same location.
///
/// - If the pointer was obtained by casting a reference, the underlying object must be
///   live and no reference may be used to access the memory.
///
/// - In most cases the pointer must also be properly aligned.
///

pub fn main() {
    let mut s = String::from("careful!");

    let r1 = &mut s as *mut String;
    //let r1 = core::ptr::addr_of_mut!(s);

    let r2 = r1 as *const String;
    //let r2 = core::ptr::addr_of!(s);

    // Safe because r1 and r2 were obtained from references and so are
    // guaranteed to be non-null and properly aligned, the objects underlying
    // the references from which they were obtained are live throughout the
    // whole unsafe block, and they are not accessed either through the
    // references or concurrently through any other pointers.
    unsafe {
        println!("r1 is: {}", *r1);
        *r1 = String::from("uhoh");
        println!("r2 is: {}", *r2);
    }

    // // NOT SAFE. DO NOT DO THIS.
    // let r3: &String = unsafe { &*r1 };
    // drop(s);
    // println!("r3 is: {}", *r3);

    // // This is an example of a common kind of UB bug: *r1 has the 'static lifetime,
    // // so r3 has type &'static String, and thus outlives s. Creating a reference
    // // from a pointer requires great care.
}
