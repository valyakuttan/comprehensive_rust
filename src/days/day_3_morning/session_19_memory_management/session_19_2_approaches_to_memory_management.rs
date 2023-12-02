/// # 19.2 Approaches to Memory Management
///
/// Traditionally, languages have fallen into two broad categories:
///
/// - Full control via manual memory management: C, C++, Pascal, $\ldots$
///
/// - Full safety via automatic memory management at runtime: Java, Python,
///   Go, Haskell, $\ldots$
///
/// Rust offers a new mix:
///
/// `Full control and safety via compile time enforcement of correct memory management`.
///
/// It does this with an explicit ownership concept.
///
/// ## Manual Memory Management
///
/// You allocate and deallocate heap memory yourself.If not done with
/// care, this can lead to crashes, bugs, security vulnerabilities,
/// and memory leaks.
///
/// ### C Example
///
/// You must call free on every pointer you allocate with `malloc`:
///
/// ```
///
/// void foo(size_t n) {
///     int* int_array = malloc(n * sizeof(int));
///     //
///     // ... lots of code
///     //
///     free(int_array);
/// }
///
/// ```
///
/// Memory is leaked if the function returns early between `malloc` and
/// `free`: the pointer is lost and we cannot deallocate the memory.
/// Worse, freeing the pointer twice, or accessing a freed pointer can lead
/// to exploitable security vulnerabilities.
///
/// ## Scope-Based Memory Management
///
/// Constructors and destructors let you hook into the lifetime
/// of an object. By wrapping a pointer in an object, you can free
/// memory when the object is destroyed. The compiler guarantees that
/// this happens, even if an exception is raised.
///
/// This is often called **Resource Acquisition Is Initialization (RAII)**
/// and gives you smart pointers.
///
/// ### C++ Example
///
/// ```
///
/// void say_hello(std::unique_ptr<Person> person) {
///   std::cout << "Hello " << person->name << std::endl;
/// }
///
/// ```
/// - The `std::unique_ptr` object is allocated on the stack, and points to
///   memory allocated on the heap.
///
/// - At the end of `say_hello`, the `std::unique_ptr` destructor will run.
///
/// - The destructor frees the `Person` object it points to.
///
/// Special move constructors are used when passing ownership to a function:
///
/// ```
/// std::unique_ptr<Person> person = find_person("Carla");
/// say_hello(std::move(person));
/// ```
///
/// ## Automatic Memory Management
///
/// An alternative to manual and scope-based memory management is
/// automatic memory management:
///
/// - The programmer never allocates or deallocates memory explicitly.
///
/// - A garbage collector finds unused memory and deallocates it for
///   the programmer.
///
/// ### Java Example
///
/// The `person` object is not deallocated after `sayHello` returns:
///
/// ```
///
/// void sayHello(Person person) {
///   System.out.println("Hello " + person.getName());
/// }
///   
/// ```
///
/// ## Memory Management in Rust
///
/// Memory management in Rust is a mix:
///
/// - Safe and correct like Java, but without a garbage collector.
///
/// - Scope-based like C++, but the compiler enforces full adherence.
///
/// - A Rust user can choose the right abstraction for the situation,
///   some even have no cost at runtime like C.
///
/// Rust achieves this by modeling ownership explicitly.
///
/// - This is usually handled in Rust by RAII wrapper types such as
///   `Box`, `Vec`, `Rc`, or `Arc`. These encapsulate ownership and
///   memory allocation via various means, and prevent the potential
///   errors in C.
///
/// - The `Drop` trait is the Rust equivalent of destructor.
///

pub fn main() {}
