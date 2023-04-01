# Wymagania

```
pip install -r requirements.txt

apt-get install zstd
```


# Generowanie bazy danych z pozycjami

### 1. lichess

1. wybrać bazę danych z https://database.lichess.org
2. pobrać plik `.pgn.zst`
3. rozpakować (np. narzędziem `unzstd`)

### 2. stockfish
wybrać, pobrać i rozpakować odpowiednią binarkę Stockfisha: https://stockfishchess.org/download


### 3. ewaluacja pozycji
instrukcja
```
python3 generate_database.py -h
```

# Czytanie bazy danych

```
from utils import read_database

directory = "./database"
white_df, black_df = read_database(directory)
```

