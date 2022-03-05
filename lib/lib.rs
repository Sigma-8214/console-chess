use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Game {
    bord: [[Option<Peice>; 8]; 8],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Peice {
    piece_color: PeiceColor,
    piece_type: PieceType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PeiceColor {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bhishop,
    Knight,
    Pawn,
}

macro_rules! peice {
    ($color:tt, $type:tt) => {
        Some(Peice {
            piece_type: PieceType::$type,
            piece_color: PeiceColor::$color,
        })
    };
}

impl Game {
    pub fn new() -> Self {
        let mut bord = [[None; 8]; 8];
        bord[0] = [
            peice!(White, Rook),
            peice!(White, Knight),
            peice!(White, Bhishop),
            peice!(White, Queen),
            peice!(White, King),
            peice!(White, Bhishop),
            peice!(White, Knight),
            peice!(White, Rook),
        ];
        bord[1] = [peice!(White, Pawn); 8];

        bord[6] = [peice!(Black, Pawn); 8];
        bord[7] = [
            peice!(Black, Rook),
            peice!(Black, Knight),
            peice!(Black, Bhishop),
            peice!(Black, Queen),
            peice!(Black, King),
            peice!(Black, Bhishop),
            peice!(Black, Knight),
            peice!(Black, Rook),
        ];
        Game { bord }
    }

    pub fn parse_fen(inp: &str) -> Self {
        let mut bord = [[None; 8]; 8];

        let parts = inp.split(' ').collect::<Vec<&str>>();
        let rows = parts[0].split('/').collect::<Vec<&str>>();

        assert_eq!(rows.len(), 8);
        for (i, e) in rows.iter().enumerate() {
            bord[i] = Self::parse_fen_row(e);
        }

        Game { bord }
    }

    fn parse_fen_row(inp: &str) -> [Option<Peice>; 8] {
        let mut row = [None; 8];
        let chars = inp.chars().collect::<Vec<char>>();

        let mut i = 0;
        for e in chars {
            if let Some(e) = e.to_digit(10) {
                i += e as usize;
                continue;
            }

            let piece_color = if e as u8 >= 97 {
                PeiceColor::Black
            } else {
                PeiceColor::White
            };

            let char = e.to_uppercase().next().unwrap();
            let piece_type = match char {
                'P' => PieceType::Pawn,
                'N' => PieceType::Knight,
                'B' => PieceType::Bhishop,
                'R' => PieceType::Rook,
                'Q' => PieceType::Queen,
                'K' => PieceType::King,
                _ => unreachable!(),
            };

            row[i] = Some(Peice {
                piece_type,
                piece_color,
            });

            i += 1;
        }

        row
    }

    pub fn render(&self) {
        let mut row_alt = false;
        let mut line_alt = true;
        for x in self.bord {
            for y in x {
                let mut color = "30";
                if let Some(i) = y {
                    if i.piece_color == PeiceColor::White {
                        color = "97";
                    }
                }

                if row_alt ^ line_alt {
                    print!("\x1B[0;{};47m", color);
                } else {
                    print!("\x1B[0;{};100m", color);
                }
                row_alt = !row_alt;

                print!(
                    "{} ",
                    match y {
                        Some(i) => i.to_string(),
                        None => " ".to_owned(),
                    }
                );
            }
            line_alt = !line_alt;
            println!("\x1B[0;0;0m");
        }
    }
}

impl fmt::Display for Peice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.piece_type {
            PieceType::King => write!(f, "♚"),
            PieceType::Queen => write!(f, "♛"),
            PieceType::Knight => write!(f, "♞"),
            PieceType::Bhishop => write!(f, "♝"),
            PieceType::Rook => write!(f, "♜"),
            PieceType::Pawn => write!(f, "♟︎"),
        }
    }
}
