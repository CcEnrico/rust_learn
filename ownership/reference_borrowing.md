# References and Borrowing

## Il problema: move-only API è scomoda

```rust
fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    greet(m1, m2);  // m1 e m2 vengono MOVED
    // let s = format!("{} {}", m1, m2); // ERRORE: m1 e m2 sono stati mossi
}

fn greet(g1: String, g2: String) {
    println!("{} {}!", g1, g2);
}  // g1 e g2 vengono droppati qui
```

Workaround verboso: restituire le stringhe come tupla → brutto e scomodo.

## Soluzione: References (Riferimenti)

Una **reference** è un puntatore **non-owning**: punta a dati senza possederne l'ownership.

```rust
fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    greet(&m1, &m2);  // & = borrow (presto in prestito)
    let s = format!("{} {}", m1, m2);  // OK: m1 e m2 sono ancora valide
}

fn greet(g1: &String, g2: &String) {  // & = reference
    println!("{} {}!", g1, g2);
}  // g1 e g2 spariscono, ma NON deallocano i dati (non li possedevano)
```

**Differenza da C**: in C i puntatori non hanno queste garanzie → dangling pointer possibili. In Rust il borrow checker garantisce che i riferimenti siano sempre validi.

---

## Dereferenziazione

L'operatore `*` (dereference) accede al valore puntato da un riferimento/box.

```rust
let mut x: Box<i32> = Box::new(1);
let a: i32 = *x;    // legge il valore heap → a = 1
*x += 1;            // modifica il valore heap → x punta a 2

let r1: &Box<i32> = &x;
let b: i32 = **r1;  // doppia deref: r1 → x (stack) → 2 (heap)

let r2: &i32 = &*x;
let c: i32 = *r2;   // singola deref: r2 → 2 (heap)
```

### Deref implicita (dot operator)

Rust inserisce automaticamente deref e borrow nelle chiamate a metodi:

```rust
let x: Box<i32> = Box::new(-1);
x.abs();        // equivalente a i32::abs(*x)

let r: &Box<i32> = &x;
r.abs();        // equivalente a i32::abs(**r) — doppia deref implicita

let s = String::from("Hello");
s.len();        // equivalente a str::len(&s) — borrow implicito
```

---

## Principio di Sicurezza dei Puntatori

> **Pointer Safety Principle**: un dato non può essere **aliasato e mutato contemporaneamente**.

### Aliasing

Aliasing = accedere agli stessi dati tramite variabili diverse.
- Aliasing da solo: innocuo.
- Aliasing + mutazione = **disastro** (dangling pointer, data race, UB).

### Esempio pericoloso (Vec + reference)

```rust
let mut v: Vec<i32> = vec![1, 2, 3];
let num: &i32 = &v[2];  // reference al terzo elemento
v.push(4);              // ERRORE: v potrebbe riallocare in heap!
// num ora punterebbe a memoria invalida → undefined behavior
println!("{}", *num);
```

In C/C++ questo compilerebbe, ma causerebbe un **buffer/heap use-after-free**: `v.push()` può riallocare l'array interno, lasciando `num` a puntare a memoria deallocata. **Questo è esattamente il tipo di bug che Rust previene**.

---

## Il Borrow Checker: Permessi sulle Variabili

Il borrow checker assegna a ogni variabile tre permessi (solo a compile-time):

| Permesso | Significato |
|----------|-------------|
| **R** (Read)  | il dato può essere copiato/letto |
| **W** (Write) | il dato può essere mutato |
| **O** (Own)   | il dato può essere mosso o droppato |

- Default: variabile ha **R + O**
- Con `let mut`: aggiunge **W**
- Creare una reference **rimuove temporaneamente** permessi dalla variabile originale

### Reference immutabile (`&`)

```rust
let mut v = vec![1, 2, 3];
let num: &i32 = &v[2];
// v perde W e O → non può essere mutata né mossa mentre num è viva
// *num guadagna R → può essere letta
println!("{}", *num);
// dopo l'ultimo uso di num → v riacquista W e O
v.push(4);  // OK ora
```

- **Permette aliasing, blocca mutazione** sulla sorgente.

### Reference mutabile (`&mut`)

```rust
let mut v = vec![1, 2, 3];
let num: &mut i32 = &mut v[2];
// v perde TUTTI i permessi (R, W, O) → nessun alias possibile
// *num guadagna R e W → può essere letta e mutata
*num += 1;  // modifica v[2]
// dopo l'ultimo uso di num → v riacquista tutti i permessi
println!("{:?}", v);  // OK
```

- **Permette mutazione, blocca aliasing** sulla sorgente.

**Regola chiave**: non si possono avere reference mutabili e immutabili attive contemporaneamente:

```rust
let mut s = String::from("hello");
let s2 = &s;        // reference immutabile
let s3 = &mut s;    // ERRORE: s3 mutable mentre s2 è ancora viva
s3.push_str(" world");
println!("{s2}");
```

---

## Lifetime di una Reference

Il **lifetime** di una reference va dalla sua creazione all'**ultimo uso** (non alla fine dello scope). Rust usa il lifetime per determinare quando restituire i permessi.

```rust
let mut x = 1;
let y = &x;    // x perde W
let z = *y;    // ultimo uso di y → x riacquista W
x += z;        // OK: y non è più viva
```

### Lifetime e Control Flow

Il lifetime può variare tra branch:

```rust
fn ascii_capitalize(v: &mut Vec<char>) {
    let c = &v[0];              // c prende borrow di *v
    if c.is_ascii_lowercase() {
        let up = c.to_ascii_uppercase();  // c usata qui → borrow attivo
        v[0] = up;              // OK: c non usata dopo questa riga
    } else {
        // c non viene usata nell'else → borrow già terminato
        println!("Already capitalized: {:?}", v);  // OK
    }
}
```

---

## I Dati Devono Sopravvivere ai Riferimenti

Rust garantisce che **nessuna reference punti a dati già deallocati**.

```rust
let s = String::from("Hello world");
let s_ref = &s;
drop(s);           // ERRORE: s perde O quando s_ref è viva
println!("{}", s_ref);
```

### Permesso F (Flow) — riferimenti in/out di funzioni

Per riferimenti in input/output di funzioni esiste un permesso aggiuntivo **F (flow)**:

```rust
// OK: Rust sa che il return viene da strings
fn first(strings: &Vec<String>) -> &String {
    &strings[0]
}

// ERRORE: Rust non sa se il return viene da strings o da default
fn first_or(strings: &Vec<String>, default: &String) -> &String {
    if strings.len() > 0 { &strings[0] } else { default }
}
```

→ Serve specificare i **lifetime parameters** (cap. 10): `fn first_or<'a>(strings: &'a Vec<String>, default: &'a String) -> &'a String`

Non si può restituire un riferimento a variabile locale:

```rust
fn return_a_string() -> &String {
    let s = String::from("Hello");
    &s  // ERRORE: s viene droppata alla fine della funzione
}
```

---

## Riepilogo

| Tipo reference | Alias | Mutazione | Permessi tolti alla sorgente |
|----------------|-------|-----------|------------------------------|
| `&T` (immutabile) | ✅ sì | ❌ no | W, O |
| `&mut T` (mutabile) | ❌ no | ✅ sì | R, W, O |

**Regole del borrow checker**:
1. Ogni variabile ha permessi R, O (e W se `mut`).
2. Creare una reference trasferisce permessi dalla variabile alla reference.
3. I permessi vengono restituiti alla fine del lifetime della reference.
4. I dati devono sopravvivere a tutte le reference che li puntano.

