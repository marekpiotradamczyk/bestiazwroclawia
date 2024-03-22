import chess
import chess.pgn
from typing import Generator, List, IO, BinaryIO, Tuple
from dataclasses import dataclass, field
import os
import json
import itertools
import bisect
import random

@dataclass
class DatasetSettings(object):
    number_of_games : int = 0
    number_of_moves : int = 0
    number_of_files : int = 0
    games_per_file : int = 1000
    """
    total_number_of_positions[n] equals to 
    total number of positions in files [0..n]
    """
    total_number_of_positions : List[int] = field(default_factory=list)
    encoding : int = 0

class EncodingType(object):
    def __init__(self):
        raise Exception("__init__ not implemented!")

    def encode_pgn(self, game : chess.pgn.Game) -> bytes:
        raise Exception("encode_pgn(self, game : chess.pgn.Game) -> bytes not implemented!")

    def decode_to_pgn(self, encoded_game : bytes) -> chess.pgn.Game:
        raise Exception("decode_to_pgn(self, encoded_game : bytes) -> chess.pgn.Game not implemented!")
    

class SimpleEncoder(EncodingType):
    def __init__(self):
        self.move_size = 8
    
    def encode_pgn(self, game: chess.pgn.Game) -> bytes:
        #3 bytes to encode one move
        def encode_move(m : chess.Move) -> bytes:
            square_nr = lambda x: (ord(x[1]) - ord('0') - 1) * 8 + ord(x[0]) - ord('a')
            s = m.uci()
            first_square = square_nr(s[:2]).to_bytes(1, 'big')
            second_square = square_nr(s[2:4]).to_bytes(1, 'big')
            promotion = ord(s[4]).to_bytes(1, 'big') if len(s) > 4 else int(0).to_bytes(1, 'big')
            res = first_square + second_square + promotion
            return res
        if game.headers['Result'] == '1-0':
            result = 2
        elif game.headers['Result'] == '0-1':
            result = 0
        elif game.headers['Result'] == '1/2-1/2':
            result = 1
        else:
            result = random.randint(0, 2)
        result = result.to_bytes(1, 'big')
        return result + b"".join([encode_move(move) for move in game.mainline_moves()])

    def decode_to_pgn(self, encoded_game: bytes) -> chess.pgn.Game:
        def decode_move(m : bytes) -> str:
            first_square_nr = m[0]
            second_square_nr = m[1]
            promotion_nr = m[2]
            nr_to_str = lambda x: chr(ord('a') + x%8) + chr(ord('0') + x//8 + 1)
            res = nr_to_str(first_square_nr) + nr_to_str(second_square_nr)
            if promotion_nr:
                res += chr(promotion_nr)
            return res
        result = encoded_game[0]
        if result == 0:
            result = '0-1'
        elif result == 2:
            result = '1-0'
        else:
            result = '1/2-1/2'
        moves = [encoded_game[i:i+3] for i in range(1, len(encoded_game), 3)]
        moves = list(map(decode_move, moves))
        board = chess.Board()
        for move in moves:
            board.push(chess.Move.from_uci(move))
        game = chess.pgn.Game.from_board(board)
        game.headers['Result'] = result
        return game

def _bytes_to_int(x : bytes) -> int:
    return int.from_bytes(x, byteorder='big')

"""
Data is stored in data{i}.bin files. Each file has a following format:
On first 4 bytes: n - number_of_games in the file.
Next 4 bytes - size of the file
Next 8 * n bytes: offsets, sizes and numbers of moves of the games in the file in format 4:2:2 (bytes).
The rest of the file contains games encoded as specified by EncodingType.
"""
class DatasetManager(object):
    _decoders : List[EncodingType]  = [SimpleEncoder()]

    def __init__(self, path_to_dataset : str, create_empty=False, settings=None):
        self.path = path_to_dataset
        os.makedirs(self.path, exist_ok=True)
        if not settings:
            settings = DatasetSettings()
        if create_empty:
            self._create_new_dataset(settings)
        else:
            self._load_settings()
        self.decoder = DatasetManager._decoders[self.settings.encoding]
        
    def _create_new_dataset(self, settings : DatasetSettings):
        self.settings = settings
        with open(os.path.join(self.path, 'settings.json'), 'w') as f:
            json.dump(settings.__dict__, f)

    def _load_settings(self):
        with open(os.path.join(self.path, 'settings.json'), 'r') as f:
            self.settings = DatasetSettings(**json.loads(f.read()))

    def _save_settings(self):
        with open(os.path.join(self.path, 'settings.json'), 'w') as f:
            json.dump(self.settings.__dict__, f)

    def _open_data_file(self, n : int, mode : str = 'rb') -> BinaryIO:
        file = open(os.path.join(self.path, f'data{n}.bin'), mode)
        return file
    
    def _create_data_file(self, n : int):
        file = self._open_data_file(n, 'wb')
        file.write(int(0).to_bytes(length=4, byteorder='big'))
        file.write(int(8).to_bytes(length=4, byteorder='big'))
        file.close()
    
    #reads at most x games from the file
    def _read_x_games(file, x : int) -> List[chess.pgn.Game]:
        games = []
        for i in range(x):
            game = chess.pgn.read_game(file)
            if game == None:
                break
            games.append(game)
        return games

    @dataclass
    class FileHeader(object):
        number_of_games : int
        size : int
        game_offsets : List[int]
        game_sizes : List[int]
        game_move_count : List[int]

    @dataclass
    class FileSections(object):
        number_of_games : int
        size : int
        table_start : int
        table_end : int
        game_offsets : List[int]
        game_sizes : List[int]
        game_move_count : List[int]
        games_encoding : bytes

    def _get_dataset_file_sections(content : bytes) -> FileSections:
        number_of_games = _bytes_to_int(content[0:4])
        file_size = _bytes_to_int(content[4:8])
        game_table_start = 8
        game_table_end = 8 * number_of_games + game_table_start
        offsets = [_bytes_to_int(content[i:i+4]) for i in range(game_table_start, game_table_end, 8)]
        sizes = [_bytes_to_int(content[i+4:i+6]) for i in range(game_table_start, game_table_end, 8)]
        move_count = [_bytes_to_int(content[i+6:i+8]) for i in range(game_table_start, game_table_end, 8)]
        games_encoding = content[game_table_end:]
        return DatasetManager.FileSections(
                number_of_games=number_of_games,
                size=file_size,
                table_start=game_table_start,
                table_end=game_table_end,
                game_offsets=offsets,
                game_sizes=sizes,
                game_move_count=move_count,
                games_encoding=games_encoding)
    """
    Reads the header of the file.
    After call to this function file read offset is at the end of the table.
    """
    def _read_header(file : BinaryIO) -> FileHeader:
        # Maybe I should add file.seek(0)
        number_of_games = _bytes_to_int(file.read(4))
        size = _bytes_to_int(file.read(4))
        game_table_size = 8 * number_of_games
        content = file.read(game_table_size)
        offsets = [_bytes_to_int(content[i:i+4]) for i in range(0, game_table_size, 8)]
        sizes = [_bytes_to_int(content[i+4:i+6]) for i in range(0, game_table_size, 8)]
        move_count = [_bytes_to_int(content[i+6:i+8]) for i in range(0, game_table_size, 8)]
        return DatasetManager.FileHeader(
            number_of_games=number_of_games,
            size=size,
            game_offsets=offsets,
            game_sizes=sizes,
            game_move_count=move_count)

    def _write_dataset_file_sections(self, n : int, sections : FileSections):
        with self._open_data_file(n, 'wb') as file:
            file.write(sections.number_of_games.to_bytes(4, 'big'))
            file.write(sections.size.to_bytes(4, 'big'))
            merge_values = lambda x, y, z: x.to_bytes(4, 'big') + y.to_bytes(2, 'big') + z.to_bytes(2, 'big')
            game_table = list(map(merge_values, sections.game_offsets, 
                                  sections.game_sizes, sections.game_move_count))
            file.write(b"".join(game_table))
            file.write(sections.games_encoding)
            file.close()

    def _append_games_to_file(self, n : int, games : List[chess.pgn.Game]):
        with self._open_data_file(n, 'rb') as file:
            file_content = file.read()
            file.close()
        sections = DatasetManager._get_dataset_file_sections(file_content)
        new_move_count = [len(list(game.mainline_moves())) for game in games]
        new_games_encoding = list(map(self.decoder.encode_pgn, games))
        new_sizes = list(map(len, new_games_encoding))
        # Each table entry is 8 bytes
        # We add len(games) entries
        number_of_new_games = len(games)
        new_entries_size = 8 * number_of_new_games
        sections.table_end += new_entries_size
        sections.game_sizes.extend(new_sizes)
        sections.game_move_count.extend(new_move_count)
        # Each game's offset is shifted by the size of new entries
        sections.game_offsets = [x + new_entries_size for x in sections.game_offsets]
        # Offset of the first new game is the previous file size + 8 * number of new games
        # Since we add len(games) entries before game encodings 
        sections.game_offsets.extend(itertools.accumulate(new_sizes, initial=sections.size + new_entries_size))
        # We pop the reduntant offset (that would be an offset of a new game)
        sections.game_offsets.pop()
        sections.number_of_games += number_of_new_games
        sections.games_encoding += b"".join(new_games_encoding)
        sections.size += new_entries_size + sum(new_sizes)
        assert len(sections.game_offsets) == len(sections.game_sizes), \
        "there should be the same amount of offsets and sizes"
        assert len(sections.game_offsets) == sections.number_of_games, \
        "each offset corresponds to one game"
        assert sections.size == 8 + sections.number_of_games * 8 + sum(sections.game_sizes), \
        "size of the file should be 8 + 8 * number_of_games + size of the games' encoding"
        # Update the dataset settings
        self.settings.number_of_games += len(games)
        new_positions = sum(new_move_count)
        self.settings.number_of_moves += new_positions
        self.settings.total_number_of_positions[n] += new_positions
        # Update the file
        self._write_dataset_file_sections(n, sections)

    def add_pgn_file(self, path : str):
        with open(path) as pgn:
            end_of_games = False
            while not end_of_games:
                max_number_of_games = self.settings.games_per_file * self.settings.number_of_files
                if self.settings.number_of_games == max_number_of_games:
                    self._create_data_file(self.settings.number_of_files)
                    self.settings.total_number_of_positions.append(self.settings.number_of_moves)
                    self.settings.number_of_files += 1
                    max_number_of_games += self.settings.games_per_file
                print(f'max_number_of_games: {max_number_of_games}')
                next_chunk_size = max_number_of_games - self.settings.number_of_games
                chunk = DatasetManager._read_x_games(pgn, next_chunk_size)
                print(f'Chunk len: {len(chunk)}')
                if len(chunk) < next_chunk_size:
                    end_of_games = True
                last_file = self.settings.number_of_files-1
                self._append_games_to_file(last_file, chunk)
                print(f'number_of_positions: {self.settings.number_of_moves}')
        self._save_settings()

    def add_multiple_pgn_files(self, paths : List[str]) -> None:
        pass

    def add_game(self, game : chess.pgn.Game):
        pass

    def game_by_position_id(self, pos_id : int) -> Tuple[int, chess.pgn.Game]:
        if pos_id >= self.settings.number_of_moves or pos_id < 0:
            raise Exception("game_by_position_id(self, pos_id : int) error: pos_id out of bounds")
        file_id = bisect.bisect_left(self.settings.total_number_of_positions, pos_id + 1)
        file = self._open_data_file(file_id, 'rb')
        if file_id > 0:
            pos_id -= self.settings.total_number_of_positions[file_id-1]
        header = DatasetManager._read_header(file)
        game_id = 0
        while pos_id >= header.game_move_count[game_id]:
            pos_id -= header.game_move_count[game_id]
            game_id += 1
            assert game_id < header.number_of_games, "We can never run out of moves"
        file.seek(header.game_offsets[game_id])
        game_content = file.read(header.game_sizes[game_id])
        decoded_game = self.decoder.decode_to_pgn(game_content)
        file.close()
        return pos_id, decoded_game
    
    def games(self, start : int, end : int = None) -> Generator[chess.pgn.Game, None, None]:
        if not end:
            end = start
        if end < start:
            return None
        start = max(0, start)
        end = min(end, self.settings.number_of_games-1)
        first_file = start // self.settings.games_per_file
        last_file = end // self.settings.games_per_file
        for f in range(first_file, last_file+1):
            first_game = 0
            last_game = self.settings.games_per_file - 1
            if f == first_file:
                first_game = start % self.settings.games_per_file
            if f == last_file:
                last_game = end % self.settings.games_per_file
            nr_games = last_game - first_game + 1
            file = self._open_data_file(f, 'rb')
            file.seek((first_game+1)*8, 0)
            t = file.read(8 * nr_games)
            game_table = [(_bytes_to_int(t[i:i+4]), _bytes_to_int(t[i+4:i+6]), _bytes_to_int(t[i+6:i+8])) 
                          for i in range(0, 8 *nr_games, 8)]
            file.seek(game_table[0][0])
            for o, s, c in game_table:
                yield self.decoder.decode_to_pgn(file.read(s))
            file.close()
