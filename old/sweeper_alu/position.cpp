#include "position.h"

#include<cassert>
#include<algorithm>
#include<iostream>
#include<unordered_map>

#include "math.h"
#include "types.h"

const int HASH_SIZE = 4*8388608;

int piece_val[] = {0, 900, 500, 300, 300, 100, 0};

Bitboard KingMovesBitboard[] = {
	770ULL, 1797ULL, 3594ULL, 7188ULL, 14376ULL, 28752ULL, 57504ULL, 49216ULL,
	197123ULL, 460039ULL, 920078ULL, 1840156ULL, 3680312ULL, 7360624ULL, 14721248ULL, 12599488ULL,
	50463488ULL, 117769984ULL, 235539968ULL, 471079936ULL, 942159872ULL, 1884319744ULL, 3768639488ULL, 3225468928ULL,
	12918652928ULL, 30149115904ULL, 60298231808ULL, 120596463616ULL, 241192927232ULL, 482385854464ULL, 964771708928ULL, 825720045568ULL,
	3307175149568ULL,	7718173671424ULL, 15436347342848ULL, 30872694685696ULL, 61745389371392ULL, 123490778742784ULL, 246981557485568ULL, 211384331665408ULL,
	846636838289408ULL, 1975852459884544ULL, 3951704919769088ULL, 7903409839538176ULL, 15806819679076352ULL, 31613639358152704ULL, 63227278716305408ULL, 54114388906344448ULL,
	216739030602088448ULL, 505818229730443264ULL, 1011636459460886528ULL, 2023272918921773056ULL, 4046545837843546112ULL, 8093091675687092224ULL, 16186183351374184448ULL, 13853283560024178688ULL,
	144959613005987840ULL, 362258295026614272ULL, 724516590053228544ULL, 1449033180106457088ULL, 2898066360212914176ULL, 5796132720425828352ULL, 11592265440851656704ULL, 4665729213955833856ULL
};

Bitboard KnightMovesBitboard[] = {
	132096ULL, 329728ULL, 659712ULL, 1319424ULL, 2638848ULL, 5277696ULL, 10489856ULL, 4202496ULL,
	33816580ULL, 84410376ULL, 168886289ULL, 337772578ULL, 675545156ULL, 1351090312ULL, 2685403152ULL, 1075839008ULL,
	8657044482ULL, 21609056261ULL, 43234889994ULL, 86469779988ULL, 172939559976ULL, 345879119952ULL, 687463207072ULL, 275414786112ULL,
	2216203387392ULL, 5531918402816ULL, 11068131838464ULL, 22136263676928ULL, 44272527353856ULL, 88545054707712ULL, 175990581010432ULL, 70506185244672ULL,
	567348067172352ULL,	1416171111120896ULL, 2833441750646784ULL, 5666883501293568ULL, 11333767002587136ULL, 22667534005174272ULL, 45053588738670592ULL, 18049583422636032ULL,
	145241105196122112ULL, 362539804446949376ULL, 725361088165576704ULL, 1450722176331153408ULL, 2901444352662306816ULL, 5802888705324613632ULL, 11533718717099671552ULL, 4620693356194824192ULL,
	288234782788157440ULL, 576469569871282176ULL, 1224997833292120064ULL, 2449995666584240128ULL, 4899991333168480256ULL, 9799982666336960512ULL, 1152939783987658752ULL, 2305878468463689728ULL,
	1128098930098176ULL, 2257297371824128ULL, 4796069720358912ULL, 9592139440717824ULL, 19184278881435648ULL, 38368557762871296ULL, 4679521487814656ULL, 9077567998918656ULL
};

Bitboard WhitePawnsCapturesBitboard[] = {
	512ULL, 1280ULL, 2560ULL, 5120ULL, 10240ULL, 20480ULL, 40960ULL, 16384ULL,
	131072ULL, 327680ULL, 655360ULL, 1310720ULL, 2621440ULL, 5242880ULL, 10485760ULL, 4194304ULL,
	33554432ULL, 83886080ULL, 167772160ULL, 335544320ULL, 671088640ULL, 1342177280ULL, 2684354560ULL, 1073741824ULL,
	8589934592ULL, 21474836480ULL, 42949672960ULL, 85899345920ULL, 171798691840ULL, 343597383680ULL, 687194767360ULL, 274877906944ULL,
	2199023255552ULL, 5497558138880ULL, 10995116277760ULL, 21990232555520ULL, 43980465111040ULL, 87960930222080ULL, 175921860444160ULL, 70368744177664ULL,
	562949953421312ULL, 1407374883553280ULL, 2814749767106560ULL, 5629499534213120ULL, 11258999068426240ULL, 22517998136852480ULL, 45035996273704960ULL, 18014398509481984ULL,
	144115188075855872ULL, 360287970189639680ULL, 720575940379279360ULL, 1441151880758558720ULL, 2882303761517117440ULL, 5764607523034234880ULL, 11529215046068469760ULL, 4611686018427387904ULL,
	0ULL, 0ULL, 0ULL, 0ULL, 0ULL, 0ULL, 0ULL, 0ULL
};

Bitboard BlackPawnsCapturesBitboard[] = {
	0ULL, 0ULL, 0ULL, 0ULL, 0ULL, 0ULL, 0ULL, 0ULL,
	2ULL, 5ULL, 10ULL, 20ULL, 40ULL, 80ULL, 160ULL, 64ULL,
	512ULL, 1280ULL, 2560ULL, 5120ULL, 10240ULL, 20480ULL, 40960ULL, 16384ULL,
	131072ULL, 327680ULL, 655360ULL, 1310720ULL, 2621440ULL, 5242880ULL, 10485760ULL, 4194304ULL,
	33554432ULL, 83886080ULL, 167772160ULL, 335544320ULL, 671088640ULL, 1342177280ULL, 2684354560ULL, 1073741824ULL,
	8589934592ULL, 21474836480ULL, 42949672960ULL, 85899345920ULL, 171798691840ULL, 343597383680ULL, 687194767360ULL, 274877906944ULL,
	2199023255552ULL, 5497558138880ULL, 10995116277760ULL, 21990232555520ULL, 43980465111040ULL, 87960930222080ULL, 175921860444160ULL, 70368744177664ULL,
	562949953421312ULL, 1407374883553280ULL, 2814749767106560ULL, 5629499534213120ULL, 11258999068426240ULL, 22517998136852480ULL, 45035996273704960ULL, 18014398509481984ULL
};

Bitboard RookOccupancyBitboard[] = {
	282578800148862ULL, 565157600297596ULL, 1130315200595066ULL, 2260630401190006ULL, 4521260802379886ULL, 9042521604759646ULL, 18085043209519166ULL, 36170086419038334ULL,
	282578800180736ULL, 565157600328704ULL, 1130315200625152ULL, 2260630401218048ULL, 4521260802403840ULL, 9042521604775424ULL, 18085043209518592ULL, 36170086419037696ULL,
	282578808340736ULL, 565157608292864ULL, 1130315208328192ULL, 2260630408398848ULL, 4521260808540160ULL, 9042521608822784ULL, 18085043209388032ULL, 36170086418907136ULL,
	282580897300736ULL, 565159647117824ULL, 1130317180306432ULL, 2260632246683648ULL, 4521262379438080ULL, 9042522644946944ULL, 18085043175964672ULL, 36170086385483776ULL,
	283115671060736ULL, 565681586307584ULL, 1130822006735872ULL, 2261102847592448ULL, 4521664529305600ULL, 9042787892731904ULL, 18085034619584512ULL, 36170077829103616ULL,
	420017753620736ULL, 699298018886144ULL, 1260057572672512ULL, 2381576680245248ULL, 4624614895390720ULL, 9110691325681664ULL, 18082844186263552ULL, 36167887395782656ULL,
	35466950888980736ULL, 34905104758997504ULL, 34344362452452352ULL, 33222877839362048ULL, 30979908613181440ULL, 26493970160820224ULL, 17522093256097792ULL, 35607136465616896ULL,
	9079539427579068672ULL, 8935706818303361536ULL, 8792156787827803136ULL, 8505056726876686336ULL, 7930856604974452736ULL, 6782456361169985536ULL, 4485655873561051136ULL, 9115426935197958144ULL
};

