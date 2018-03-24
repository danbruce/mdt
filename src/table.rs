use std::borrow::Cow;
use std::io::{Result, Write};
use std::iter;

pub trait TableFns<'a> {
    fn set_table_state(&mut self, state: TableState);
    fn set_width(&mut self, w: usize);
    fn width(&self) -> usize;
    fn table_state(&self) -> TableState;
    fn inc_col(&mut self);
    fn inc_index(&mut self);
    fn set_index(&mut self, idx: usize);
    fn index(&self) -> usize;
    fn table(&self) -> &[Cow<'a, str>];
}

pub trait Table<'a>: TableFns<'a> {
    const F_INNER_HORIZONTAL: char;
    const F_INNER_INTERSECT: char;
    const F_OUTER_LEFT_INTERSECT: char;
    const F_OUTER_RIGHT_INTERSECT: char;
    const H_INNER_VERTICAL: char;
    const H_OUTER_LEFT_VERTICAL: char;
    const H_OUTER_RIGHT_VERTICAL: char;
    const INNER_HORIZONTAL: char;
    const INNER_VERTICAL: char;
    const OUTER_BOTTOM_HORIZONTAL: char;
    const OUTER_BOTTOM_INTERSECT: char;
    const OUTER_BOTTOM_LEFT: char;
    const OUTER_BOTTOM_RIGHT: char;
    const OUTER_TOP_HORIZONTAL: char;
    const OUTER_TOP_INTERSECT: char;
    const OUTER_TOP_LEFT: char;
    const OUTER_TOP_RIGHT: char;

    fn new(width: usize) -> Self;

    fn draw<W: Write>(&mut self, w: &mut W) -> Result<()> {
        // let total_width: usize = self.table()[0..self.index() - 1]
        //     .iter()
        //     .map(|x| x.len())
        //     .sum();
        // if total_width > self.width() {
        //     let avg = total_width / self.width();
        // }

        let col_widths = get_ideal_widths(self.width(), &self.table());
        // println!("Column widths: {:?}", col_widths);

        let char_row = |left: char, hor: char, intr: char, right: char, w: &mut W| -> Result<()> {
            write!(w, "{}", left)?;
            for col in 0..self.index() - 1 {
                // let width = self.table()[col].len();
                write!(
                    w,
                    "{}{}",
                    iter::repeat(hor).take(col_widths[col]).collect::<String>(),
                    intr
                )?;
            }
            // let width = self.table()[self.index() - 1].len();
            write!(
                w,
                "{}{}\n",
                iter::repeat(hor)
                    .take(col_widths[col_widths.len() - 1])
                    .collect::<String>(),
                right
            )?;
            Ok(())
        };

        // top row
        char_row(
            Self::OUTER_TOP_LEFT,
            Self::OUTER_TOP_HORIZONTAL,
            Self::OUTER_TOP_INTERSECT,
            Self::OUTER_TOP_RIGHT,
            w,
        )?;

        // header row
        write!(w, "{}", Self::H_OUTER_LEFT_VERTICAL)?;
        for col in 0..self.index() - 1 {
            let pad_count: usize = {
                let diff: isize = col_widths[col] as isize - self.table()[col].len() as isize - 2;
                if diff < 0 {
                    0
                } else {
                    diff as usize
                }
            };
            write!(
                w,
                " {}{} {}",
                self.table()[col],
                iter::repeat(' ').take(pad_count).collect::<String>(),
                Self::H_INNER_VERTICAL
            )?;
        }

        let pad_count: usize = {
            let diff: isize = col_widths[self.index() - 1] as isize
                - self.table()[self.index() - 1].len() as isize - 2;
            if diff < 0 {
                0
            } else {
                diff as usize
            }
        };
        write!(
            w,
            " {}{} {}\n",
            self.table()[self.index() - 1],
            iter::repeat(' ').take(pad_count).collect::<String>(),
            Self::H_OUTER_RIGHT_VERTICAL
        )?;

        // bottom head
        char_row(
            Self::OUTER_BOTTOM_LEFT,
            Self::OUTER_BOTTOM_HORIZONTAL,
            Self::OUTER_BOTTOM_INTERSECT,
            Self::OUTER_BOTTOM_RIGHT,
            w,
        )?;

        // body rows
        let pos = |row: usize, col: usize| row * self.index() + col;

        for row in 1..(self.table().len() / self.index()) {
            write!(w, "{}", Self::INNER_VERTICAL)?;
            for col in 0..self.index() - 1 {
                let idx = pos(row, col);
                let max_text_width = col_widths[col] - 2;
                let pad_count: usize = {
                    let text_width: usize = if self.table()[idx].len() > max_text_width {
                        max_text_width
                    } else {
                        self.table()[idx].len()
                    };
                    let diff: isize =
                        col_widths[col] as isize - text_width as isize;
                    if diff < 0 {
                        0
                    } else {
                        (diff - 2) as usize
                    }
                };
                write!(
                    w,
                    " {}{} {}",
                    if self.table()[idx].len() < col_widths[col] {
                        self.table()[idx].to_string()
                    } else {
                        let mut s = self.table()[idx][0..(max_text_width-3)].to_string();
                        s.push_str("...");
                        s
                    },
                    iter::repeat(' ').take(pad_count).collect::<String>(),
                    Self::INNER_VERTICAL
                )?;
            }
            let pad_count: usize = {
                let diff: isize =
                    col_widths[self.index() - 1] as isize - self.table()[pos(row, self.index() - 1)].len() as isize - 2;
                if diff < 0 {
                    0
                } else {
                    diff as usize
                }
            };
            write!(
                w,
                " {}{} {}\n",
                self.table()[pos(row, self.index() - 1)],
                iter::repeat(' ').take(pad_count).collect::<String>(),
                Self::INNER_VERTICAL
            )?;
        }

        // footer row
        char_row(
            Self::F_OUTER_LEFT_INTERSECT,
            Self::F_INNER_HORIZONTAL,
            Self::F_INNER_INTERSECT,
            Self::F_OUTER_RIGHT_INTERSECT,
            w,
        )?;

        Ok(())
    }
    fn push(&mut self, item: Cow<'a, str>);
}

#[derive(Debug, Clone, Copy)]
pub enum TableState {
    Head,
    Body,
}

impl Default for TableState {
    fn default() -> Self {
        TableState::Head
    }
}

macro_rules! impl_table {
    ($name:ident) => (
        impl<'a> TableFns<'a> for $name<'a> {
            fn table(&self) -> &[Cow<'a, str>] {
                self.table.as_slice()
            }
            fn set_table_state(&mut self, state: TableState) {
                self.table_state = state;
            }

            fn table_state(&self) -> TableState {
                self.table_state
            }

            fn inc_col(&mut self) {
                self.col_count += 1;
            }

            fn inc_index(&mut self) {
                self.table_cell_index += 1;
                self.cur += 1;
            }

            fn index(&self) -> usize {
                self.table_cell_index
            }

            fn set_index(&mut self, idx: usize) {
                self.table_cell_index = idx;
            }

            fn set_width(&mut self, w: usize) {
                self.width = w;
            }

            fn width(&self) -> usize {
                self.width
            }
        }
    )
}

