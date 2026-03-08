# Il Tipo Slice

## Introduzione

Gli **slice** permettono di riferirsi a una sequenza contigua di elementi in una collezione invece che all'intera collezione. Uno slice è un tipo di riferimento, quindi è un **puntatore non-proprietario**.

## Problema Motivazionale

### Senza Slice

Scrivere una funzione che restituisce la prima parola di una stringa:

```rust
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
```

**Problema**: Restituire solo un `usize` crea problemi di sincronizzazione. L'indice può diventare non valido se la stringa viene modificata:

```rust
fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s); // word = 5
    s.clear(); // s è ora vuota, ma word contiene ancora 5!
    // Bug: word non è più valido!
}
```

## String Slice

### Definizione

Uno **string slice** è un riferimento a parte di una `String`:

```rust
let s = String::from("hello world");

let hello: &str = &s[0..5];   // riferimento a "hello"
let world: &str = &s[6..11];  // riferimento a "world"
let s2: &String = &s;         // riferimento all'intera String
```

### Struttura Interna

Gli slice sono **"fat pointers"** (puntatori con metadata):

```
Stack:
hello   { ptr: ●, len: 5 }
world   { ptr: ●, len: 5 }
s2      { ptr: ● }

Heap:
h e l l o   w o r l d
```

- `&String`: puntatore singolo (8 bytes su 64-bit)
- `&str`: puntatore + lunghezza (16 bytes su 64-bit)

## Sintassi Range

```rust
let s = String::from("hello");

// Inizio implicito (da indice 0)
let slice = &s[0..2];
let slice = &s[..2];     // equivalente

// Fine implicita (fino alla fine)
let len = s.len();
let slice = &s[3..len];
let slice = &s[3..];     // equivalente

// Intera stringa
let slice = &s[0..len];
let slice = &s[..];      // equivalente
```

**Nota**: Gli indici devono essere ai confini validi dei caratteri UTF-8!

## Soluzione con Slice

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

### Vantaggi

1. **Valore legato ai dati**: Lo slice mantiene un riferimento valido
2. **Sicurezza a compile-time**: Il compilatore previene l'uso non valido

```rust
fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);  // crea riferimento immutabile
    
    s.clear(); // ERRORE! Non puoi modificare s mentre esiste word
    println!("{}", word);
}
```

**Errore del compilatore**:
```
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
```

## Permessi e Borrowing

```rust
fn main() {
    let mut s = String::from("hello");
    // s: +R +W +O

    let hello: &str = &s[0..5];
    // s:    R  -W  -O  (perde write e own)
    // hello: +R  -  +O
    // *hello: +R  -  -

    println!("{hello}");
    // s: R +W +O (recupera i permessi)

    s.push_str(" world");
}
```

## String Literals

I **string literals** sono `&str` che puntano al binario:

```rust
let s = "Hello, world!";  // s è di tipo &str
```

Questo è il motivo per cui i string literals sono immutabili: `&str` è un riferimento immutabile.

## Firma Migliorata con &str

### Versione meno flessibile
```rust
fn first_word(s: &String) -> &str {
```

### Versione più flessibile (preferita)
```rust
fn first_word(s: &str) -> &str {
```

### Vantaggi della firma con &str

Permette di passare sia `&String` che `&str`:

```rust
fn main() {
    let my_string = String::from("hello world");

    // Funziona con slice parziali di String
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    
    // Funziona con riferimenti a String
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // Funziona con slice di string literals
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);
    
    // Funziona direttamente con string literals
    let word = first_word(my_string_literal);
}
```

**Nota**: Questo sfrutta le **deref coercions** (conversioni implicite).

## Altri Tipi di Slice

Gli slice non sono specifici per le stringhe, funzionano con qualsiasi collezione:

### Array Slice

```rust
let a = [1, 2, 3, 4, 5];
let slice: &[i32] = &a[1..3];

assert_eq!(slice, &[2, 3]);
```

Questo slice ha tipo `&[i32]` e funziona allo stesso modo degli string slice:
- Memorizza un riferimento al primo elemento
- Memorizza una lunghezza

## Quiz - Punti Chiave

### Dimensioni in Memoria

```rust
fn main() {
    let s = String::from("hello");
    let s2: &String = &s;   // 8 bytes (puntatore singolo)
    let s3: &str = &s[..];  // 16 bytes (puntatore + lunghezza)
}
```

**Risultato**: `s3` occupa più bytes di `s2`.

Verifica con:
```rust
println!(
    "&String={} &str={}",
    std::mem::size_of::<&String>(),
    std::mem::size_of::<&str>(),
);
```

### Borrowing e Mutabilità

```rust
fn main() {
    let mut s = String::from("hello");
    for &item in s.as_bytes().iter() {
        if item == b'l' {
            s.push_str(" world");  // ERRORE!
        }
    }
    println!("{s}");
}
```

**Non compila**: `s.as_bytes()` crea un riferimento immutabile, quindi non si può mutare `s` con `push_str` all'interno del loop.

## Riepilogo

### Caratteristiche Principali

1. **Slice = riferimenti speciali** a sotto-range di sequenze
2. **Rappresentazione runtime**: fat pointer (puntatore + lunghezza)
3. **Sicurezza**: lo slice non può essere invalidato mentre è in uso
4. **Vantaggio principale**: eliminano errori di sincronizzazione a compile-time

### Differenze Chiave

| Tipo | Dimensione | Descrizione |
|------|-----------|-------------|
| `&String` | 8 bytes | Riferimento normale a String |
| `&str` | 16 bytes | Slice di stringa (ptr + len) |
| `&[T]` | 16 bytes | Slice generico (ptr + len) |

### Best Practice

- Preferire `&str` a `&String` nelle firme di funzioni
- Gli indici devono essere ai confini dei caratteri UTF-8
- Gli slice prevengono bug tramite il borrow checker

