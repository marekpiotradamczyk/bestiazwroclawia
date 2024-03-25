## Silnik szachowy morphebot

Autor: Mateusz Burdyna

Zobacz go w akcji na lichess: [@/morphe157bot/tv](https://lichess.org/@/morphe157bot/tv)

Silnik hostowany jest na maszynce wydzialowej, komunikacja z lichessem odbywa się z pomocą [lichess-bot api](https://github.com/lichess-bot-devs/lichess-bot)


## Uruchamianie i budowanie:

Do budowania / uruchomienia potrzebny jest kompilator Rust wraz z Cargo.
`https://rustup.rs/`

### Uruchamianie

Najprościej i najszybciej
`cargo run --bin engine --release`

### Budowanie
Komenda: `cargo build --release`
Silnik `target/release/engine`

Skompilowany w ten sposób silnik można uruchamiać z commendline'a:
`./engine`, ale w katalogu z executable muszą znajdować się pliki:
- `rooks_magics.bin`
- `bishops_magics.bin`

### Testy
Komenda `cargo test`

## Interfejs

Działające komendy UCI (Universal Chess Interface):
- position
- go 
- d
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

Przykładowy input (search bez limitu):
```
$ cargo run --bin engine --release 
position fen 8/1R2bppk/4p1bp/4P3/2qN4/2P2PBP/2P3PK/r2R4 w - - 0 42
go
stop    // Gdy nam się już znudzi
```

Przykładowy input (search na określoną glebokość):
```
$ cargo run --bin engine --release 
position startpos
go depth 8
```

## Trochę o strukturze projektu

Projekt cargo sklada się z kilku części:
- `sdk` - Framework morphebota, zawiera reprezentacje szachownicy, pól, figur itp.
- `move-gen` - Modul odpowiedzialny za generowanie legalnych ruchów, zawiera reprezentacje ruchów (Move)
- `magic` - Binarka slużąca do wygenerowania plików `rooks_magics` i `bishops_magics`.
- `engine` - Binarka implementująca UCI, Searcha i Evala (wlaściwy silnik)

Logika dot. przeszukiwania znajduje się niemal w calości w `engine/src/search/mod.rs`
Logika dot. ewaluacji znajduje się niemal w calości w `engine/src/eval/mod.rs`

## Gra lokalna

Granie w terminalu jest bardzo niewygodne,
dlatego najlepiej pobrać jakąś implementacje areny szachowej (np. Banksiagui na MacOS) i dodać do niej executable silnika,
w ten sposób można stworzyć mecz Morphebot vs Czlowiek, a nawet Morphebot vs Morphebot lub Morphebot vs Stockfish
