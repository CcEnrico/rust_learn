use std::cmp::PartialEq;

// Per definire una struct:
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

//Possiamo definire una funzione di build per creare un istanza di un utente:
fn build_user(email: String, username: String) -> User {
    // sintassi esplicita:
    // User{
    //     email: email,
    //     username: username,
    //     active: true,
    //     sign_in_count: 1,
    // }

    // La sintassi è più concisa se il nome del campo è uguale al nome del parametro (init shorthand):
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

// Per definire una tuple struct:(sintassi simile a quella di una tupla)
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-like struct:
struct AlwaysEqual;


fn main() {
    // Instanziamo un utente:
    let gonfio = User {
        // L'ordine dei campi è irrilevante, basta specificare il nome del campo
        active: true,
        username: String::from("gonfio89"),
        email: String::from("gonfio89@gmail.com"),
        sign_in_count: 1,
    };

    // È possibile anche gestire campi struct mutabili:
    // NB: non e' possibile etichettare un singolo campo come mutabile, ma solo l'intera struct
    let mut pipoti = User {
        active: true,
        username: String::from("pipoti54"),
        email: String::from("pipoti54@libero.it"),
        sign_in_count: 0,
    };

    // pipoti cambia email:
    pipoti.email = String::from("pipoti54@gmail.com");

    // struct update syntax: per creare una nuova struct a partire da un'altra, copiando i campi che non vengono specificati:
    let pipoti_jr = User {
        email: String::from("pipoti_jr@gmail.com"),
        ..pipoti
    };
    // NB: Quando usi ..pipoti, i campi che implementano il trait Copy (come active e sign_in_count, che sono rispettivamente bool e u64)
    // vengono copiati, mentre i campi che non implementano Copy (come username, che è una String) vengono mossi, trasferendo l' ownership.
    // Dopo questa operazione, non puoi più usare pipoti per i campi mossi, ma puoi ancora accedere ai campi copiati:
    println!("Pipoti è attivo? {}", pipoti.active); // Questo funziona perché active è stato copiato
    // println!("Pipoti username: {}", pipoti.username); // Questo non compila perché username è stato mosso da pipoti a pipoti_jr

    // Tuple Struct:
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("Black is {} ,{}, {}", black.0, black.1, black.2 );

    // NB le tuple struct hanno tipo diverso anche se definite con gli stessi campi e valori:

    let subject = AlwaysEqual;

}
