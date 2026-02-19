fn main() {
    // === 1. IF È ESPRESSIONE ===
    let condition = true;
    let number = if condition { 5 } else { 6 };  // assegna risultato
    println!("number: {number}");

    // DIFFERENZA DA C: NO conversione implicita a bool
    let x = 3;
    if x != 0 {  // ok: esplicito
        println!("x != 0");
    }

    // Entrambi i rami devono avere stesso tipo
    // let y = if condition { 5 } else { "sei" };  // ERRORE

    // === 2. LOOP INFINITO ESPLICITO ===
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;  // break con valore
        }
    };
    println!("result: {result}");

    // === 3. LOOP LABEL (annidati) ===
    let mut count = 0;
    'outer: loop {
        let mut remaining = 3;
        loop {
            if count == 2 {
                break 'outer;  // esce dal loop esterno
            }
            if remaining == 0 {
                break;  // esce solo dal loop interno
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("count finale: {count}");

    // === 4. WHILE ===
    let mut n = 3;
    while n != 0 {
        println!("{n}!");
        n -= 1;
    }

    // === 5. FOR (SICURO vs while con indice) ===
    let a = [10, 20, 30, 40, 50];

    // Modo C-like (possibile panic se indice sbagliato)
    // let mut i = 0;
    // while i < 5 { println!("{}", a[i]); i += 1; }

    // Modo Rust idiomatico: for su iterator (NO indici, NO panic)
    for element in a {
        println!("valore: {element}");
    }

    // Range con rev() per countdown
    for n in (1..4).rev() {
        println!("{n}!");
    }
    println!("LIFTOFF!");

    // enumerate() per avere indice+valore
    for (i, val) in a.iter().enumerate() {
        println!("a[{i}] = {val}");
    }

    // === 6. RETURN vs BREAK ===
    fn demo_return() -> i32 {
        loop {
            // return esce dalla funzione
            return 42;
        }
    }
    println!("demo_return: {}", demo_return());
}
