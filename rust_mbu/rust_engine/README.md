Silnik szachowy w języku Rust in progress

Testowanie:
Silnik posiada kilka pozycji do testowania ogólnej poprawności generowania ruchów.
Można to odpalić przy użyciu `cargo test`.

Budowanie:
`cargo build --release`

Uruchamianie:
`cargo run --bin engine --release`

Działające komendy UCI:
- position
- go 
- isready
- uci 
- stop 
- quit

Heurystyki użyte do redukcji przeszukiwanych pozycji:
1. Alpha beta pruning
2. Delta pruning
3. Late move reduction 
4. Null move pruning
5. Move ordering (LVV-MVA)
6. Killer moves
7. History moves 
8. Transposition table
9. _Static Exchange Evaluation (SEE, in progress)_

Silnik aktualnie jest w stanie zejść na głębokość 15-20 w kilka sekund w zależności od złożoności pozycji.

Dodatkowo na głębokości 0 działa quiescence.

Przykładowy input (search z limitem 10sek.):
```
$ cargo run --bin engine --release 
position startpos moves e2e4 e7e5 e1e2 e8e7
go movetime 10000
```