Bitboard BishopOccupancyBitboard[] = {
	18049651735527936ULL, 70506452091904ULL, 275415828992ULL, 1075975168ULL, 38021120ULL, 8657588224ULL, 2216338399232ULL, 567382630219776ULL,
	9024825867763712ULL, 18049651735527424ULL, 70506452221952ULL, 275449643008ULL, 9733406720ULL, 2216342585344ULL, 567382630203392ULL, 1134765260406784ULL,
	4512412933816832ULL, 9024825867633664ULL, 18049651768822272ULL, 70515108615168ULL, 2491752130560ULL, 567383701868544ULL, 1134765256220672ULL, 2269530512441344ULL,
	2256206450263040ULL, 4512412900526080ULL, 9024834391117824ULL, 18051867805491712ULL, 637888545440768ULL, 1135039602493440ULL, 2269529440784384ULL, 4539058881568768ULL,
	1128098963916800ULL, 2256197927833600ULL, 4514594912477184ULL, 9592139778506752ULL, 19184279556981248ULL, 2339762086609920ULL, 4538784537380864ULL, 9077569074761728ULL,
	562958610993152ULL, 1125917221986304ULL, 2814792987328512ULL, 5629586008178688ULL, 11259172008099840ULL, 22518341868716544ULL, 9007336962655232ULL, 18014673925310464ULL,
	2216338399232ULL, 4432676798464ULL, 11064376819712ULL, 22137335185408ULL, 44272556441600ULL, 87995357200384ULL, 35253226045952ULL, 70506452091904ULL,
	567382630219776ULL, 1134765260406784ULL, 2832480465846272ULL, 5667157807464448ULL, 11333774449049600ULL, 22526811443298304ULL, 9024825867763712ULL, 18049651735527936ULL
};

Bitboard RowBitboard[] = {
	255ULL, 65280ULL, 16711680ULL, 4278190080ULL, 1095216660480ULL, 280375465082880ULL, 71776119061217280ULL, 18374686479671623680ULL
};

U64 RookMagic[] = {
	0x380008020400014ULL,
  0x20084000110120ULL,
  0x20000802640820ULL,
  0x40140a0800403000ULL,
  0x8008010340840200ULL,
  0xc440010400008200ULL,
  0x2308020000800100ULL,
  0x5080032100014880ULL,
  0x3080080010410ULL,
  0x4040008e281104ULL,
  0xc01900800804006ULL,
  0x520030604008840ULL,
  0x400402404900ULL,
  0x80022041240c0004ULL,
  0x8002040406000058ULL,
  0x202400028008840ULL,
  0x808300804004201ULL,
  0x4022020809000ULL,
  0x8020002008440890ULL,
  0x500020008408410ULL,
  0x2048400900423400ULL,
  0x48851001014080ULL,
  0x40300300880ULL,
  0x48900480102108c0ULL,
  0x1082200040a4ULL,
  0x8410000822012000ULL,
  0x2010500406000cULL,
  0x8000900484850008ULL,
  0x800044040401000aULL,
  0x210a804040803ULL,
  0x2000408640028060ULL,
  0x8e0a00120040840ULL,
  0x400080800012ULL,
  0x5aa4024000400854ULL,
  0x400410800c01003ULL,
  0xa40800602800c04ULL,
  0x28190d0102280081ULL,
  0x100019104040002ULL,
  0x41410014080a80ULL,
  0x40028000c2200412ULL,
  0x8000203480422000ULL,
  0x8080881040002001ULL,
  0x100080a414004ULL,
  0x8421044030080800ULL,
  0x2910204100808ULL,
  0x40808020090ULL,
  0x4004209220401ULL,
  0xc299000e004801ULL,
  0x8800012c0002080ULL,
  0x2040036002c4ULL,
  0x105008801011008ULL,
  0x48004a401440ULL,
  0x8008c00b22045440ULL,
  0x200000c40009c040ULL,
  0x80281024010402ULL,
  0x100844a010410080ULL,
  0x888014800300c421ULL,
  0x80104000800901ULL,
  0x1301a0080409eULL,
  0x210500101200409ULL,
  0x1020112a000402ULL,
  0xb801104208a0002ULL,
  0x8002c1c8a004007ULL,
  0x2028004104008162ULL
};

U64 BishopMagic[] = {
	0x1c0008100e044801ULL,
  0x52240150404820ULL,
  0x10a4300100a8ULL,
  0x1400202640400000ULL,
  0x406802050020000ULL,
  0xa142001160c082ULL,
  0x800041111012000ULL,
  0x40918054800ULL,
  0x4180a03000504cULL,
  0x2002820140888080ULL,
  0x40201000024200d0ULL,
  0x804a02230000ULL,
  0x402024110420ULL,
  0x201450410004ULL,
  0xc010010040a8800ULL,
  0x4880000090680600ULL,
  0x40202004080a010cULL,
  0x805012003140290ULL,
  0x802012104029cULL,
  0x4000009002000ULL,
  0x20101220048200ULL,
  0x440800a09100ULL,
  0x100806400801ULL,
  0x80801080020c0918ULL,
  0x41104c0060008ULL,
  0x1024c01000080011ULL,
  0x90301080512ULL,
  0x14240000441001ULL,
  0x203024a00051008ULL,
  0x20810a80c3241d4ULL,
  0x1120022050b80090ULL,
  0x210200080800ULL,
  0x200100080012ULL,
  0x108020001a00ULL,
  0x8120102408480001ULL,
  0x14022100009200ULL,
  0x200a1114028c1ULL,
  0x10000200041040ULL,
  0x10c081c890200410ULL,
  0x8c00810040810a4ULL,
  0x4408010b88016044ULL,
  0x140014c120102000ULL,
  0x82c829011104ULL,
  0x4118108822011010ULL,
  0x9041080102291ULL,
  0x1080a08c800801c0ULL,
  0xc0428450200840ULL,
  0x2040048201101ULL,
  0x800480440102ULL,
  0x80140080c020a808ULL,
  0x804420400080208ULL,
  0x2108804009000800ULL,
  0x404281820008400ULL,
  0x1200090812400248ULL,
  0x901080004000700ULL,
  0x4012100802000843ULL,
  0x4021202329000ULL,
  0x2000408604880100ULL,
  0x11420200a00ULL,
  0x44001810608010ULL,
  0x280120091042184ULL,
  0x4004000020042322ULL,
  0x808023008010010ULL,
  0x10140040040004ULL
};


//Slightly modified development tables from Sunsetter

int pawn_development[] = {
	0, 0, 0, 0, 0, 0, 0, 0,
	4, 6, 2, -4, -6, 4, 8, 4,
	-1, -2, -3, 0, 0, -3, -2, -1,
	-2, -2, 0, 2, 2, 0, -2, -2,
	0, 0, 0, 3, 3, 0, 0, 0,
	0, 0, 0, 3, 3, 0, 0, 0,
	5, 6, 5, 5, 5, 5, 6, 5,
	0, 0, 0, 0, 0, 0, 0, 0
};

