/// # Automatic Memory Management
/// 
/// An alternative to manual and scope-based memory management is
/// automatic memory management:
/// 
/// - The programmer never allocates or deallocates memory explicitly.
/// 
/// - A garbage collector finds unused memory and deallocates it for
///   the programmer.
/// 
/// ## Java Example
///
/// The `person` object is not deallocated after `sayHello` returns:
/// ```
/// void sayHello(Person person) {
///   System.out.println("Hello " + person.getName());
/// }  
/// ```
/// 

#[allow(dead_code)]
pub fn main() {}
