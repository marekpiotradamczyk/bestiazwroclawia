Silnik szachowy w języku Rust in progress

TODO: 
- Naprawić wykrywanie move repetition
- Dokończyć UCI
- Dodać lepszego searcha i evala

Testowanie:
Silnik posiada kilka pozycji do testowania ogólnej poprawności generowania ruchów.
Można to odpalić przy użyciu `cargo test`.

Silnik jeszcze nie ma skończonego protokołu UCI, więc aby przetestować jak działa w akcji,
trzeba pewne rzeczy shardcodować.

Przykłady:
Dodałem przykładowy kod, który zaczyna selfplaya na startowej pozycji i głębokości 3.
Można uruchomić komendą `cargo run --example selfplay`

Budowanie:
`cargo build --release`

Uruchamianie:
`cargo run --bin engine`