int knight_development[] = {
	-6, -5, -3, -3, -3, -3, -5, -6,
	0, 0, 0, 0, 0, 0, 0, 0,
	0, 0, 2, 0, 0, 2, 0, 0,
	0, 0, 2, 3, 3, 2, 0, 0,
	0, 0, 3, 4, 4, 3, 0, 0,
	0, 0, 3, 4, 4, 3, 0, 0,
	0, 0, 0, 0, 0, 0, 0, 0,
	0, 0, 0, 0, 0, 0, 0, 0
};

int queen_development[] = {
	-16,-14,-10, -4, -8,-10,-14,-16
	-12,-12,-10, -8, -8,-10,-12,-12,
	-20,-20,-20,-20,-20,-20,-20,-20,
	-20,-20,-20,-20,-20,-20,-20,-20,
	-20,-20,-20,-20,-20,-20,-20,-20,
	-20,-20,-20,-20,-20,-20,-20,-20,
	-10,-10,-10,-10,-10,-10,-10,-10,
	-10,-10,-10,-10,-10,-10,-10,-10
};

int king_development[] = {
	1, 2, 1, 0, 1, 0, 2, 1,
	-2, -6,-11,-11,-11,-11, -6, -2,
	-10,-18,-25,-25,-25,-25,-25,-10,
	-18,-25,-35,-35,-35,-35,-25,-18,
	-25,-35,-35,-35,-35,-35,-35,-25,
	-25,-35,-35,-35,-35,-35,-35,-25,
	-18,-25,-25,-25,-25,-25,-25,-18,
	-10,-18,-25,-25,-25,-25,-18,-10
};

int development[2][7][64];

int RookDstWhenCastle[64], RookSrcWhenCastle[64];
Bitboard ShouldBeEmptyWhenCastle[64];
Bitboard WhatToDoWithCastleRights[64];

Bitboard better_rook_moves[64][4096];
Bitboard better_bishop_moves[64][4096];

Position::Position(){}

int Position::PieceCnt(int side, int piece_type){
	return __builtin_popcountll(pieces[side][piece_type]);
}

void Position::Debug(){
	PrintChessBoard(piece_on_square);
}

void Position::PrintBitboard(Bitboard bitboard){
  for(int i=7; i>=0; i--){
    for(int j=0; j<8; j++){
      std::cout << (bool)(bitboard&(1ULL<<(8*i+j)));
    }
    std::cout << "\n";
  }

  std::cout << "\n";
}

void Position::PrintChessBoard(PieceType chessboard[]){
  static char pieces_symbols[] = {'K', 'Q', 'R', 'B', 'N', 'P', '.'};
  for(int i=7; i>=0; i--){
    for(int j=0; j<8; j++){
      std::cout << pieces_symbols[chessboard[8*i+j]];
    }
    std::cout << "\n";
  }

  std::cout << "\n";
}

void Position::ClearBoard(){
  for(int i=KING; i<=PAWN; i++){
    pieces[WHITE][i] = pieces[BLACK][i] = 0;
  }

  friendly[WHITE] = friendly[BLACK] = 0;

  for(int i=0; i<64; i++){
    piece_on_square[i] = EMPTY;
  }

	en_passant = 64;
	castle_rights[WHITE] = castle_rights[BLACK] = 0;
}

void Position::StartingPosition(){
  ClearBoard();
  pieces[WHITE][ROOK] |= (1ULL << A1);
  pieces[WHITE][KNIGHT] |= (1ULL<<B1);
  pieces[WHITE][BISHOP] |= (1ULL << C1);
  pieces[WHITE][QUEEN] |= (1ULL << D1);
  pieces[WHITE][KING] |= (1ULL << E1);
  pieces[WHITE][BISHOP] |= (1ULL << F1);
  pieces[WHITE][KNIGHT] |= (1ULL << G1);
  pieces[WHITE][ROOK] |= (1ULL << H1);

  piece_on_square[A1] = ROOK;
  piece_on_square[B1] = KNIGHT;
  piece_on_square[C1] = BISHOP;
  piece_on_square[D1] = QUEEN;
  piece_on_square[E1] = KING;
  piece_on_square[F1] = BISHOP;
  piece_on_square[G1] = KNIGHT;
  piece_on_square[H1] = ROOK;

  pieces[BLACK][ROOK] |= (1ULL << A8);
  pieces[BLACK][KNIGHT] |= (1ULL << B8);
  pieces[BLACK][BISHOP] |= (1ULL << C8);
  pieces[BLACK][QUEEN] |= (1ULL << D8);
  pieces[BLACK][KING] |= (1ULL << E8);
  pieces[BLACK][BISHOP] |= (1ULL << F8);
  pieces[BLACK][KNIGHT] |= (1ULL << G8);
  pieces[BLACK][ROOK] |= (1ULL << H8);

  piece_on_square[A8] = ROOK;
  piece_on_square[B8] = KNIGHT;
  piece_on_square[C8] = BISHOP;
  piece_on_square[D8] = QUEEN;
  piece_on_square[E8] = KING;
  piece_on_square[F8] = BISHOP;
  piece_on_square[G8] = KNIGHT;
  piece_on_square[H8] = ROOK;

  for(int i=A2; i<=H2; i++){
    pieces[WHITE][PAWN] |= (1ULL << i);
    piece_on_square[i] = PAWN;
  }

  for(int i=A7; i<=H7; i++){
    pieces[BLACK][PAWN] |= (1ULL << i);
    piece_on_square[i] = PAWN;
  }

  for(int i=WHITE; i<=BLACK; i++){
    for(int j=KING; j<=PAWN; j++){
      friendly[i] |= pieces[i][j];
    }
  }

	for(int i=WHITE; i<=BLACK; i++){
    for(int j=KING; j<=PAWN; j++){
      stash[i][j] = 0;
    }
  }

	crazyhouse = false;

	castle_rights[WHITE] = ((1ULL << C1) | (1ULL << G1));
	castle_rights[BLACK] = ((1ULL << C8) | (1ULL << G8));
	curr_side = WHITE;

	for(int p = QUEEN; p <= PAWN; p++)
		material[WHITE] += PieceCnt(WHITE, p)*(piece_val[p]);
	for(int p = QUEEN; p <= PAWN; p++)
		material[BLACK] += PieceCnt(BLACK, p)*(piece_val[p]);

	for(int p = KING; p <= PAWN; p++){
		curr_development[WHITE] += GetDevelopment(WHITE, static_cast<PieceType>(p));
	}
	for(int p = KING; p <= PAWN; p++){
		curr_development[BLACK] += GetDevelopment(BLACK, static_cast<PieceType>(p));
	}
}

Bitboard Position::ProcessRookSubset(int sq, Bitboard subset){
  Bitboard ret = 0;
  for(int i=sq+8; i<64; i+=8){
    ret += (1ULL << i);
    if((1ULL<<i)&subset) break;
  }
  for(int i=sq-8; i>=0; i-=8){
    ret += (1ULL << i);
    if((1ULL<<i)&subset) break;
  }
  for(int i=sq+1; i%8!=0; i++){
    ret += (1ULL << i);
    if((1ULL<<i)&subset) break;
  }
  for(int i=sq-1; i%8!=7 && i>=0; i--){
    ret += (1ULL << i);
    if((1ULL<<i)&subset) break;
  }
  return ret;
}

