// this is intended to be an exact replica of sunfish from https://github.com/thomasahle/sunfish
// rust specifics will only be used where absolutely needed.
use std::cmp::max;
use std::collections::HashMap;

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
    let letters: Vec<char> = (
        "         \n".to_owned() + //   0 -  9
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
        "         \n"
        // 110 -119
    )
    .chars()
    .collect();
    let initial: [char; 120] = letters.try_into().unwrap();
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
    // Constants for tuning search
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
            let (n, e, w) = (-10, 1, -1);

            let mut moves = Vec::new();
            for i in 0..120 {
                let p = self.board[i];
                if !p.is_ascii_uppercase() {
                    continue; // skip empty squares and opponent pieces
                }
                let directions = directions(p);
                for &d in &directions {
                    let mut j = i;
                    loop {
                        j = (j as i32 + d) as usize;
                        let q = self.board[j];
                        // Stay inside the board, and off friendly pieces
                        if q.is_ascii_uppercase() || q.is_whitespace() || q == '\n' {
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
                            if [n + w, n + e].contains(&d) && q == '.' && ![self.ep].contains(&j) {
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
                        if i == h1 && self.board[(j as i32 + w) as usize] == 'K' && self.wc.1 {
                            moves.push(Move {
                                i: (j as i32 + w) as usize,
                                j: (j as i32 + e) as usize,
                                prom: ' ',
                            })
                        }
                    }
                }
            }
            moves
        }
        fn rotate(&self, nullmove: bool) -> Position {
            Position {
                board: Self::swap_player(self.board),
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
        fn swap_player(board: [char; 120]) -> [char; 120] {
            let mut new_board = [' '; 120];
            for (i, &c) in board.iter().enumerate() {
                new_board[119 - i] = if c.is_ascii_lowercase() {
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
            let (n, s) = (-10i32, 10i32);
            let (i, j) = (mov.i, mov.j);
            let p = self.board[i];
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
            let s = 10i32;
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
        history: Vec<Position>,
        nodes: u32,
    }
    impl Searcher {
        fn new() -> Searcher {
            Searcher {
                tp_score: HashMap::new(),
                tp_move: HashMap::new(),
                history: Vec::new(),
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
            let moves: Vec<(Option<Move>, i32)> = Self::getmoves(self, depth, can_null, pos, gamma);

            let mut best = -mate_upper;
            for (mov, score) in moves {
                best = max(best, score);
                if best >= gamma {
                    // Save the move for pv construction and killer heuristic
                    if mov.is_some() {
                        self.tp_move.insert(*pos, mov.unwrap());
                    }
                    break;
                }
            }
            // Stalemate checking is a bit tricky: Say we failed low, because
            // we can't (legally) move and so the (real) score is -infty.
            // At the next depth we are allowed to just return r, -infty <= r < gamma,
            // which is normally fine.
            // However, what if gamma = -10 and we don't have any legal moves?
            // Then the score is actaully a draw and we should fail high!
            // Thus, if best < gamma and best < 0 we need to double check what we are doing.

            // We will fix this problem another way: We add the requirement to bound, that
            // it always returns MATE_UPPER if the king is capturable. Even if another move
            // was also sufficient to go above gamma. If we see this value we know we are either
            // mate, or stalemate. It then suffices to check whether we're in check.

            // Note that at low depths, this may not actually be true, since maybe we just pruned
            // all the legal moves. So sunfish may report "mate", but then after more search
            // realize it's not a mate after all. That's fair.

            // This is too expensive to test at depth == 0
            if depth > 2 && best == -mate_upper {
                let flipped = pos.rotate(true);
                // Hopefully this is already in the TT because of null-move
                let in_check = self.bound(&flipped, mate_upper, 0, true) == mate_upper;
                best = if in_check { -mate_lower } else { 0 };
            }
            // Table part 2
            if best >= gamma {
                self.tp_score.insert(
                    (*pos, depth, can_null),
                    Entry {
                        lower: best,
                        upper: entry.upper,
                    },
                );
            }
            if best < gamma {
                self.tp_score.insert(
                    (*pos, depth, can_null),
                    Entry {
                        lower: entry.lower,
                        upper: best,
                    },
                );
            }
            return best;
        }
        // Generator of moves to search in order.
        // This allows us to define the moves, but only calculate them if needed.
        fn getmoves(
            &mut self,
            depth: i32,
            can_null: bool,
            pos: &Position,
            gamma: i32,
        ) -> Vec<(Option<Move>, i32)> {
            let qs = 40;
            let qs_a = 140;
            let mate_lower: i32 = piece('K') - 10 * piece('Q');
            let mate_upper: i32 = piece('K') + 10 * piece('Q');

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
                return ans;
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
                    ans.push((
                        Some(*killer_move),
                        -self.bound(&pos.domove(*killer_move), 1 - gamma, depth - 1, true),
                    ));
                }
            }
            let moves_vec = pos.gen_moves();
            let mut ms1: Vec<(i32, &Move)> = moves_vec.iter().map(|m| (pos.value(m), m)).collect();
            ms1.sort_by_key(|(v, _)| -v);
            for (val, mov) in ms1 {
                // Quiescent search
                if val < val_lower {
                    break;
                }
                // If the new score is less than gamma, the opponent will for sure just
                // stand pat, since ""pos.score + val < gamma === -(pos.score + val) >= 1-gamma""
                // This is known as futility pruning.
                if depth <= 1 && pos.score + val < gamma {
                    // Need special case for MATE, since it would normally be caught
                    // before standing pat.
                    let scr = if val < mate_lower {
                        pos.score + val
                    } else {
                        mate_upper
                    };
                    ans.push((Some(*mov), scr));
                    // We can also break, since we have ordered the moves by value,
                    // so it can't get any better than this.
                    break;
                }
                ans.push((
                    Some(*mov),
                    -self.bound(&pos.domove(*mov), 1 - gamma, depth - 1, true),
                ));
            }
            ans
        }
        fn search(&mut self, history: Vec<Position>, depth: i32) -> Vec<(i32, i32, i32, Move)> {
            let mate_lower: i32 = piece('K') - 10 * piece('Q');
            let eval_roughness = 15;
            let mut ans = Vec::new();
            // Iterative deepening MTD-bi search
            self.nodes = 0;
            self.history = history.clone();
            self.tp_score.clear();
            let mut gamma = 0;
            // The inner loop is a binary search on the score of the position.
            // Inv: lower <= score <= upper
            // 'while lower != upper' would work, but it's too much effort to spend
            // on what's probably not going to change the move played.
            let (mut lower, mut upper) = (-mate_lower, mate_lower);
            while lower < upper - eval_roughness {
                let score = self.bound(&history[history.len() - 1], gamma, depth, false);
                let mv = *self
                    .tp_move
                    .get(&history[history.len() - 1])
                    .expect("move not in table");
                if score >= gamma {
                    lower = score;
                }
                if score < gamma {
                    upper = score;
                }
                ans.push((depth, gamma, score, mv));
                gamma = (lower + upper + 1) / 2;
            }
            ans
        }
    }
    //###############################################################################
    // UCI User interface
    //###############################################################################
    fn parse(c: [char; 2]) -> i32 {
        let a1 = 91;
        let fil = (c[0] as u8 - b'a') as i32;
        let rank = (c[1].to_digit(10).unwrap() as i32) - 1;
        return a1 + fil - 10 * rank;
    }
    fn chr(val: usize) -> String {
        //let chrs = b'a'..=b'h';
        //if val < 1 || val > 8 {
        //    return " ".to_string(); // Invalid input
        //}
        //(chrs.clone().nth(val - 1).unwrap() as char).to_string()
        let i: u8 = 96 + val as u8;
        (i as char).to_string()
    }
    fn render(i: usize) -> String {
        let h1: usize = 98;
        let rank = (h1 - i) / 10;
        let fil = i % 10;
        chr(fil) + &((rank + 1).to_string())
    }
    fn render_move(mov: Option<Move>, white_pov: bool) -> String {
        if mov.is_none() {
            return "(none)".to_string();
        }
        let (mut i, mut j) = (mov.unwrap().i, mov.unwrap().j);
        if !white_pov {
            (i, j) = (119 - i, 119 - j);
        }
        return render(i) + &render(j) + &mov.unwrap().prom.to_ascii_lowercase().to_string();
    }
    fn parse_move(move_str: &str, white_pov: bool) -> Move {
        let chars: Vec<char> = move_str.chars().collect();
        let mut i = parse([chars[0], chars[1]]);
        let mut j = parse([chars[2], chars[3]]);
        let prom = if chars.len() > 4 {
            chars[4].to_ascii_uppercase()
        } else {
            ' '
        };
        if !white_pov {
            (i, j) = (119 - i, 119 - j);
        }
        Move {
            i: i as usize,
            j: j as usize,
            prom,
        }
    }
    fn go_loop(
        searcher: &mut Searcher,
        hist: &Vec<Position>,
        max_movetime: i32,
        max_depth: i32,
        debug: bool,
    ) -> () {
        if debug {
            println!("Going movetime={max_movetime}, depth={max_depth}");
        }
        let start = std::time::Instant::now();
        for idepth in 1..max_depth + 1 {
            for (depth, gamma, score, _mov) in searcher.search(hist.clone(), idepth) {
                // Our max_depth implementation is a bit wasteful.
                // We never know when we've seen the last at a certain depth
                // before we get to the next one
                if depth - 1 >= max_depth {
                    break;
                }
                let elapsed = std::time::Instant::now() - start;
                if score >= gamma {
                    //println!("move return {}",
                    //    render_move(Some(_mov), (hist.len() % 2) == 0));
                    let pv_vec = pv(&searcher, &hist[hist.len() - 1]);
                    let pv_str = pv_vec.join("");
                    println!(
                        "info depth {} time {} nodes {} nps {} score cp {} lowerbound pv {}",
                        depth,
                        (1000.0 * elapsed.as_secs_f64()).round() as u64,
                        searcher.nodes,
                        if elapsed.as_secs_f64() > 0.0 {
                            (searcher.nodes as f64 / elapsed.as_secs_f64()).round() as u64
                        } else {
                            0
                        },
                        score,
                        pv_str,
                    );
                } else {
                    println!(
                        "info depth {} time {} nodes {} nps {} score cp {} upperbound",
                        depth,
                        (1000.0 * elapsed.as_secs_f64()).round() as u64,
                        searcher.nodes,
                        if elapsed.as_secs_f64() > 0.0 {
                            (searcher.nodes as f64 / elapsed.as_secs_f64()).round() as u64
                        } else {
                            0
                        },
                        score
                    );
                    // We may not have a move yet at depth = 1
                    if depth > 1
                        && elapsed > std::time::Duration::from_secs((max_movetime * 2 / 3) as u64)
                    {
                        break;
                    }
                }
            }
        }
        // FIXME: If we are in "go infinite" we aren't actually supposed to stop the
        // go-loop before we got stop_event. Unfortunately we currently don't know if
        // we are in "go infinite" since it's simply translated to "go depth 100".
        let my_pv = pv(searcher, &hist[hist.len() - 1]);
        println!(
            "bestmove {}",
            if !my_pv.is_empty() {
                my_pv[0].clone()
            } else {
                "(none)".to_string()
            }
        );
    }
    fn mate_loop(
        mut searcher: Searcher,
        hist: Vec<Position>,
        max_movetime: i32,
        max_depth: i32,
        find_draw: bool,
    ) -> () {
        let mate_lower: i32 = piece('K') - 10 * piece('Q');
        let start = std::time::Instant::now();
        for d in 1..max_depth + 1 {
            if find_draw {
                let s0 = searcher.bound(&hist[hist.len() - 1], 0, d, true);
                //let mut elapsed = std::time::Instant::now() - start;
                println!("info depth {} score lowerbound cp {}", d, s0);
                let s1 = searcher.bound(&hist[hist.len() - 1], 1, d, true);
                //elapsed = std::time::Instant::now() - start;
                println!("info depth {} score lowerbound cp {}", d, s1);
                if s0 >= 0 && s1 < 1 {
                    break;
                }
            } else {
                let score = searcher.bound(&hist[hist.len() - 1], mate_lower, d, true);
                let elapsed = std::time::Instant::now() - start;
                let pv_vec = pv(&searcher, &hist[hist.len() - 1]);
                let pv_str = pv_vec.join("");
                println!(
                    "info depth {} score lowerbound cp {} time {} pv {}",
                    d,
                    score,
                    (1000.0 * elapsed.as_secs_f64()).round() as u64,
                    pv_str
                );
                if score >= mate_lower {
                    break;
                }
            }
            let elapsed = std::time::Instant::now() - start;
            if elapsed > std::time::Duration::from_millis(max_movetime as u64) {
                break;
            }
        }
        let mov = searcher.tp_move.get(&hist[hist.len() - 1]).copied();
        let move_str = render_move(mov, (hist.len()) % 2 == 1);
        println!("bestmove {}", move_str);
    }
    fn _perft_count(pos: &Position, depth: i32) -> i32 {
        // Check that we didn't get to an illegal position
        if can_kill_king(pos) {
            return -1;
        }
        if depth == 0 {
            return 1;
        }
        let mut res = 0;
        for mov in pos.gen_moves() {
            let cnt = _perft_count(&(pos.domove(mov)), depth - 1);
            if cnt != -1 {
                res += cnt
            }
        }
        return res;
    }
    fn perft(pos: &Position, depth: i32) -> () {
        let mut total = 0;
        for mov in pos.gen_moves() {
            let move_uci = render_move(Some(mov), get_color(pos) == 0);
            let cnt = _perft_count(&pos.domove(mov), depth - 1);
            if cnt != -1 {
                println!("{move_uci}: {cnt}");
                total += cnt;
            }
        }
        println!("Nodes searched: {}", total);
    }
    fn input() -> String {
        use std::io::{self, Write};
        let mut s = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut s).expect("Failed to read line");
        s.trim().to_string()
    }
    fn run(startpos: Position) -> () {
        let qs_name = "QS";
        let qs_a_name = "QS_A";
        let eval_roughness_name = "EVAL_ROUGHNESS";
        let mut qs = 40;
        let mut qs_a = 140;
        let mut eval_roughness = 15;
        let (qs_min, qs_max) = (0, 300);
        let (qs_a_min, qs_a_max) = (0, 300);
        let (eval_roughness_min, eval_roughness_max) = (0, 50);
        let debug = false;
        let mut hist = vec![startpos];
        let mut searcher = Searcher::new();
        loop {
            let line = input();
            let args: Vec<&str> = line.split_whitespace().collect();
            if args.len() == 0 {
                continue;
            }
            if args[0] == "quit" {
                break;
            }
            if args[0] == "uci" {
                println!("id name {}", VERSION);
                println!(
                    "option name {} type spin default {} min {} max {}",
                    qs_name, qs, qs_min, qs_max
                );
                println!(
                    "option name {} type spin default {} min {} max {}",
                    qs_a_name, qs_a, qs_a_min, qs_a_max
                );
                println!(
                    "option name {} type spin default {} min {} max {}",
                    eval_roughness_name, eval_roughness, eval_roughness_min, eval_roughness_max
                );
                println!("uciok");
            }
            if args[0] == "setoption" {
                let uci_key = args[2];
                let uci_val: i32 = args[4].parse::<i32>().unwrap();
                if uci_key == qs_name {
                    qs = uci_val;
                } else if uci_key == qs_a_name {
                    qs_a = uci_val;
                } else if uci_key == eval_roughness_name {
                    eval_roughness = uci_val;
                } else {
                    println!("Unknown option: {}", uci_key);
                    continue;
                }
            }
            // FIXME: It seems we should reply to "isready" even while thinking.
            // See: https://talkchess.com/forum3/viewtopic.php?f=7&t=81233&start=10
            if args[0] == "isready" {
                println!("readyok")
            }
            if args[0] == "position" && args[1] == "startpos" {
                hist = vec![startpos];
                for (ply, mov) in args[3..].iter().enumerate() {
                    hist.push(hist[hist.len() - 1].domove(parse_move(mov, ply % 2 == 0)));
                }
            }
            if args[0] == "position" && args[1] == "fen" {
                let pos = from_fen(args[2], args[3], args[4], args[5], args[6], args[7]);
                println!("position score {}", pos.score);
                let mut hist = if get_color(&pos) == 0 {
                    vec![pos]
                } else {
                    vec![pos.rotate(false), pos]
                };
                if args.len() > 8 {
                    for (_ply, mov) in args[9..].iter().enumerate() {
                        hist.push(
                            hist[hist.len() - 1].domove(parse_move(mov, hist.len() % 2 == 1)),
                        );
                    }
                }
            }
            if args[0] == "go" {
                let think = 100 ^ 6;
                let max_depth = 30;
                if args.len() > 1 && args[1] == "infinite" {
                    go_loop(&mut searcher, &hist, think, max_depth, debug);
                } else if args.len() > 1 && args[1] == "movetime" {
                    let max_movetime: i32 = args[2].parse::<i32>().unwrap();
                    go_loop(&mut searcher, &hist, max_movetime, max_depth, debug);
                } else if args.len() > 1 && args[1] == "depth" {
                    let max_depth: i32 = args[2].parse::<i32>().unwrap();
                    go_loop(&mut searcher, &hist, think, max_depth, debug);
                } else {
                    println!("Unknown go command: {}", line);
                }
            }
        }
    }

    fn from_fen(
        board: &str,
        color: &str,
        castling: &str,
        enpas: &str,
        _hclock: &str,
        _fclock: &str,
    ) -> Position {
        let mut iboard = board.to_string();
        for i in 1..9 {
            iboard = iboard.replace(&i.to_string(), &".".repeat(i));
        }
        iboard = iboard.replace("/", "\n ");
        iboard = "         \n         \n ".to_string() + &iboard + "\n         \n         \n";
        let board: [char; 120] = iboard.chars().collect::<Vec<char>>().try_into().unwrap();
        let wc: (bool, bool) = (castling.contains("Q"), castling.contains("K"));
        let bc: (bool, bool) = (castling.contains("k"), castling.contains("q"));
        let ep: usize = if enpas != "-" && enpas.len() == 2 {
            parse([enpas.chars().nth(0).unwrap(), enpas.chars().nth(1).unwrap()]) as usize
        } else {
            0
        };
        let mut score: i32 = board
            .iter()
            .enumerate()
            .filter(|&(_i, &c)| c.is_uppercase())
            .map(|(i, &c)| pst(c)[i])
            .sum();
        score -= board
            .iter()
            .enumerate()
            .filter(|&(_i, &c)| c.is_lowercase())
            .map(|(i, &c)| pst(c.to_ascii_uppercase())[119 - i])
            .sum::<i32>();
        let pos = Position {
            board: board,
            score: score,
            wc: wc,
            bc: bc,
            ep: ep,
            kp: 0,
        };
        if color == "w" {
            return pos;
        } else {
            return pos.rotate(false);
        }
    }
    fn get_color(pos: &Position) -> i32 {
        //A slightly hacky way to to get the color from a sunfish position
        if pos.board[0] == '\n' { 1 } else { 0 }
    }
    fn can_kill_king(pos: &Position) -> bool {
        // If we just checked for opponent moves capturing the king, we would miss
        // captures in case of illegal castling.
        //MATE_LOWER = 60_000 - 10 * 929
        //return any(pos.value(m) >= MATE_LOWER for m in pos.gen_moves())
        for m in pos.gen_moves() {
            if pos.board[m.j] == 'k' || ((m.j as isize - pos.kp as isize).abs() < 2) {
                return true;
            }
        }
        return false;
    }
    fn pv(searcher: &Searcher, pos: &Position) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        let mut color = get_color(&pos);
        //let origc = color;
        let mut pos = pos.clone();
        loop {
            let mov = searcher.tp_move.get(&pos);
            // The tp may have illegal moves, given lower depths don't detect king killing
            if mov.is_none() || can_kill_king(&pos.domove(*mov.unwrap())) {
                break;
            }
            res.push(render_move(mov.cloned(), get_color(&pos) == 0));
            pos = pos.domove(*mov.unwrap());
            color = 1 - color;
        }
        return res;
    }

    let hist: Vec<Position> = vec![Position {
        board: initial,
        score: 0,
        wc: (true, true),
        bc: (true, true),
        ep: 0,
        kp: 0,
    }];

    run(hist[hist.len() - 1].clone());
}
