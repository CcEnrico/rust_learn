

// 1. TIPI OBBLIGATORI
fn greet(name: &str) {
    println!("Ciao {name}");
}

// 2. ORDINE IRRILEVANTE (definita dopo main)
fn call_later() {
    println!("chiamata successiva");
}

// 4. OWNERSHIP
fn consume(s: String) {
    println!("consumato: {s}");
}

fn borrow(s: &String) {
    println!("preso in prestito: {s}");
}

// 5. EXPRESSION vs STATEMENT
fn demo_expr() -> i32 {
    let x = {
        let y = 3;
        y + 1  // espressione: valuta a 4
    };
    x  // ritorno implicito
}

// 6. RITORNO IMPLICITO
fn add(a: i32, b: i32) -> i32 {
    a + b  // no `return` necessario
}

fn early_return(x: i32) -> i32 {
    if x < 0 {
        return 0;  // return esplicito
    }
    x * 2  // ritorno implicito
}

// 8. COPY vs NON-COPY
fn copy_demo(n: i32) {
    println!("copia: {n}");
}

// 10. UNIT TYPE
fn no_return() {
    println!("ritorna ()");
}

fn main() {
    // 2. ordine irrilevante
    call_later();

    // 4. ownership/borrow
    let s = String::from("test");
    consume(s);
    // println!("{s}"); // ERRORE: moved

    let s2 = String::from("test2");
    borrow(&s2);
    println!("{s2}"); // OK

    // 5. expression
    println!("demo_expr: {}", demo_expr());

    // 6. return
    println!("add: {}", add(2, 3));
    println!("early_return(-5): {}", early_return(-5));

    // 7. assegnamento NON è expression
    let mut x = 0;
    let mut y = 0;
    y = 5;
    x = y;
    println!("x={x}, y={y}");

    // 8. Copy
    let num = 42;
    copy_demo(num);
    println!("num ancora valido: {num}");

    // 9. no conversione implicita
    if 5 > 0 {
        println!("solo bool in if");
    }

    // 10. unit
    no_return();
}