Bitboard Position::ProcessBishopSubset(int sq, Bitboard subset){
  Bitboard ret = 0;
  for(int i=sq+9; i<64 && i%8!=0; i+=9){
    ret += (1ULL << i);
    if((1ULL<<i)&subset) break;
  }
  for(int i=sq-9; i>=0 && i%8!=7; i-=9){
    ret += (1ULL << i);
    if((1ULL<<i)&subset) break;
  }
  for(int i=sq+7; i<64 && i%8!=7; i+=7){
    ret += (1ULL << i);
    if((1ULL<<i)&subset) break;
  }
  for(int i=sq-7; i>=0 && i%8!=0; i-=7){
    ret += (1ULL << i);
    if((1ULL<<i)&subset) break;
  }
  return ret;
}

void Position::PrecomputeRookMoves(){
  for(int sq = 0; sq < 64; sq++){
    Bitboard bitboard = RookOccupancyBitboard[sq];
    for(Bitboard subset = bitboard; subset > 0; subset = ((subset-1)&bitboard)){
      better_rook_moves[sq][(subset*RookMagic[sq])>>52] = ProcessRookSubset(sq, subset);
    }
    better_rook_moves[sq][0] = ProcessRookSubset(sq, 0);
  }
}

void Position::PrecomputeBishopMoves(){
  for(int sq = 0; sq < 64; sq++){
    Bitboard bitboard = BishopOccupancyBitboard[sq];
    for(Bitboard subset = bitboard; subset > 0; subset = ((subset-1)&bitboard)){
      better_bishop_moves[sq][(subset*BishopMagic[sq])>>52] = ProcessBishopSubset(sq, subset);
    }
    better_bishop_moves[sq][0] = ProcessBishopSubset(sq, 0);
  }
}

void Position::PrecomputeDevelopment(PieceType piece, int table[]){
	for(int side = WHITE; side <= BLACK; side++){
		for(int i=0; i<8; i++){
			for(int j=0; j<8; j++){
				development[side][piece][8*i+j] = (side == WHITE) ? table[8*i+j] : table[8*(7-i)+j];
			}
		}
	}
}

void Position::Preprocessing(){
  PrecomputeBishopMoves();
  PrecomputeRookMoves();

	PrecomputeDevelopment(PAWN, pawn_development);
	PrecomputeDevelopment(KNIGHT, knight_development);
	PrecomputeDevelopment(QUEEN, queen_development);
	PrecomputeDevelopment(KING, king_development);
}

int Position::GetDevelopment(int side, PieceType piece){
	int ret = 0;
	Bitboard bb = pieces[side][piece];
	for(; bb > 0; bb = bb&(bb-1)){
		ret += development[side][piece][__builtin_ctzll(bb)];
	}

	return ret;
}

void Position::Init(){
  Preprocessing();
	RookSrcWhenCastle[C1] = A1;
	RookSrcWhenCastle[G1] = H1;
	RookSrcWhenCastle[C8] = A8;
	RookSrcWhenCastle[G8] = H8;

	RookDstWhenCastle[C1] = D1;
	RookDstWhenCastle[G1] = F1;
	RookDstWhenCastle[C8] = D8;
	RookDstWhenCastle[G8] = F8;

	ShouldBeEmptyWhenCastle[C1] = (1ULL << B1) | (1ULL << C1) | (1ULL << D1);
	ShouldBeEmptyWhenCastle[G1] = (1ULL << F1) | (1ULL << G1);
	ShouldBeEmptyWhenCastle[C8] = (1ULL << B8) | (1ULL << C8) | (1ULL << D8);
	ShouldBeEmptyWhenCastle[G8] = (1ULL << F8) | (1ULL << G8);

	for(int i=0; i<64; i++) WhatToDoWithCastleRights[i] = ~(0ULL);
	WhatToDoWithCastleRights[A1] ^= (1ULL << C1);
	WhatToDoWithCastleRights[H1] ^= (1ULL << G1);
	WhatToDoWithCastleRights[A8] ^= (1ULL << C8);
	WhatToDoWithCastleRights[H8] ^= (1ULL << G8);

	for(int ind = 0; ind <= 1; ind++){
		for(int side = WHITE; side <= BLACK; side++){
			for(int i=KING; i<=PAWN; i++){
				for(int j=A1; j<=H8; j++){
					zobrist_piece[ind][side][i][j] = rand();
					if(ind==0) zobrist_piece[ind][side][i][j] %= HASH_SIZE;
				}
			}

			for(int j=A1; j<=H8; j++)
				zobrist_piece[ind][side][EMPTY][j] = 0;
		}
	}

	for(int ind = 0; ind <= 1; ind++){
		for(int side = WHITE; side <= BLACK; side++){
			for(int i=KING; i<=PAWN; i++){
				for(int j=0; j<=20; j++){
					zobrist_stash[ind][side][i][j] = rand();
					if(ind == 0) zobrist_stash[ind][side][i][j] %= HASH_SIZE;
				}
			}
		}
	}

	for(int i=0; i<65; i++){
		zobrist_en_passant[i] = rand()%HASH_SIZE;
	}

	for(int side = WHITE; side <= BLACK; side++){
		for(int i=0; i<256; i++){
			zobrist_castle[side][i] = rand()%HASH_SIZE;
		}
	}

	for(int side = WHITE; side <= BLACK; side++){
		zobrist_side[side] = rand()%HASH_SIZE;
	}
}

void Position::NewGame(){
	StartingPosition();
}

void Position::GeneratePawnMoves(int side, std::vector<MoveType> &move_list){
  Bitboard one_square = (side == WHITE) ? (pieces[side][PAWN] << 8) : (pieces[side][PAWN] >> 8);
  one_square &= ~(friendly[WHITE] | friendly[BLACK]);
	Bitboard promotions = one_square & (RowBitboard[0] | RowBitboard[7]);

  for(Bitboard occupancy = (one_square^promotions); occupancy > 0; occupancy = occupancy&(occupancy-1)){
    int i = __builtin_ctzll(occupancy);
    move_list.push_back(MoveType((side == WHITE) ? (i - 8) : (i + 8), i, PAWN));
  }

	for(Bitboard occupancy = promotions; occupancy > 0; occupancy = occupancy&(occupancy-1)){
		int i = __builtin_ctzll(occupancy);
		for(int p = QUEEN; p<=KNIGHT; p++)
			move_list.push_back(MoveType((side == WHITE) ? (i - 8) : (i + 8), i, (PieceType)p, PROMOTION));
	}

  one_square &= (side == WHITE) ? RowBitboard[2] : RowBitboard[5];
  one_square = (side == WHITE) ? (one_square << 8) : (one_square >> 8);
  one_square &= ~(friendly[WHITE] | friendly[BLACK]);
  for(Bitboard occupancy = one_square; occupancy > 0; occupancy = occupancy&(occupancy-1)){
    int i = __builtin_ctzll(occupancy);
    move_list.push_back(MoveType((side == WHITE) ? (i - 16) : (i + 16), i, PAWN));
  }
}