#[derive(Debug, Default)]
pub struct AsciiTable<'a> {
    table: Vec<Cow<'a, str>>,
    cur: usize,
    table_state: TableState,
    col_count: usize,
    // table_alignments: Vec<Alignment>,
    table_cell_index: usize,
    width: usize,
}

impl_table!(AsciiTable);

impl<'a> Table<'a> for AsciiTable<'a> {
    const F_INNER_HORIZONTAL: char = '-';
    const F_INNER_INTERSECT: char = '┴';
    const F_OUTER_LEFT_INTERSECT: char = '└';
    const F_OUTER_RIGHT_INTERSECT: char = '┘';
    const H_INNER_VERTICAL: char = '|';
    const H_OUTER_LEFT_VERTICAL: char = '|';
    const H_OUTER_RIGHT_VERTICAL: char = '|';
    const INNER_HORIZONTAL: char = '-';
    const INNER_VERTICAL: char = '|';
    const OUTER_BOTTOM_HORIZONTAL: char = '-';
    const OUTER_BOTTOM_INTERSECT: char = '+';
    const OUTER_BOTTOM_LEFT: char = '+';
    const OUTER_BOTTOM_RIGHT: char = '+';
    const OUTER_TOP_HORIZONTAL: char = '-';
    const OUTER_TOP_INTERSECT: char = '+';
    const OUTER_TOP_LEFT: char = '+';
    const OUTER_TOP_RIGHT: char = '+';

    fn new(width: usize) -> Self {
        AsciiTable {
            width,
            ..AsciiTable::default()
        }
    }

    fn push(&mut self, item: Cow<'a, str>) {
        let len = self.table.len();
        if len == self.cur {
            self.table.push(item);
        } else {
            self.table[self.cur].to_mut().push_str(&item);
        }
    }
}

fn divide_term_width_lossless(width: usize, cols: usize) -> Vec<usize> {
    let mut v = Vec::with_capacity(cols);
    // potentially implement a minimum width instead
    if cols * 3 > width {
        panic!("Cannot render table, terminal too narrow.");
    }
    let min: usize = width / cols;
    let remaining: usize = width - min * cols;
    for i in 0..cols {
        v.push({
            if i >= remaining {
                min
            } else {
                min + 1
            }
        });
    }
    v
}

fn get_ideal_widths<'a>(term_width: usize, table: &[Cow<'a, str>]) -> Vec<usize> {
    let cols = 3;
    divide_term_width_lossless(term_width - cols - 1, cols)
}

// Unicode table impl
#[derive(Debug, Default)]
pub struct UnicodeTable<'a> {
    table: Vec<Cow<'a, str>>,
    cur: usize,
    table_state: TableState,
    col_count: usize,
    table_cell_index: usize,
    width: usize,
}

impl_table!(UnicodeTable);

impl<'a> Table<'a> for UnicodeTable<'a> {
    const F_INNER_HORIZONTAL: char = '─';
    const F_INNER_INTERSECT: char = '┴';
    const F_OUTER_LEFT_INTERSECT: char = '└';
    const F_OUTER_RIGHT_INTERSECT: char = '┘';
    const H_INNER_VERTICAL: char = '│';
    const H_OUTER_LEFT_VERTICAL: char = '│';
    const H_OUTER_RIGHT_VERTICAL: char = '│';
    const INNER_HORIZONTAL: char = '─';
    const INNER_VERTICAL: char = '│';
    const OUTER_BOTTOM_HORIZONTAL: char = '─';
    const OUTER_BOTTOM_INTERSECT: char = '┼';
    const OUTER_BOTTOM_LEFT: char = '├';
    const OUTER_BOTTOM_RIGHT: char = '┤';
    const OUTER_TOP_HORIZONTAL: char = '─';
    const OUTER_TOP_INTERSECT: char = '┬';
    const OUTER_TOP_LEFT: char = '┌';
    const OUTER_TOP_RIGHT: char = '┐';

    fn new(width: usize) -> Self {
        UnicodeTable {
            width,
            ..UnicodeTable::default()
        }
    }

    fn push(&mut self, item: Cow<'a, str>) {
        let len = self.table.len();
        if len == self.cur {
            self.table.push(Cow::from(item.trim().to_string()));
        } else {
            self.table[self.cur].to_mut().push_str(&item.trim());
        }
    }
}
