# Alfa
[TOC]

## UCI
Initial assumptions:
* By GUI I really mean standard python input and output.
* Since the *engine* is very basic (let's be honest, it's non-existent at the moment) every command that UCI waits for is from the stdin.
* Engine is not connected to 'search' at the moment.
* All handling commands (input/output) functions are written in a way that is intended to be simple, but that may be changed in the future. Mainly because of *multiprocessing*, so the code struture may differ significantly from the original.
* Not all commands are implemented due to errors encountered or lack of certainty if they are will be needed.

Attached files:
* **engine.py**
Simple engine that currently don't handle search.
Exists for testing purposes (currently UCI only, even not fully).

* **uci.py**
Handles UCI commands (input/output) described below.

* **commands_data.py**
Contains dataclasses used in UCI communication.
Basicly holds data for specific commands.

### How it currently works:
$$GUI\\
\updownarrow\\
ENGINE \dashrightarrow SEARCH\\
\updownarrow\\
UCI$$

### How it should work:
$$GUI\\
\updownarrow\\
UCI\leftrightarrow ENGINE\leftrightarrow SEARCH$$

### Input Commands [GUI to ENGINE]:
#### uci
Command used to initialize the engine, let it know to use UCI protocol and set up communication with chess engine.
Usage:
```=
uci
```

#### isready
This command is sent to check if the engine is ready to receive commands.
Usage:
```=
isready
```

#### setoption
Command used to set specific engine options.
Usage:
```=
setoption name <option_name> value <option_value>
```

#### ucinewgame [NOT SURE IF IS NEEDED]
Command used to notify the engine when a new game is about to start.
Usage:
```=
ucinewgame
```

#### position
Command used to set the position on the chessboard.
Usage:
*   ```=
    position startpos
    ```
    Sets up the chessboard with the standard starting positions.
*   ```=
    position fen <FEN>
    ```
    Where `<FEN>` must be replaced with actual FEN string representing the desired state of the game.
    Example below sets up the standard starting positions:
    ```=
    position fen rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
    ```
*   ```=
    position [fen <FEN>] moves <move_1> <move_2> ...
    ```
    `fen <FEN>` is optional and allows to specify the initial state of the game, while sequence of `<move_x>` are moves played on the board.
    Example:
    ```=
    position fen r1bqkb1r/ppp2ppp/2n1pn2/4N3/2Pp4/5N2/PP2PPPP/R1BQKB1R b KQkq - 1 7 moves e4 e5 g3 Nf6 Nf3 Nxe4 Nxe5 Qe7
    ```
    Example without fen:
    ```=
    position moves e4 e5 g3 Nf6 Nf3 Nxe4 Nxe5 Qe7
    ```

#### go
Command used to inform the engine to start searching for the best move.
Usage:
*   ```=
    go
    ```
    Starts engine's search with no extra parameters, i.e. it will start engine with default settings.
*   ```=
    go <parameter> [<value>]
    ```
    Example:
    ```=
    go depth 8
    ```

*   ```=
    go <parameter_1> <value_1> <parameter_2> <value_2> ...
    ```
    Format with more than one parameter.

#### stop
Used to inform the engine to stop search and return the best move found so far.
Usage:
```=
stop
```

#### ponderhit [NOT SURE IF NEEDED]
Used when opponent played the move that the engine was considering as a possible reply during opponent's turn.
Usage:
```=
ponderhit
```

#### quit
Used to close engine's process and  connection (to GUI).
Usage:
```=
quit
```

### Output Commands [ENGINE to GUI]:
#### id
Respose to the 'uci' command.
Used to identify the engine.
Usage:
```=
id name <engine_name>
id author <author_name>
```

#### uciok
Respose to the 'uci' command.
Informs that engine initialization was successfully completed and is ready to receive commands.
Usage:
```=
uciok
```

#### readyok
Respose to the 'isready' command.
Command sent to inform that synchronisation between engine and GUI was successfully completed and is ready to receive commands.
Usage:
```=
readyok
```

#### bestmove
Command sent to communicate the best move it has found after the search is complete.
Usage:
*   ```=
    bestmove <move>
    ```
    Best move found: `<move>`.
    Example:
    ```=
    bestmove b2b4
    ```
*   ```=
    bestmove <move> ponder <ponder_move>
    ```
    Format used when engine has been pondering opponent's move and provides the move considered during opponent's turn.
    Example:
    ```=
    bestmove b2b4 ponder b7b5
    ```

#### info [NOT SURE IF NEEDED]
Command sent to inform about search process and current position evaluation.
Usage:
```=
info <parameter_1> <value_1> <parameter_2> <value_2> ...
```
Example:
```=
info depth 2 score cp 214 time 1242 nodes 2124 nps 34928 pv e2e4 e7e5 g1f3
```
Inform that engine's search process reached depth of 8, evaluation score is 50 cp (centipawns) in favor of the engine, pv (principal variation) being considered is e2e4 e7e5 and current move number is 3.

Explaination:
Centipawn - [here](https://chess.fandom.com/wiki/Centipawn) or [here](https://www.chessprogramming.org/Centipawns).
Principal variation - [here](https://www.chessprogramming.org/Principal_Variation).