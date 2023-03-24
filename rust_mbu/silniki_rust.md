Najlepszy silnik rustowy jaki udało mi się znaleźć:

[Velvet Chess](https://github.com/mhonert/velvet-chess) ~3400 ELO (**TOP 38** blitz, **TOP 14** _40/2_, **TOP 27** _40/15_), 

Dla porównania stockfish 15.1 (**TOP 1** blitz, _40/2_, _40/15_) ma ~3750 ELO (blitz)

_40/2_* - 2 minuty na 40 ruchów
_40/15_* - 15 minut na 40 ruchów
źródło: [CCR](https://www.computerchess.org.uk/ccrl/404/rating_list_all.html)

----
Inne silniki: 

https://github.com/pleco-rs/Pleco - Stockfish przepisany na Rust, niestety nie znalazłem benchmarków

https://github.com/jordanbray/chess (tylko generowanie ruchów)

https://github.com/niklasf/shakmaty (tylko generowanie ruchów)

Jest ich wiele więcej, ale wypisałem tylko te najciekawsze.

----

[NPS](https://www.chessprogramming.org/Nodes_per_Second)(nodes per second) nie jest dobrym wskaźnikiem jeśli chodzi o porównywanie **różnych** silików szachowych, bo silniki liczą je na różne sposoby (po użyciu `search`, po użyciu funkcji `evaluate`, po użyciu funkcji `make_move`...), na dodatek wyniki zależą od sprzętu i sposobu testowania (pojedyńczy rdzeń vs wiele rdzeni).

Próba manualnego pomiaru NPS na mojej maszynie (procesor Apple Silicon) także nie dała rzetelnych rezultatów (NPS Stockfisha kilkukrotnie mniejsze od NPS Velveta?!)

---
Inny wskaźnik:
Testy [Perft](https://www.chessprogramming.org/Perft) - Jak szybko (tutaj z pozycji startowej) silnik jest w stanie dojść do wszystkich liści drzewa pozycji na danej głębokości.

![](https://i.imgur.com/uhUTSLX.png)
Źródło: https://github.com/niklasf/shakmaty

----

Wnioski:
- C/C++ wciąż dominuje jeśli chodzi o silniki szachowe, jest tzw. złotym standardem
- Rust to stosunkowo nowy język, ale jego konstrukcja ułatwia wiele kluczowych w tworzeniu silnika rzeczy: memory-safety, zero cost abstraction, łatwy multithreading...
- Da się napisać searcha w ruscie który jest w stanie konkurować z najlepszymi silnikami napisanymi w C/C++
