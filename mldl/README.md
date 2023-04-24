# Wymagania

```
pip install -r requirements.txt

apt-get install zstd
```


# Generowanie bazy danych z pozycjami

### 1. lichess

1. wybrać bazę danych z https://database.lichess.org
2. pobrać plik `.pgn.zst`
3. rozpakować narzędziem:
   - `unzstd` i zapisać rozpakowany plik lub
   - `zstdcat` i przekazać na standardowe wejście do skryptu `generate_database.py`

### 2. stockfish
wybrać, pobrać i rozpakować odpowiednią binarkę Stockfisha: https://stockfishchess.org/download


### 3. ewaluacja pozycji
instrukcja
```
python3 generate_database.py -h
```

# Czytanie bazy danych

```python
from utils import read_database, train_test_split_by_game_id, filter_dataset, get_features_and_labels

directory = "/home/database"
score_threshold = 100

white_df, black_df = read_database(directory)

white_df_filtered = filter_dataset(white_df, score_threshold)
black_df_filtered = filter_dataset(black_df, score_threshold)

trn_w, tst_w, trn_b, tst_b = train_test_split_by_game_id(white_df_filtered, black_df_filtered)

X_trn_w, Y_trn_w = get_features_and_labels(trn_w)
X_tst_w, Y_tst_w = get_features_and_labels(tst_w)
X_trn_b, Y_trn_b = get_features_and_labels(trn_b)
X_tst_b, Y_tst_b = get_features_and_labels(tst_b)
```

