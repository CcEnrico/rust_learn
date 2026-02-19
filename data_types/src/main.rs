fn main() {
    // rust è un linguaggio statically typed quindi se non è chiaro dal contesto bisogna definire il tipo al compilatore
    let _guess: u32 = "42".parse().expect("Not a number!");

    // Trattiamo scalar e compound
    // scalar:

    // Appunti: tipi numerici e correlati (stile commenti Rust)

    // Tipi interi
    // Se non sei sicuro, usa i32 come default. Usare isize/usize principalmente per indicizzare collezioni.
    // Range comuni: i8/u8, i16/u16, i32/u32, i64/u64, i128/u128, isize/usize (arch-dependent).
    // Letterali: 98_222, 0xff, 0o77, 0b1111_0000, b'A'

    // Integer overflow
    // - Debug build: controlli di overflow causano panic a runtime.
    // - Release build (compilato con il flag `--release`): nessun panic, avviene wrapping (two's complement).
    //   Esempio u8: 256 -> 0, 257 -> 1 (wrapping).

    // Non fare affidamento sul wrapping; gestisci esplicitamente overflow con i metodi:
    // - wrapping_* (es. wrapping_add): wrap sempre
    // - checked_*  (es. checked_add): ritorna `Option<T>` (None in caso di overflow)
    // - overflowing_* (es. overflowing_add): ritorna (valore, bool) indicando overflow
    // - saturating_* (es. saturating_add): si ferma AI min/max del tipo

    // Esempi di uso rapido:
    let a: u8 = 255;
    let _wrap = a.wrapping_add(1); // 0
    let _checked = a.checked_add(1); // None
    let (_val, _overflowed) = a.overflowing_add(1); // (0, true)
    let _sat = a.saturating_add(1); // 255

    // Floating-point
    // Tipi: f32, f64. Default: f64 (maggiore precisione).
    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32
    // Rappresentazione: IEEE-754. Tutti i tipi float sono signed.

    // Operazioni numeriche
    // Supportate: +, -, *, /, %
    // Divisione intera tronca verso zero.
    let _sum = 5 + 10;
    let _difference = 95.5 - 4.3;
    let _product = 4 * 30;
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3; // -1
    let _remainder = 43 % 5;

    // Booleano
    // Tipo: bool (1 byte), valori: true, false
    let _t = true;
    let _f: bool = false;

    // Carattere
    // Tipo: char (4 byte), rappresenta uno Unicode scalar value.
    // Esempi:
    let _c = 'z';
    let _z: char = 'ℤ';
    let _heart_eyed_cat = '😻';

    // Compound types:
    // --- Tuple ---
    // Group di valori di tipi diversi, lunghezza fissa.
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Destructuring: scomporre la tuple in variabili
    let (_x, y,_z) = tup;
    // y vale 6.4
    println!("y = {y}");

    // Accesso diretto per indice: .0, .1, .2
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("{}. {}. {}", five_hundred, six_point_four, one);

    // Unit: tuple vuota rappresentata da ()
    let unit: () = ();
    println!("unit vale: {:?}", unit);

    // Tuple mutabile: si possono modificare elementi singoli
    let mut pair: (i32, i32) = (1, 2);
    pair.0 = 0;
    pair.1 += 5;
    println!("pair modificata: ({}, {})", pair.0, pair.1);

    // --- Array ---
    // Tutti gli elementi devono avere lo stesso tipo, lunghezza fissa.
    let a = [1, 2, 3, 4, 5];
    let _b: [i32; 5] = [1, 2, 3, 4, 5]; // con annotazione tipo/len
    let _c = [3; 5]; // inizializza 5 elementi tutti a 3

    // Accesso tramite indice (usize)
    let first = a[0];
    let second = a[1];
    println!("first = {}, second = {}", first, second);

    // Array allocati sullo stack, utili quando la dimensione è nota e immutabile.
    // Se serve una collezione dinamica usare `Vec<T>`.

    // --- Accesso fuori limite (esempio commentato perché panicerebbe) ---
    // Il seguente esempio compilerebbe ma panicerebbe a runtime se eseguito con indici >= len:
    /*
    use std::io;
    let a = [1, 2, 3, 4, 5];
    println!("Inserisci indice di array:");
    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("read failed");
    let index: usize = index.trim().parse().expect("non è un numero");
    let element = a[index]; // se index >= 5 -> panic: index out of bounds
    println!("elemento: {}", element);
    */
    // il fatto che panica è vantaggioso perche non ci permette di raggiungere regioni
    // di memoria che non ci appartengono come ad esempio nel c, è una caratteristica di Rust memory safety.
}