void Position::GenerateNonSliding(Bitboard occupancy, int side, Bitboard moves[], bool captures, std::vector<MoveType> &move_list){
  for(; occupancy > 0; occupancy = occupancy&(occupancy-1)){
    int i = __builtin_ctzll(occupancy);
    Bitboard bb = (moves[i]&(~friendly[side]));
    bb &= (captures ? (friendly[side^1]) : (~friendly[side^1]));

		if(piece_on_square[i] == PAWN){
			Bitboard promotions = bb & (RowBitboard[0] | RowBitboard[7]);
			bb ^= promotions;

			for(; promotions > 0; promotions = promotions&(promotions-1)){
				int j = __builtin_ctzll(promotions);
				for(int p = QUEEN; p<=KNIGHT; p++)
					move_list.push_back(MoveType(i, j, (PieceType)p, PROMOTION));
			}
		}

    for(; bb > 0; bb = bb&(bb-1)){
      int j = __builtin_ctzll(bb);
      move_list.push_back(MoveType(i, j, piece_on_square[i]));
    }
  }
}

void Position::GenerateSliding(
  Bitboard occupancy,
  int side,
  Bitboard mask[],
	U64 moves[][4096],
	U64 magic[],
  bool captures,
  std::vector<MoveType> &move_list){
  for(; occupancy > 0; occupancy = occupancy&(occupancy-1)){
    int i = __builtin_ctzll(occupancy);
		Bitboard tmp = mask[i]&(friendly[WHITE] | friendly[BLACK]);
    Bitboard bb = moves[i][(tmp*magic[i])>>52];
    bb &= captures ? (friendly[side^1]) : (~(friendly[WHITE] | friendly[BLACK]));
    for(; bb > 0; bb = bb&(bb-1)){
      int j = __builtin_ctzll(bb);
      move_list.push_back(MoveType(i, j, piece_on_square[i]));
    }
  }
}

void Position::GenerateBishopMoves(int side, bool captures, std::vector<MoveType> &move_list){
  GenerateSliding(pieces[side][BISHOP], side, BishopOccupancyBitboard, better_bishop_moves, BishopMagic, captures, move_list);
}

void Position::GenerateCastles(int side, std::vector<MoveType> &move_list){
	int king_pos = __builtin_ctzll(pieces[side][KING]);
	if(IsAttacked(king_pos, side^1)) return;
	Bitboard castles = castle_rights[side];
	for(; castles > 0; castles = castles&(castles-1)){
		int sq = __builtin_ctzll(castles);
		if((ShouldBeEmptyWhenCastle[sq]&(friendly[WHITE] | friendly[BLACK])) == 0 &&
		!IsAttacked(RookDstWhenCastle[sq], side^1))
			move_list.push_back(MoveType(king_pos, sq, KING));
	}
}

void Position::GenerateKingMoves(int side, bool captures, std::vector<MoveType> &move_list){
  GenerateNonSliding(pieces[side][KING], side, KingMovesBitboard, captures, move_list);
	if(!captures){
		GenerateCastles(side, move_list);
	}
}

void Position::GenerateKnightMoves(int side, bool captures, std::vector<MoveType> &move_list){
  GenerateNonSliding(pieces[side][KNIGHT], side, KnightMovesBitboard, captures, move_list);
}

void Position::GeneratePawnCaptures(int side, Bitboard en_passant_bitboard, std::vector<MoveType> &move_list){
	friendly[side^1] ^= en_passant_bitboard;
  GenerateNonSliding(pieces[side][PAWN], side,
    (side == WHITE ? WhitePawnsCapturesBitboard : BlackPawnsCapturesBitboard), true, move_list);
	friendly[side^1] ^= en_passant_bitboard;
}

void Position::GenerateRookMoves(int side, bool captures, std::vector<MoveType> &move_list){
  GenerateSliding(pieces[side][ROOK], side, RookOccupancyBitboard, better_rook_moves, RookMagic, captures, move_list);
}

void Position::GenerateQueenMoves(int side, bool captures, std::vector<MoveType> &move_list){
  GenerateSliding(pieces[side][QUEEN], side, RookOccupancyBitboard, better_rook_moves, RookMagic, captures, move_list);
  GenerateSliding(pieces[side][QUEEN], side, BishopOccupancyBitboard, better_bishop_moves, BishopMagic, captures, move_list);
}

void Position::GenerateDrops(int side, std::vector<MoveType> &move_list){
	Bitboard empty_squares = ~(friendly[WHITE] | friendly[BLACK]);
	Bitboard not_for_pawns = empty_squares&(RowBitboard[0] | RowBitboard[7]);
	empty_squares ^= not_for_pawns;
	for(; empty_squares > 0; empty_squares = empty_squares&(empty_squares-1)){
		int sq = __builtin_ctzll(empty_squares);
		for(int i=QUEEN; i<=PAWN; i++){
			if(stash[side][i] > 0){
				move_list.push_back(MoveType(sq, sq, (PieceType)i, DROP));
			}
		}
	}

	for(; not_for_pawns > 0; not_for_pawns = not_for_pawns&(not_for_pawns-1)){
		int sq = __builtin_ctzll(not_for_pawns);
		for(int i=QUEEN; i<=KNIGHT; i++){
			if(stash[side][i] > 0){
				move_list.push_back(MoveType(sq, sq, (PieceType)i, DROP));
			}
		}
	}
}

inline void Position::SetPieceOnASquare(int sq, int side, PieceType piece_type){
  assert(((friendly[WHITE] | friendly[BLACK]) & (1ULL << sq)) == 0);
  pieces[side][piece_type] |= (1ULL << sq);
  friendly[side] |= (1ULL << sq);
	piece_on_square[sq] = piece_type;
}

inline void Position::EraseFromBoard(int side, int sq){
  pieces[side][piece_on_square[sq]] &= ~(1ULL << sq);
  friendly[side] &= ~(1ULL << sq);
	piece_on_square[sq] = EMPTY;
}

std::vector<MoveType> Position::GeneratePseudoLegalCaptures(){
	std::vector<MoveType> ret;
	GeneratePseudoLegalCaptures(curr_side, ret);
	return ret;
}

std::vector<MoveType> Position::GeneratePseudoLegalMoves(){
	return Position::GeneratePseudoLegalMoves(curr_side);
}

std::vector<MoveType> Position::GeneratePseudoLegalMoves(int side){
  std::vector<MoveType> ret;
  GeneratePseudoLegalCaptures(side, ret);
  GeneratePseudoLegalNonCaptures(side, ret);
  return ret;
}

void Position::GeneratePseudoLegalCaptures(int side, std::vector<MoveType> &move_list){
  GeneratePawnCaptures(side, (en_passant < 64 ? (1ULL<<en_passant) : 0), move_list);
  GenerateBishopMoves(side, true, move_list);
  GenerateKnightMoves(side, true, move_list);
  GenerateRookMoves(side, true, move_list);
  GenerateQueenMoves(side, true, move_list);
  GenerateKingMoves(side, true, move_list);
}

void Position::GeneratePseudoLegalNonCaptures(int side, std::vector<MoveType> &move_list){
  GeneratePawnMoves(side, move_list);
  GenerateBishopMoves(side, false, move_list);
  GenerateKnightMoves(side, false, move_list);
  GenerateRookMoves(side, false, move_list);
  GenerateQueenMoves(side, false, move_list);
  GenerateKingMoves(side, false, move_list);

	if(crazyhouse){
		GenerateDrops(side, move_list);
	}
}

