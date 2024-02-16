Silnik szachowy w języku Rust in progress

Testowanie:
Silnik posiada kilka pozycji do testowania ogólnej poprawności generowania ruchów.
Można to odpalić przy użyciu `cargo test`.

Budowanie:

Do budowania potrzebny jest kompilator Rust wraz z Cargo.
`https://rustup.rs/`

Komenda: `cargo build --release`

Uruchamianie:
`cargo run --bin engine --release`

Działające komendy UCI:
- position
- go 
- isready
- uci 
- stop 
- quit

Przykładowy input (search z limitem 10sek.):
```
$ cargo run --bin engine --release 
position startpos moves e2e4 e7e5 e1e2 e8e7
go movetime 10000
```
