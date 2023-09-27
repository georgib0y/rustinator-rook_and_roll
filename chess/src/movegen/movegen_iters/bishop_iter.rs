use crate::board::board::{Board, ALL_PIECES, BISHOP, ROOK};
use crate::movegen::move_tables::MT;
use crate::movegen::movegen::{get_xpiece, ALL_SQUARES, NO_SQUARES};
use crate::movegen::movegen_iters::MovegenIterator;
use crate::movegen::moves::{Move, MoveType};

pub struct BishopQuietIterator {
    piece: u32,
    bishops: u64,
    occ: u64,
    quiet: u64,
    target: u64,
}

impl BishopQuietIterator {
    pub fn new(b: &Board, pinned: u64, target: u64) -> BishopQuietIterator {
        let bishops = b.pieces[BISHOP + b.ctm] & !pinned;
        let occ = b.util[ALL_PIECES];
        let from = bishops.trailing_zeros() as usize % 64;
        let quiet = MT::bishop_moves(occ, from) & !occ & target;

        BishopQuietIterator {
            piece: (BISHOP + b.ctm) as u32,
            bishops,
            occ,
            quiet,
            target,
        }
    }

    fn next_quiet(&mut self) -> Option<Move> {
        if self.bishops == 0 || (self.quiet == 0 && self.bishops.count_ones() == 1) {
            return None;
        }

        let mut from = self.bishops.trailing_zeros();

        if self.quiet == 0 {
            self.bishops &= self.bishops - 1;
            from = self.bishops.trailing_zeros();
            self.quiet = MT::bishop_moves(self.occ, from as usize) & !self.occ & self.target;
            if self.quiet == 0 {
                return None;
            }
        }

        let to = self.quiet.trailing_zeros();
        self.quiet &= self.quiet - 1;

        Some(Move::new(from, to, self.piece, 0, MoveType::Quiet))
    }
}

impl Iterator for BishopQuietIterator {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_quiet()
    }
}

pub struct BishopAttackIterator<'a> {
    board: &'a Board,
    piece: u32,
    bishops: u64,
    occ: u64,
    attack: u64,
    opp: u64,
}

impl<'a> BishopAttackIterator<'a> {
    pub fn new(b: &Board, pinned: u64, target: u64) -> BishopAttackIterator {
        let bishops = b.pieces[BISHOP + b.ctm] & !pinned;
        let opp = b.util[b.ctm ^ 1] & target;
        let occ = b.util[ALL_PIECES];
        let from = bishops.trailing_zeros() % 64;
        let attack = MT::bishop_moves(occ, from as usize) & opp;

        BishopAttackIterator {
            board: b,
            piece: (BISHOP + b.ctm) as u32,
            bishops,
            occ,
            attack,
            opp,
        }
    }

    fn next_attack(&mut self) -> Option<Move> {
        if self.bishops == 0 || (self.attack == 0 && self.bishops.count_ones() == 1) {
            return None;
        }

        let mut from = self.bishops.trailing_zeros();

        if self.attack == 0 {
            self.bishops &= self.bishops - 1;
            from = self.bishops.trailing_zeros();
            self.attack = MT::bishop_moves(self.occ, from as usize) & self.opp;
            if self.attack == 0 {
                return None;
            }
        }

        let to = self.attack.trailing_zeros();
        self.attack &= self.attack - 1;

        let xpiece = get_xpiece(self.board, to).unwrap();

        Some(Move::new(from, to, self.piece, xpiece, MoveType::Cap))
    }
}

impl<'a> Iterator for BishopAttackIterator<'a> {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_attack()
    }
}

#[test]
fn bishop_moves_iter() {
    crate::init();
    let b = Board::new_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq -").unwrap();
    let wquiet = BishopQuietIterator::new(&b, NO_SQUARES, ALL_SQUARES);
    let moves: Vec<_> = wquiet.collect();
    moves.iter().for_each(|m| println!("{m}"));
    assert_eq!(moves.len(), 7);

    let wattack = BishopAttackIterator::new(&b, NO_SQUARES, ALL_SQUARES);
    assert_eq!(wattack.count(), 2);

    let b = Board::new_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 b kq -").unwrap();

    let bquiet = BishopQuietIterator::new(&b, NO_SQUARES, ALL_SQUARES);
    assert_eq!(bquiet.count(), 6);

    let battack = BishopAttackIterator::new(&b, NO_SQUARES, ALL_SQUARES);
    assert_eq!(battack.count(), 3);
}