void Position::MakeMove(int side, MoveType move){
	if(move.dst == en_passant && piece_on_square[move.src] == PAWN){
		int location = en_passant + (side == WHITE ? -8 : +8);
		EraseFromBoard(side^1, location);
		material[side^1] -= piece_val[PAWN];
		curr_development[side^1] -= development[side^1][PAWN][location];
		if(crazyhouse){
			stash[side][PAWN]++;
			material[side] += piece_val[PAWN];
		}
	}

	if(piece_on_square[move.src] == PAWN && std::abs(move.dst-move.src) == 16){
		en_passant = move.src + (move.dst - move.src)/2;
	}
	else{
		en_passant = 64;
	}

	if(piece_on_square[move.src] == KING){
		if(std::abs(move.dst-move.src) == 2){
			SimplyPerformMove(side, MoveType(RookSrcWhenCastle[move.dst], RookDstWhenCastle[move.dst], ROOK));
		}
		castle_rights[side] = 0;
	}

	if(piece_on_square[move.src] == ROOK){
		castle_rights[side] &= WhatToDoWithCastleRights[move.src];
	}

	if(piece_on_square[move.dst] == ROOK){
		castle_rights[side^1] &= WhatToDoWithCastleRights[move.dst];
	}

	if(move.kind_of_a_move == DROP){
		stash[side][move.piece_dst]--;
		material[side] -= piece_val[move.piece_dst]; //will be adjusted in SimplyPerformMove
	}

	SimplyPerformMove(side, move);
}

void Position::MakeMove(MoveType move){
	MakeMove(curr_side, move);
	curr_side ^= 1;
}

void Position::SimplyPerformMove(int side, MoveType move){
	if(move.piece_dst == UNSPECIFIED){
		move.piece_dst = piece_on_square[move.src];
	}

	//Handle material balance change when (un)promoting or dropping.
	material[side] += piece_val[move.piece_dst] - piece_val[piece_on_square[move.src]];

	//Handle development change
	curr_development[side] += development[side][move.piece_dst][move.dst] - development[side][piece_on_square[move.src]][move.src];
	curr_development[side^1] -= development[side^1][piece_on_square[move.dst]][move.dst];

	if(piece_on_square[move.src] != EMPTY){
		pieces[side][piece_on_square[move.src]] ^= (1ULL << move.src);
	}
	if(move.piece_dst != EMPTY){
  	pieces[side][move.piece_dst] ^= (1ULL << move.dst);
	}
  friendly[side] ^= (1ULL << move.src) | (1ULL << move.dst);

  if(piece_on_square[move.dst] != EMPTY){
    pieces[side^1][piece_on_square[move.dst]] ^= (1ULL << move.dst);
    friendly[side^1] ^= (1ULL << move.dst);
		material[side^1] -= piece_val[piece_on_square[move.dst]];

		if(crazyhouse){
			stash[side][piece_on_square[move.dst]]++;
			material[side] += piece_val[piece_on_square[move.dst]];
		}
  }

  piece_on_square[move.src] = EMPTY;
	piece_on_square[move.dst] = move.piece_dst;
}

void Position::UnmakeMove(MoveType move, MoveMetaDataType data){
	UnmakeMove(curr_side^1, move, data);
	curr_side ^= 1;
}

void Position::UnmakeMove(int side, MoveType move, MoveMetaDataType data){
	if(move.dst == data.en_passant && piece_on_square[move.dst] == PAWN && move.kind_of_a_move != DROP){
		en_passant = data.en_passant + (side == WHITE ? -8 : +8);
		piece_on_square[en_passant] = PAWN;
		pieces[side^1][PAWN] ^= (1ULL << en_passant);
		friendly[side^1] ^= (1ULL << en_passant);
		material[side^1] += piece_val[PAWN];
		curr_development[side^1] += development[side^1][PAWN][en_passant];
		if(crazyhouse){
			assert(stash[side][PAWN] > 0);
			stash[side][PAWN]--;
			material[side] -= piece_val[PAWN];
		}
	}
	en_passant = data.en_passant;

	if(piece_on_square[move.dst] == KING && std::abs(move.dst-move.src) == 2){
		SimplyPerformMove(side, MoveType(RookDstWhenCastle[move.dst], RookSrcWhenCastle[move.dst], ROOK));
	}

	castle_rights[WHITE] = data.castle_rights[WHITE];
	castle_rights[BLACK] = data.castle_rights[BLACK];

	if(move.kind_of_a_move == DROP){
		stash[side][move.piece_dst]++;
		friendly[side] ^= (1ULL << move.dst);
		pieces[side][move.piece_dst] ^= (1ULL << move.dst);
		piece_on_square[move.dst] = EMPTY;
		curr_development[side] -= development[side][move.piece_dst][move.dst];
	}
	else{
		SimplyPerformMove(side, MoveType(move.dst, move.src, data.what_piece_was_moving));

		if(data.what_was_there != EMPTY){
	    piece_on_square[move.dst] = data.what_was_there;
	    pieces[side^1][data.what_was_there] |= (1ULL << move.dst);
	    friendly[side^1] |= (1ULL << move.dst);
			material[side^1] += piece_val[data.what_was_there];
			curr_development[side^1] += development[side^1][data.what_was_there][move.dst];

			if(crazyhouse){
				stash[side][data.what_was_there]--;
				material[side] -= piece_val[data.what_was_there];
			}
	  }
	}
}

bool Position::IsLegal(){
	return !IsAttacked(__builtin_ctzll(pieces[curr_side^1][KING]), curr_side);
}

U64 Position::Perft(int side, int depth, int &captures_no){
    U64 nodes = 0;

    if(depth == 0){
			return 1;
		}

    std::vector<MoveType> move_list;
    move_list = GeneratePseudoLegalMoves(side);
    for(const auto &move : move_list){
      MoveMetaDataType data = MoveMetaDataType(piece_on_square[move.dst],
				en_passant, castle_rights[WHITE],
				castle_rights[BLACK], piece_on_square[move.src]);
      MakeMove(side, move);//MakeMove(move);

			if(!IsAttacked(__builtin_ctzll(pieces[side][KING]), side^1)){
				if(depth == 1 && (data.what_was_there != EMPTY ||
					(move.dst == data.en_passant && data.what_piece_was_moving == PAWN)))
					captures_no++;
      	nodes += Perft(side^1, depth - 1, captures_no);
			}
      UnmakeMove(side, move, data);//UnmakeMove(move, data);
    }

    return nodes;
}

bool Position::IsAttacked(int sq, int by_side){
	if(KingMovesBitboard[sq]&pieces[by_side][KING]) return true;
	if(KnightMovesBitboard[sq]&pieces[by_side][KNIGHT]) return true;
	if((by_side == WHITE ?
		BlackPawnsCapturesBitboard[sq] :
		WhitePawnsCapturesBitboard[sq])&pieces[by_side][PAWN]) return true;
	Bitboard tmp = BishopOccupancyBitboard[sq] & (friendly[WHITE] | friendly[BLACK]);
	if(better_bishop_moves[sq][(tmp * BishopMagic[sq]) >> 52]&
		(pieces[by_side][BISHOP] | pieces[by_side][QUEEN])) return true;
	tmp = RookOccupancyBitboard[sq] & (friendly[WHITE] | friendly[BLACK]);
	if(better_rook_moves[sq][(tmp * RookMagic[sq]) >> 52]&
		(pieces[by_side][ROOK] | pieces[by_side][QUEEN])) return true;

	return false;
}

bool Position::IsMyKingAttaked(){
	return IsAttacked(__builtin_ctzll(pieces[curr_side][KING]), curr_side^1);
}

