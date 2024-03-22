#ifndef MATH_H
#define MATH_H

// get board column that a square is part of
#define COL(sq)  ( (sq) & 7 )

// get board row that a square is part of
#define ROW(sq) ( (sq) >> 3 )

// generate square number from row and column
#define SET_SQ(col, row) (row * 8 + col)

#define COL_CHAR(sq) ((char)('a' + COL(sq)))
#define ROW_CHAR(sq) ((char)('1' + ROW(sq)))
#define COL_INT(sq) ((int)(sq - 'a'))
#define ROW_INT(sq) ((int)(sq - '1'))

#endif // MATH_H
