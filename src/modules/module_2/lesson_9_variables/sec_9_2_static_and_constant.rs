/// # Static and Constant Variables
///
/// Static and constant variables are two different ways to
/// create globally-scoped values that cannot be moved or
/// reallocated during the execution of the program.
///
/// ## const
///
/// Constant variables are evaluated at compile time and their
/// values are inlined wherever they are used:
///
/// ```
/// const DIGEST_SIZE: usize = 3;
/// const ZERO: Option<u8> = Some(42);
///
/// fn compute_digest(text: &str) -> [u8; DIGEST_SIZE] {
///    let mut digest = [ZERO.unwrap_or(0); DIGEST_SIZE];
///    for (idx, &b) in text.as_bytes().iter().enumerate() {
///        digest[idx % DIGEST_SIZE] = digest[idx % DIGEST_SIZE].wrapping_add(b);
///    }
///    digest
/// }
///
/// fn main() {
///    let digest = compute_digest("Hello");
///    println!("digest: {digest:?}");
/// }
/// ```
///
/// Only functions marked `const` can be called at compile time
/// to generate `const` values. `const` functions can however be
/// called at runtime.
///
/// ```
/// const NUM: i16 = foo();
///
/// pub fn main() {
///    let x = foo();
///    let y = NUM;
///
///    assert_eq!(x, y);
/// }

/// const fn foo() -> i16 {
///    123
/// }
/// ```
///
/// ## static
/// 
/// Static variables will live during the whole execution of the program,
/// and therefore will not move. Static variables are not inlined upon use
/// and have an actual associated memory location. This is useful for unsafe
/// and embedded code, and the variable lives through the entirety of the
/// program execution. When a globally-scoped value does not have a reason to
/// need object identity, `const` is generally preferred.
/// 
/// Because `static` variables are accessible from any thread, they must be `Sync`.
/// Interior mutability is possible through a `Mutex`, atomic or similar. It is
/// also possible to have mutable statics, but they require manual synchronisation
/// so any access to them requires `unsafe` code.
/// 
/// Properties table:
/// 
/// | Property                                     | `static`                          | `const`      |
/// | ============================================ | ================================= | ============ |
/// | Has an address in memory                     | Yes                               | No (inlined) |
/// |                                              |                                   |              |
/// | Lives for the entire duration of the program | Yes                               | No           |
/// |                                              |                                   |              |
/// | Can be mutable                               | Yes (unsafe)                      | No           |
/// |                                              |                                   |              |
/// | Evaluated at compile time	                   | Yes (initialised at compile time) | Yes          |
/// |                                              |                                   |              |
/// | Inlined wherever it is used                  | No                                | Yes          |
/// |                                              |                                   |              |
///
///
 
#[allow(dead_code)]
pub fn main() {

}