void Position::Kiwipete(){
	ClearBoard();
	SetPieceOnASquare(A1, WHITE, ROOK);
	SetPieceOnASquare(E1, WHITE, KING);
	SetPieceOnASquare(H1, WHITE, ROOK);
	SetPieceOnASquare(A2, WHITE, PAWN);
	SetPieceOnASquare(B2, WHITE, PAWN);
	SetPieceOnASquare(C2, WHITE, PAWN);
	SetPieceOnASquare(D2, WHITE, BISHOP);
	SetPieceOnASquare(E2, WHITE, BISHOP);
	SetPieceOnASquare(F2, WHITE, PAWN);
	SetPieceOnASquare(G2, WHITE, PAWN);
	SetPieceOnASquare(H2, WHITE, PAWN);
	SetPieceOnASquare(C3, WHITE, KNIGHT);
	SetPieceOnASquare(F3, WHITE, QUEEN);
	SetPieceOnASquare(E4, WHITE, PAWN);
	SetPieceOnASquare(D5, WHITE, PAWN);
	SetPieceOnASquare(E5, WHITE, KNIGHT);

	SetPieceOnASquare(A8, BLACK, ROOK);
	SetPieceOnASquare(E8, BLACK, KING);
	SetPieceOnASquare(H8, BLACK, ROOK);
	SetPieceOnASquare(A7, BLACK, PAWN);
	SetPieceOnASquare(C7, BLACK, PAWN);
	SetPieceOnASquare(D7, BLACK, PAWN);
	SetPieceOnASquare(E7, BLACK, QUEEN);
	SetPieceOnASquare(F7, BLACK, PAWN);
	SetPieceOnASquare(G7, BLACK, BISHOP);
	SetPieceOnASquare(A6, BLACK, BISHOP);
	SetPieceOnASquare(B6, BLACK, KNIGHT);
	SetPieceOnASquare(E6, BLACK, PAWN);
	SetPieceOnASquare(F6, BLACK, KNIGHT);
	SetPieceOnASquare(G6, BLACK, PAWN);
	SetPieceOnASquare(B4, BLACK, PAWN);
	SetPieceOnASquare(H3, BLACK, PAWN);

	PrintChessBoard(piece_on_square);

	castle_rights[WHITE] = ((1ULL << C1) | (1ULL << G1));
	castle_rights[BLACK] = ((1ULL << C8) | (1ULL << G8));

	curr_side = WHITE;
}

//https://chessprogramming.wikispaces.com/Perft+Results#Position3
void Position::Position3(){
	ClearBoard();
	SetPieceOnASquare(E2, WHITE, PAWN);
	SetPieceOnASquare(G2, WHITE, PAWN);
	SetPieceOnASquare(B4, WHITE, ROOK);
	SetPieceOnASquare(A5, WHITE, KING);
	SetPieceOnASquare(B5, WHITE, PAWN);

	SetPieceOnASquare(F4, BLACK, PAWN);
	SetPieceOnASquare(H4, BLACK, KING);
	SetPieceOnASquare(H5, BLACK, ROOK);
	SetPieceOnASquare(D6, BLACK, PAWN);
	SetPieceOnASquare(C7, BLACK, PAWN);

	PrintChessBoard(piece_on_square);

	curr_side = WHITE;
}

MoveMetaDataType Position::StandardMetaData(MoveType move){
	return MoveMetaDataType(piece_on_square[move.dst],
		en_passant, castle_rights[WHITE],
		castle_rights[BLACK], piece_on_square[move.src]);
}

unsigned int Position::NewHash(unsigned int old_hash, int side, MoveType move, int ind){
	unsigned int ret = old_hash;
	if(move.dst == en_passant && piece_on_square[move.src] == PAWN){
		int target = en_passant + (side == WHITE ? -8 : +8);
		ret ^= zobrist_piece[ind][side^1][PAWN][target];
	}

	if(piece_on_square[move.src] == KING && std::abs(move.dst-move.src) == 2){
		ret ^= zobrist_piece[ind][side][ROOK][RookSrcWhenCastle[move.dst]];
		ret ^= zobrist_piece[ind][side][ROOK][RookDstWhenCastle[move.dst]];
	}

	ret ^= zobrist_piece[ind][side][piece_on_square[move.src]][move.src];
	ret ^= zobrist_piece[ind][side][move.piece_dst][move.dst];

	if(piece_on_square[move.dst] != EMPTY){
		PieceType cap = piece_on_square[move.dst];
		ret ^= zobrist_piece[ind][side^1][cap][move.dst];

		if(crazyhouse){
			ret ^= zobrist_stash[ind][side][cap][stash[side][cap]];
			ret ^= zobrist_stash[ind][side][cap][stash[side][cap]+1];
		}
	}

	if(move.kind_of_a_move == DROP){
		ret ^= zobrist_stash[ind][side][move.piece_dst][stash[side][move.piece_dst]];
		ret ^= zobrist_stash[ind][side][move.piece_dst][stash[side][move.piece_dst]-1];
	}

	return ret;
}

unsigned int Position::AuxHash(){
	return (zobrist_en_passant[en_passant]^zobrist_castle[WHITE][castle_rights[WHITE]]^zobrist_castle[BLACK][castle_rights[BLACK]>>56]^zobrist_side[curr_side]);
}

unsigned int Position::CurrentPositionHash(int ind){
	unsigned int ret = 0;
	for(int side = WHITE; side <= BLACK; side++){
		for(int i=KING; i<=PAWN; i++){
			for(int j=A1; j<=H8; j++){
				if((1ULL << j) & pieces[side][i]){
					ret ^= zobrist_piece[ind][side][i][j];
				}
			}
		}
	}

	if(crazyhouse){
		for(int side = WHITE; side <= BLACK; side++){
			for(int i=KING; i<=PAWN; i++){
				ret ^= zobrist_stash[ind][side][i][stash[side][i]];
			}
		}
	}

	return ret;
}

//------------------------------------------------------------------------------
//Tests

void Position::TestPiecesInStartingPosition(int side){
  std::cout << "TestPiecesInStartingPosition\nside: " << side << "\n";
  StartingPosition();
  assert(pieces[side][PAWN] == (side == WHITE ? (255ULL << 8) : (255ULL << 48)));
  Bitboard tmp = 0;
  for(int i=KING; i <= KNIGHT; i++) tmp |= pieces[side][i];
  assert(tmp == (side == WHITE ? 255ULL : (255ULL << 56)));
}

void Position::TestKnightMovesInStartingPosition(int side){
  std::cout << "TestKnightMovesInStartingPosition\nside: " << side << "\n";
  StartingPosition();
  std::vector<MoveType> move_list;
  GenerateKnightMoves(side, false, move_list);
  for(auto move : move_list){
    std::cout << move.src << " " << move.dst <<"\n";
  }
  assert(move_list.size() == 4);
}

void Position::TestPawnMovesInStartingPosition(int side){
  std::cout << "TestPawnMovesInStartingPosition\nside: " << side << "\n";
  StartingPosition();
  std::vector<MoveType> move_list;
  GeneratePawnMoves(side, move_list);
  for(auto move : move_list){
    std::cout << move.src << " " << move.dst <<"\n";
  }
  assert(move_list.size() == 16);
}

void Position::TestFriendlyInStartingPosition(int side){
  std::cout << "TestFriendlyInStartingPosition\nside: " << side << "\n";
  StartingPosition();
  assert(friendly[side] == (side == WHITE ? 65535ULL : 65535ULL<<48));
}

