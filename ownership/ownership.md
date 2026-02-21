Understanding Ownership
Ownership is Rust’s most unique feature and has deep implications for the rest of the language. It enables Rust to make memory safety guarantees without needing a garbage collector, so it’s important to understand how ownership works. In this chapter, we’ll talk about ownership as well as several related features: borrowing, slices, and how Rust lays data out in memory.
Ecco il riassunto della pagina in formato Markdown per `ownership/readme.md`:

```markdown
# Understanding Ownership

Ownership is Rust's most unique feature and has deep implications for the rest of the language. It enables Rust to make memory safety guarantees without needing a garbage collector, so it's important to understand how ownership works. In this chapter, we'll talk about ownership as well as several related features: borrowing, slices, and how Rust lays data out in memory.

## Ownership as a Discipline for Memory Safety

### Problema in C/C++
- Use-after-free (accesso a memoria deallocata)
- Double-free (doppia deallocazione)
- Memory leak
- Undefined behavior

### Soluzione: Ownership
Sistema di regole verificate **a compile-time** che garantisce sicurezza memoria senza garbage collector.

### Regole fondamentali
1. Ogni valore ha un **owner** (una variabile)
2. Un valore può avere **un solo owner** alla volta
3. Quando l'owner esce dallo scope, il valore viene **droppato** (deallocato automaticamente)

### Move Semantics
```rust
let s1 = String::from("hello");
let s2 = s1;  // s1 viene MOVED in s2
// println!("{s1}"); // ERRORE: s1 non è più valida
```

**Differenza da C**:
- In C la copia è shallow → entrambi i puntatori puntano agli stessi dati → rischio double-free
- Rust invalida `s1` dopo il move e previene il problema

### Clone (copia profonda esplicita)
```rust
let s1 = String::from("hello");
let s2 = s1.clone();  // copia heap completa
// s1 e s2 sono entrambe valide
```

- `clone()` esegue copia profonda **esplicita** (più lenta)
- Nessuna copia implicita costosa (diverso da C++ con copy constructor)

### Copy Trait (tipi stack-only)
```rust
let x = 5;
let y = x;  // copia bit-a-bit (x rimane valida)
```

Tipi che implementano `Copy`:
- Tipi semplici: `i32`, `bool`, `char`, `f64`, etc.
- Tuple di tipi `Copy`: `(i32, i32)`
- Array di tipi `Copy`: `[i32; 5]`

**Regola**: un tipo NON può essere `Copy` se contiene dati heap (es. `String`, `Vec`)

### Ownership e funzioni
```rust
fn takes_ownership(s: String) {
    println!("{s}");
}  // s droppato qui

fn main() {
    let s = String::from("hello");
    takes_ownership(s);  // s MOVED nella funzione
    // println!("{s}");  // ERRORE: s non più valida
}
```

**Regola**: passare valore a funzione = **move** (o copia se tipo implementa `Copy`)

### Ritornare ownership
```rust
fn gives_ownership() -> String {
    String::from("hello")  // ownership trasferita al chiamante
}

fn main() {
    let s = gives_ownership();  // s riceve ownership
    println!("{s}");
}
```
