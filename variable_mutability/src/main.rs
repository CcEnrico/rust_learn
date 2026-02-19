fn main() {
    // keyword mut
    // immutable di default, con uso della keyword mut la variabile diventa mutabile e quindi posso riassegnare il valore, altrimenti errore a compile time.

    println!("Mutability:");

    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Keyword const
    // le const sono immutabili per definizione
    // la logica è simile a immutability, due differenze la costanza si definisce usando const al posto di let e va definito anche il tipo del dato,
    // le const si usano per definizioni costanti non per variabili calcolate a runtime.
    // Here’s an example of a constant declaration:

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // const a differenza di let permette alla costante di essere definita al di fuori dello scope della funzione.

    // Shadowing:
    // posso riusare con let la stessa variabile andando a ombreggiare quella prima, l'ombreggiatura resta fino a quando una variabile muore o finisce il suo scope,
    // quando muore tipo la 3 variabile ritorna fuori dall'ombreggiatura l'ultima variabile con lo stesso nome, es:

    println!("Shadowing:");

    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");
    // è diverso da avere una variabile mutabile e riassegnare il valore let crea proprio un altra variabile che va a obreggiare la prima, infatti io posso
    // ombreggiare una variabile con una di un altro tipo: (questo ci permette di non dover inventare nomi strani e usare la stessa variabile)
    let spaces = "   ";
    let spaces = spaces.len();
    // se invece provo: (!mismatched types, compile time)
    // let mut spaces = "   ";
    // spaces = spaces.len();
}
