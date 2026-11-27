# Key things to remember

A running cheat sheet of the big ideas. Short on purpose; the detail lives in the numbered lesson notes.

---

## 1. How values move between functions

### The three ways to pass a value
| Way | Syntax | What happens | Caller can use it after? |
|---|---|---|---|
| **Move** | `f(s)` | Ownership transfers to `f` | No |
| **Copy** | `f(n)` | Implicit bitwise copy (only `Copy` types) | Yes |
| **Borrow** | `f(&s)` / `f(&mut s)` | `f` gets a reference; owner keeps ownership | Yes |

```rust
let s = String::from("hi");
take(s);            // move: s is gone
// println!("{s}"); // error: borrow of moved value

let n = 5;
take_num(n);        // copy: n still usable
println!("{n}");

let mut v = vec![1];
read(&v);           // shared borrow
push(&mut v);       // mutable borrow
println!("{v:?}");  // still the owner
```

### Borrow rules
- `&T` (shared): **many at once, read-only**.
- `&mut T` (mutable): **exactly one at a time**, and no `&T` alongside it.
- Think "many readers XOR one writer", checked at compile time.

### `Copy` types
Integers, floats, `bool`, `char`, `&T`, and tuples/arrays made only of `Copy` types.
Anything that owns heap memory (`String`, `Vec`, `Box`) is **not** `Copy`, it moves.

### Related tools
- `.clone()`: explicit deep copy, which you then move. `take(s.clone())` keeps `s`.
- **Returning** a value moves ownership back to the caller: `fn make() -> String`.
- `Box<T>`: owns a heap allocation; moving the box moves only the pointer.
- `Rc<T>` / `Arc<T>`: shared ownership (reference counted; `Arc` is thread-safe).
- `RefCell<T>` / `Mutex<T>`: the borrow rules are checked **at runtime** instead of compile time (panic / block on violation).

### Raw pointers are not a fourth way
`*const T` / `*mut T` are just values, and the pointer itself is `Copy`.
What changes is that **the compiler stops checking access**: in `unsafe` code you keep the promises (valid, aligned, no aliasing `&mut`) yourself.

### The two-axis mental model
1. **How is the value passed?** Move or copy. (A reference is itself a small value that gets copied, or moved for `&mut`.)
2. **How is access checked?** Owned, `&T`, `&mut T`, or raw pointer (unchecked).