void Position::TestBishopMovesWithOneOtherPiece(int side, int bishop_pos, int other_side, int other_pos, bool captures, int expected_moves_cnt){
  std::cout << "TestBishopMovesWithOneOtherPiece\n";
  std::cout << "side: " << side << " bishop_pos: " << bishop_pos << "\n";
  std::cout << "other_side: " << other_side << " other_pos: " << other_pos << "\n";
  std::cout << "captures: " << captures << " expected_moves_cnt: " << expected_moves_cnt << "\n";

  ClearBoard();
  SetPieceOnASquare(bishop_pos, side, BISHOP);
  SetPieceOnASquare(other_pos, other_side, KNIGHT);

  std::vector<MoveType> move_list;
  GenerateBishopMoves(side, captures, move_list);
  int moves_cnt = move_list.size();
  assert(moves_cnt == expected_moves_cnt);
}

void Position::TestRookMovesWithOneOtherPiece(int side, int rook_pos, int other_side, int other_pos, bool captures, int expected_moves_cnt){
  std::cout << "TestRookMovesWithOneOtherPiece\n";
  std::cout << "side: " << side << " rook_pos: " << rook_pos << "\n";
  std::cout << "other_side: " << other_side << " other_pos: " << other_pos << "\n";
  std::cout << "captures: " << captures << " expected_moves_cnt: " << expected_moves_cnt << "\n";

  ClearBoard();
  SetPieceOnASquare(rook_pos, side, ROOK);
  SetPieceOnASquare(other_pos, other_side, KNIGHT);

  std::vector<MoveType> move_list;
  GenerateRookMoves(side, captures, move_list);
  int moves_cnt = move_list.size();
  assert(moves_cnt == expected_moves_cnt);
}

void Position::TestKingMovesWithOneOtherPiece(int side, int king_pos, int other_side, int other_pos, bool captures, int expected_moves_cnt){
  std::cout << "TestKingMovesWithOneOtherPiece\n";
  std::cout << "side: " << side << " king_pos: " << king_pos << "\n";
  std::cout << "other_side: " << other_side << " other_pos: " << other_pos << "\n";
  std::cout << "captures: " << captures << " expected_moves_cnt: " << expected_moves_cnt << "\n";

  ClearBoard();
  SetPieceOnASquare(king_pos, side, KING);
  SetPieceOnASquare(other_pos, other_side, KNIGHT);

  std::vector<MoveType> move_list;
  GenerateKingMoves(side, captures, move_list);
  int moves_cnt = move_list.size();
  assert(moves_cnt == expected_moves_cnt);
}

void Position::TestKnightMovesWithOneOtherPiece(int side, int knight_pos, int other_side, int other_pos, bool captures, int expected_moves_cnt){
  std::cout << "TestKnightMovesWithOneOtherPiece\n";
  std::cout << "side: " << side << " knight_pos: " << knight_pos << "\n";
  std::cout << "other_side: " << other_side << " other_pos: " << other_pos << "\n";
  std::cout << "captures: " << captures << " expected_moves_cnt: " << expected_moves_cnt << "\n";

  ClearBoard();
  SetPieceOnASquare(knight_pos, side, KNIGHT);
  SetPieceOnASquare(other_pos, other_side, BISHOP);

  std::vector<MoveType> move_list;
  GenerateKnightMoves(side, captures, move_list);
  int moves_cnt = move_list.size();
  assert(moves_cnt == expected_moves_cnt);
}

void Position::TestPawnMovesWithOneOtherPiece(int side, int pawn_pos, int other_side, int other_pos, bool captures, int expected_moves_cnt){
  std::cout << "TestPawnMovesWithOneOtherPiece\n";
  std::cout << "side: " << side << " pawn_pos: " << pawn_pos << "\n";
  std::cout << "other_side: " << other_side << " other_pos: " << other_pos << "\n";
  std::cout << "captures: " << captures << " expected_moves_cnt: " << expected_moves_cnt << "\n";

  ClearBoard();
  SetPieceOnASquare(pawn_pos, side, PAWN);
  SetPieceOnASquare(other_pos, other_side, BISHOP);

  std::vector<MoveType> move_list;
  if(captures)
    GeneratePawnCaptures(side, (en_passant < 64 ? (1ULL<<en_passant) : 0), move_list);
  else
    GeneratePawnMoves(side, move_list);
  int moves_cnt = move_list.size();
  assert(moves_cnt == expected_moves_cnt);
}

void Position::TestStartingPositionMovesGeneration(int side){
  std::cout << "TestStartingPositionMovesGeneration\n";
  std::cout << "side: " << side << "\n";

  StartingPosition();
  int moves_cnt = GeneratePseudoLegalMoves(side).size();
  assert(moves_cnt == 20);
}

void Position::TestPerftFromStartingPosition(int depth, int expected_moves_cnt){
  std::cout << "TestPerftFromStartingPosition\n";
  std::cout << "depth: " << depth << " expected_moves_cnt: " << expected_moves_cnt << "\n";

  StartingPosition();
	int captures_cnt = 0;
  U64 res = Perft(WHITE, depth, captures_cnt);
  std::cout << res << "\n";
	std::cout << captures_cnt << "\n";
  assert(res == expected_moves_cnt);
}

void Position::TestPerftFromKiwipete(int depth, int expected_moves_cnt){
  std::cout << "TestPerftFromKiwipete\n";
  std::cout << "depth: " << depth << " expected_moves_cnt: " << expected_moves_cnt << "\n";

  Kiwipete();
	int captures_cnt = 0;
  U64 res = Perft(WHITE, depth, captures_cnt);
  std::cout << res << "\n";
	std::cout << captures_cnt << "\n";
  assert(res == expected_moves_cnt);
}

void Position::TestPromotionsSimply(){
  std::cout << "SimplePromotionsTest\n";

	ClearBoard();
	SetPieceOnASquare(E7, WHITE, PAWN);
	SetPieceOnASquare(A1, WHITE, KING);
	SetPieceOnASquare(H1, BLACK, KING);
	int captures_cnt = 0;
  U64 res = Perft(WHITE, 1, captures_cnt);
  std::cout << res << "\n";
	std::cout << captures_cnt << "\n";
  assert(res == 7);
}

void Position::TestPromotionsSimply2(){
  std::cout << "SimplePromotionsTest\n";

	ClearBoard();
	SetPieceOnASquare(E7, WHITE, PAWN);
	SetPieceOnASquare(A1, WHITE, KING);
	SetPieceOnASquare(H1, BLACK, KING);
	SetPieceOnASquare(D8, BLACK, KNIGHT);
	SetPieceOnASquare(F8, BLACK, BISHOP);
	int captures_cnt = 0;
  U64 res = Perft(WHITE, 1, captures_cnt);
  std::cout << res << "\n";
	std::cout << captures_cnt << "\n";
  assert(res == 15);
}

void Position::TestPerftFromPosition3(int depth, int expected_moves_cnt){
  std::cout << "TestPerftFromPosition3\n";
  std::cout << "depth: " << depth << " expected_moves_cnt: " << expected_moves_cnt << "\n";

  Position3();
	int captures_cnt = 0;
  U64 res = Perft(WHITE, depth, captures_cnt);
  std::cout << res << "\n";
	std::cout << captures_cnt << "\n";
  assert(res == expected_moves_cnt);
}

void Position::TestPerftFromStartingPositionInCrazyhouse(int depth, int expected_moves_cnt){
  std::cout << "TestPerftFromStartingPositionInCrazyhouse\n";
  std::cout << "depth: " << depth << " expected_moves_cnt: " << expected_moves_cnt << "\n";

  StartingPosition();
	crazyhouse = true;
	int captures_cnt = 0;
  U64 res = Perft(WHITE, depth, captures_cnt);
  std::cout << res << "\n";
	std::cout << captures_cnt << "\n";
  assert(res == expected_moves_cnt);
}
