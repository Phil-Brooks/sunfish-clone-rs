// this is intended to be an exact replica of sunfish from https://github.com/thomasahle/sunfish
// rust specifics will only be used where absolutely needed.
use std::cmp::max;
use std::collections::HashMap;
use std::collections::HashSet;

const VERSION: &str = "sunfish 2023";

fn main() {
    //##############################################################################
    // Piece-Square tables. Tune these to change sunfish's behaviour
    //###############################################################################

    // With xz compression this whole section takes 652 bytes.
    // That's pretty good given we have 64*6 = 384 values.
    // Though probably we could do better...
    // For one thing, they could easily all fit into int8.
    fn piece(p: char) -> i32 {
        match p {
            'P' => 100,
            'N' => 280,
            'B' => 320,
            'R' => 479,
            'Q' => 929,
            'K' => 60000,
            _ => 0,
        }
    }
    fn pst(p: char) -> [i32; 120] {
        match p {
            'P' => [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 100, 100, 100, 100,
                100, 100, 100, 100, 0, 0, 178, 183, 186, 173, 202, 182, 185, 190, 0, 0, 107, 129,
                121, 144, 140, 131, 144, 107, 0, 0, 83, 116, 98, 115, 114, 100, 115, 87, 0, 0, 74,
                103, 110, 109, 106, 101, 100, 77, 0, 0, 78, 109, 105, 89, 90, 98, 103, 81, 0, 0,
                69, 108, 93, 63, 64, 86, 103, 69, 0, 0, 100, 100, 100, 100, 100, 100, 100, 100, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            'N' => [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 214, 227, 205, 205,
                270, 225, 222, 210, 0, 0, 277, 274, 380, 244, 284, 342, 276, 266, 0, 0, 290, 347,
                281, 354, 353, 307, 342, 278, 0, 0, 304, 304, 325, 317, 313, 321, 305, 297, 0, 0,
                279, 285, 311, 301, 302, 315, 282, 280, 0, 0, 262, 290, 293, 302, 298, 295, 291,
                266, 0, 0, 257, 265, 282, 280, 282, 280, 257, 260, 0, 0, 206, 257, 254, 256, 261,
                245, 258, 211, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            'B' => [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 261, 242, 238, 244,
                297, 213, 283, 270, 0, 0, 309, 340, 355, 278, 281, 351, 322, 298, 0, 0, 311, 359,
                288, 361, 372, 310, 348, 306, 0, 0, 345, 337, 340, 354, 346, 345, 335, 330, 0, 0,
                333, 330, 337, 343, 337, 336, 320, 327, 0, 0, 334, 345, 344, 335, 328, 345, 340,
                335, 0, 0, 339, 340, 331, 326, 327, 326, 340, 336, 0, 0, 313, 322, 305, 308, 306,
                305, 310, 310, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            'R' => [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 514, 508, 512, 483,
                516, 512, 535, 529, 0, 0, 534, 508, 535, 546, 534, 541, 513, 539, 0, 0, 498, 514,
                507, 512, 524, 506, 504, 494, 0, 0, 479, 484, 495, 492, 497, 475, 470, 473, 0, 0,
                451, 444, 463, 458, 466, 450, 433, 449, 0, 0, 437, 451, 437, 454, 454, 444, 453,
                433, 0, 0, 426, 441, 448, 453, 450, 436, 435, 426, 0, 0, 449, 455, 461, 484, 477,
                461, 448, 447, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            'Q' => [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 935, 930, 921, 825,
                998, 953, 1017, 955, 0, 0, 943, 961, 989, 919, 949, 1005, 986, 953, 0, 0, 927, 972,
                961, 989, 1001, 992, 972, 931, 0, 0, 930, 913, 951, 946, 954, 949, 916, 923, 0, 0,
                915, 914, 927, 924, 928, 919, 909, 907, 0, 0, 899, 923, 916, 918, 913, 918, 913,
                902, 0, 0, 893, 911, 929, 910, 914, 914, 908, 891, 0, 0, 890, 899, 898, 916, 898,
                893, 895, 887, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            'K' => [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 60004, 60054, 60047,
                59901, 59901, 60060, 60083, 59938, 0, 0, 59968, 60010, 60055, 60056, 60056, 60055,
                60010, 60003, 0, 0, 59938, 60012, 59943, 60044, 59933, 60028, 60037, 59969, 0, 0,
                59945, 60050, 60011, 59996, 59981, 60013, 60000, 59951, 0, 0, 59945, 59957, 59948,
                59972, 59949, 59953, 59992, 59950, 0, 0, 59953, 59958, 59957, 59921, 59936, 59968,
                59971, 59968, 0, 0, 59996, 60003, 59986, 59950, 59943, 59982, 60013, 60004, 0, 0,
                60017, 60030, 59997, 59986, 60006, 59999, 60040, 60018, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            _ => [0; 120],
        }
    }
    //###############################################################################
    // Global constants
    //###############################################################################
    // Our board is represented as a 120 character string. The padding allows for
    // fast detection of moves that don't stay within the board.
    let (a1, h1, a8, h8) = (91, 98, 21, 28);
    let initial = "         \n".to_owned() + //   0 -  9
        "         \n" + //  10 - 19
        " rnbqkbnr\n" + //  20 - 29
        " pppppppp\n" + //  30 - 39
        " ........\n" + //  40 - 49
        " ........\n" + //  50 - 59
        " ........\n" + //  60 - 69
        " ........\n" + //  70 - 79
        " PPPPPPPP\n" + //  80 - 89
        " RNBQKBNR\n" + //  90 - 99
        "         \n" + // 100 -109
        "         \n"; // 110 -119
    // Lists of possible moves for each piece type.
    fn directions(p: char) -> Vec<i32> {
        let (n, e, s, w) = (-10, 1, 10, -1);
        match p {
            'P' => vec![n, n + n, n + w, n + e],
            'N' => vec![
                n + n + e,
                e + n + e,
                e + s + e,
                s + s + e,
                s + s + w,
                w + s + w,
                w + n + w,
                n + n + w,
            ],
            'B' => vec![n + e, n + w, s + e, s + w],
            'R' => vec![n, e, s, w],
            'Q' => vec![n, e, s, w, n + e, s + e, s + w, n + w],
            'K' => vec![n, e, s, w, n + e, s + e, s + w, n + w],
            _ => vec![],
        }
    }
    // Mate value must be greater than 8*queen + 2*(rook+knight+bishop)
    // King value is set to twice this value such that if the opponent is
    // 8 queens up, but we got the king, we still exceed MATE_VALUE.
    // When a MATE is detected, we'll set the score to MATE_UPPER - plies to get there
    // E.g. Mate in 3 will be MATE_UPPER - 6
    let mate_lower: i32 = piece('K') - 10 * piece('Q');
    let mate_upper: i32 = piece('K') + 10 * piece('Q');
    // Constants for tuning search
    let qs = 40;
    let qs_a = 140;
    let eval_roughness = 15;
    let qs_lim = (0, 300);
    let qs_a_lim = (0, 300);
    let eval_roughness_lim = (0, 50);
    //###############################################################################
    // Chess logic
    //###############################################################################
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    struct Move {
        i: usize,
        j: usize,
        prom: char,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    struct Position {
        //A state of a chess game
        board: [char; 120], // a 120 char representation of the board
        score: i32,         // the board evaluation
        wc: (bool, bool),   // the castling rights, [west/queen side, east/king side]
        bc: (bool, bool),   // the opponent castling rights, [west/king side, east/queen side]
        ep: usize,          // the en passant square
        kp: usize,          // the king passant square
    }
    impl Position {
        fn gen_moves(&self) -> Vec<Move> {
            let (a1, h1, a8, h8) = (91, 98, 21, 28);
            let (n, e, s, w) = (-10, 1, 10, -1);

            let mut moves = Vec::new();
            for i in 0..120 {
                let p = self.board[i];
                if !p.is_ascii_uppercase() {
                    continue; // skip empty squares and opponent pieces
                }
                let directions = directions(p);
                for &d in &directions {
                    let j = i + d as usize;
                    let q = self.board[j];
                    // Stay inside the board, and off friendly pieces
                    if q.is_ascii_uppercase() || q.is_whitespace() {
                        break; // skip moves that capture own pieces
                    }
                    if p == 'P' {
                        if [n, n + n].contains(&d) && q != '.' {
                            break;
                        }
                        if d == (n + n)
                            && (i < (a1 as i32 + n) as usize
                                || self.board[(i as i32 + n) as usize] != '.')
                        {
                            break;
                        }
                        if [n + w, n + e].contains(&d)
                            && q == '.'
                            && [self.ep, self.kp, self.kp - 1, self.kp + 1].contains(&j)
                        {
                            break;
                        }
                        // If we move to the last row, we can be anything
                        if a8 <= j && j <= h8 {
                            for prom in "NBRQ".chars() {
                                moves.push(Move {
                                    i: i,
                                    j: j,
                                    prom: prom,
                                });
                            }
                            break;
                        }
                    }
                    // Move it
                    moves.push(Move {
                        i: i,
                        j: j,
                        prom: ' ',
                    });
                    // Stop crawlers from sliding, and sliding after captures
                    if "PNK".contains(p) || q.is_lowercase() {
                        break;
                    }
                    // Castling, by sliding the rook next to the king
                    if i == a1 && self.board[j + e as usize] == 'K' && self.wc.0 {
                        moves.push(Move {
                            i: j + e as usize,
                            j: j + w as usize,
                            prom: ' ',
                        })
                    }
                    if i == h1 && self.board[j + w as usize] == 'K' && self.wc.1 {
                        moves.push(Move {
                            i: j + w as usize,
                            j: j + e as usize,
                            prom: ' ',
                        })
                    }
                }
            }
            moves
        }
        fn rotate(&self, nullmove: bool) -> Position {
            Position {
                board: Self::swap_case(self.board),
                score: -self.score,
                wc: self.wc,
                bc: self.bc,
                ep: if self.ep == 0 || nullmove {
                    0
                } else {
                    119 - self.ep
                },
                kp: if self.kp == 0 || nullmove {
                    0
                } else {
                    119 - self.kp
                },
            }
        }
        // Helper function to swap the case of each character in the board array
        fn swap_case(board: [char; 120]) -> [char; 120] {
            let mut new_board = [' '; 120];
            for (i, &c) in board.iter().enumerate() {
                new_board[i] = if c.is_ascii_lowercase() {
                    c.to_ascii_uppercase()
                } else if c.is_ascii_uppercase() {
                    c.to_ascii_lowercase()
                } else {
                    c
                };
            }
            new_board
        }
        fn domove(&self, mov: Move) -> Position {
            let (a1, h1, a8, h8) = (91, 98, 21, 28);
            let (n, e, s, w) = (-10i32, 1, 10i32, -1);
            let (i, j) = (mov.i, mov.j);
            let p = self.board[i];
            let q = self.board[j];
            let put_ = |mut board: [char; 120], i: usize, p: char| -> [char; 120] {
                board[i] = p;
                board
            };
            // Copy variables and reset ep and kp
            let mut board = self.board;
            let mut wc = self.wc;
            let mut bc = self.bc;
            let mut ep = 0;
            let mut kp = 0;
            //  !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!! TODO
            let score = self.score + self.value(&mov);
            // Actual move
            board = put_(board, j, board[i]);
            board = put_(board, i, '.');
            // Castling rights, we move the rook or capture the opponent's
            if i == a1 {
                wc = (false, wc.1);
            }
            if i == h1 {
                wc = (wc.0, false);
            }
            if j == a8 {
                bc = (bc.0, false);
            }
            if j == h8 {
                bc = (false, bc.1);
            }
            // Castling
            if p == 'K' {
                wc = (false, false);
                if (j as isize - i as isize).abs() == 2 {
                    kp = (i + j) / 2;
                    board = put_(board, if j < i { a1 } else { h1 }, '.');
                    board = put_(board, kp, 'R');
                }
            }
            // Pawn promotion, double move and en passant capture
            if p == 'P' {
                if a8 <= j && j <= h8 {
                    board = put_(board, j, mov.prom);
                }
                if (j as i32) - (i as i32) == 2 * n {
                    ep = (i as i32 + n) as usize;
                }
                if j == self.ep {
                    board = put_(board, (j as i32 + s) as usize, '.');
                }
            }
            Position {
                board: board,
                score: score,
                wc: wc,
                bc: bc,
                ep: ep,
                kp: kp,
            }
            .rotate(false)
        }
        fn value(&self, mov: &Move) -> i32 {
            let (a1, h1, a8, h8) = (91, 98, 21, 28);
            let (n, e, s, w) = (-10i32, 1, 10i32, -1);
            let (i, j) = (mov.i, mov.j);
            let p = self.board[i];
            let q = self.board[j];
            // Actual move
            let mut score = pst(p)[j] - pst(p)[i];
            // Capture
            if q.is_ascii_lowercase() {
                score += pst(q.to_ascii_uppercase())[119 - j];
            }
            // Castling check detection
            if (j as isize - self.kp as isize).abs() < 2 {
                score += pst('K')[119 - j];
            }
            // Castling
            if p == 'K' && ((i as isize - j as isize).abs() == 2) {
                score += pst('R')[(i + j) / 2];
                score -= pst('R')[if j < i { a1 } else { h1 }];
            }
            // Special pawn stuff
            if p == 'P' {
                if a8 <= j && j <= h8 {
                    score += pst(mov.prom)[j] - pst('P')[j];
                }
                if j == self.ep {
                    score += pst('P')[(119 - (j as i32 + s)) as usize]
                }
            }
            score
        }
    }
    //###############################################################################
    // Search logic
    //###############################################################################
    // lower <= s(pos) <= upper
    #[derive(Clone, Copy)]
    struct Entry {
        lower: i32,
        upper: i32,
    }

    struct Searcher {
        tp_score: HashMap<(Position, i32, bool), Entry>,
        tp_move: HashMap<Position, Move>,
        history: HashSet<Position>,
        nodes: u32,
    }
    impl Searcher {
        fn new() -> Searcher {
            Searcher {
                tp_score: HashMap::new(),
                tp_move: HashMap::new(),
                history: HashSet::new(),
                nodes: 0,
            }
        }
        fn bound(&mut self, pos: &Position, gamma: i32, mut depth: i32, can_null: bool) -> i32 {
            let mate_lower: i32 = piece('K') - 10 * piece('Q');
            let mate_upper: i32 = piece('K') + 10 * piece('Q');
            let default_entry: Entry = Entry {
                lower: -mate_upper,
                upper: mate_upper,
            };
            // Let s* be the "true" score of the sub-tree we are searching.
            // The method returns r, where
            // if gamma >  s* then s* <= r < gamma  (A better upper bound)
            // if gamma <= s* then gamma <= r <= s* (A better lower bound)
            self.nodes += 1;
            // Depth <= 0 is QSearch. Here any position is searched as deeply as is needed for
            // calmness, and from this point on there is no difference in behaviour depending on
            // depth, so so there is no reason to keep different depths in the transposition table.
            depth = max(depth, 0);
            // Sunfish is a king-capture engine, so we should always check if we
            // still have a king. Notice since this is the only termination check,
            // the remaining code has to be comfortable with being mated, stalemated
            //# or able to capture the opponent king.
            if pos.score <= -mate_lower {
                return -mate_upper;
            }
            // Look in the table if we have already searched this position before.
            // We also need to be sure, that the stored search was over the same
            // nodes as the current search.
            let entry = *self
                .tp_score
                .get(&(*pos, depth, can_null))
                .unwrap_or(&default_entry);
            if entry.lower >= gamma {
                return entry.lower;
            }
            if entry.upper < gamma {
                return entry.upper;
            }
            // Let's not repeat positions. We don't chat
            // - at the root (can_null=False) since it is in history, but not a draw.
            // - at depth=0, since it would be expensive and break "futility pruning".
            if can_null && depth > 0 && self.history.contains(&pos) {
                return 0;
            }
            // Call moves
            let moves = Self::moves(self, depth, can_null, pos, gamma);

            let mut best = -mate_upper;
            // First try not moving at all

            return 0;
        }
        // Generator of moves to search in order.
        // This allows us to define the moves, but only calculate them if needed.
        fn moves(
            &mut self,
            depth: i32,
            can_null: bool,
            pos: &Position,
            gamma: i32,
        ) -> Vec<(Option<Move>, i32)> {
            let qs = 40;
            let qs_a = 140;
            let mut ans: Vec<(Option<Move>, i32)> = Vec::new();
            // First try not moving at all. We only do this if there is at least one major
            // piece left on the board, since otherwise zugzwangs are too dangerous.
            // FIXME: We also can't null move if we can capture the opponent king.
            // Since if we do, we won't spot illegal moves that could lead to stalemate.
            // For now we just solve this by not using null-move in very unbalanced positions.
            // TODO: We could actually use null-move in QS as well. Not sure it would be very useful.
            // But still.... We just have to move stand-pat to be before null-move.
            //if depth > 2 and can_null and any(c in pos.board for c in "RBNQ"):
            //if depth > 2 and can_null and any(c in pos.board for c in "RBNQ") and abs(pos.score) < 500:
            if depth > 2 && can_null && pos.score.abs() < 500 {
                ans.push((
                    None,
                    -self.bound(&pos.rotate(true), 1 - gamma, depth - 3, true),
                ));
            }
            // For QSearch we have a different kind of null-move, namely we can just stop
            // and not capture anything else.
            if depth == 0 {
                ans.push((None, pos.score));
            }
            // Look for the strongest ove from last time, the hash-move.
            let mut killer = self.tp_move.get(pos);
            // If there isn't one, try to find one with a more shallow search.
            // This is known as Internal Iterative Deepening (IID). We set
            // can_null=True, since we want to make sure we actually find a move.
            if killer.is_none() && depth > 2 {
                self.bound(pos, gamma, depth - 3, false);
                killer = self.tp_move.get(pos);
            }
            // If depth == 0 we only try moves with high intrinsic score (captures and
            // promotions). Otherwise we do all moves. This is called quiescent search.
            let val_lower = qs - depth * qs_a;
            // Only play the move if it would be included at the current val-limit,
            // since otherwise we'd get search instability.
            // We will search it again in the main loop below, but the tp will fix
            // things for us.
            if let Some(killer_move) = killer {
                if pos.value(killer_move) >= val_lower {
                    ans.push((Some(*killer_move), -self.bound(&pos.domove(*killer_move), 1 - gamma, depth - 1, true)));
                }
            }


            ans
        }
    }

    println!("Hello, world!");
}
