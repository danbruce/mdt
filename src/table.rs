// use pulldown_cmark::Alignment;
use MDResult;
use std::fmt::{self, Display};
use std::io::{Result, Write};
use std::iter;

pub trait TableFns {
    fn set_table_state(&mut self, state: TableState);
    fn table_state(&self) -> TableState;
    fn inc_col(&mut self);
    fn inc_index(&mut self);
    fn set_index(&mut self, idx: usize);
    fn index(&self) -> usize;
    fn table(&self) -> &[String];
}

pub trait Table: TableFns {
    const F_INNER_HORIZONTAL: char;
    const F_INNER_INTERSECT: char;
    const F_INNER_VERTICAL: char;
    const F_OUTER_LEFT_INTERSECT: char;
    const F_OUTER_LEFT_VERTICAL: char;
    const F_OUTER_RIGHT_INTERSECT: char;
    const F_OUTER_RIGHT_VERTICAL: char;
    const H_INNER_HORIZONTAL: char;
    const H_INNER_INTERSECT: char;
    const H_INNER_VERTICAL: char;
    const H_OUTER_LEFT_INTERSECT: char;
    const H_OUTER_LEFT_VERTICAL: char;
    const H_OUTER_RIGHT_INTERSECT: char;
    const H_OUTER_RIGHT_VERTICAL: char;
    const INNER_HORIZONTAL: char;
    const INNER_INTERSECT: char;
    const INNER_VERTICAL: char;
    const OUTER_BOTTOM_HORIZONTAL: char;
    const OUTER_BOTTOM_INTERSECT: char;
    const OUTER_BOTTOM_LEFT: char;
    const OUTER_BOTTOM_RIGHT: char;
    const OUTER_LEFT_INTERSECT: char;
    const OUTER_LEFT_VERTICAL: char;
    const OUTER_RIGHT_INTERSECT: char;
    const OUTER_RIGHT_VERTICAL: char;
    const OUTER_TOP_HORIZONTAL: char;
    const OUTER_TOP_INTERSECT: char;
    const OUTER_TOP_LEFT: char;
    const OUTER_TOP_RIGHT: char;
    fn new() -> Self;
    fn draw<W: Write>(&mut self, w: &mut W) -> Result<()> {
        let char_row = |left: char, hor: char, intr: char, right: char, w: &mut W| -> Result<()> {
            write!(w, "{}{}", left, hor)?;
            for col in 0..self.index() - 1 {
                let width = self.table()[col].len();
                write!(
                    w,
                    "{}{}",
                    iter::repeat(hor).take(width).collect::<String>(),
                    intr
                )?;
            }
            let width = self.table()[self.index() - 1].len();
            write!(
                w,
                "{}{}\n",
                iter::repeat(hor).take(width).collect::<String>(),
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
        write!(w, "{} ", Self::H_OUTER_LEFT_VERTICAL)?;
        for col in 0..self.index() - 1 {
            write!(w, "{}{}", self.table()[col], Self::H_INNER_VERTICAL)?;
        }
        write!(
            w,
            "{}{}\n",
            self.table()[self.index() - 1],
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
            write!(w, "{} ", Self::INNER_VERTICAL)?;
            for col in 0..self.index() - 1 {
                let idx = pos(row, col);
                write!(w, "{}{}", self.table()[idx], Self::INNER_VERTICAL)?;
            }
            write!(
                w,
                "{}{}\n",
                self.table()[pos(row, self.index() - 1)],
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
    fn push(&mut self, item: &str);
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

#[derive(Debug, Default)]
pub struct AsciiTable {
    table: Vec<String>,
    cur: usize,
    table_state: TableState,
    col_count: usize,
    // table_alignments: Vec<Alignment>,
    table_cell_index: usize,
}

macro_rules! impl_table {
    ($name:ident) => (
        impl TableFns for $name {
            fn table(&self) -> &[String] {
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
        }
    )
}

impl_table!(AsciiTable);

impl Table for AsciiTable {
    const F_INNER_HORIZONTAL: char = '-';
    const F_INNER_INTERSECT: char = '+';
    const F_INNER_VERTICAL: char = '|';
    const F_OUTER_LEFT_INTERSECT: char = '+';
    const F_OUTER_LEFT_VERTICAL: char = '|';
    const F_OUTER_RIGHT_INTERSECT: char = '+';
    const F_OUTER_RIGHT_VERTICAL: char = '|';
    const H_INNER_HORIZONTAL: char = '-';
    const H_INNER_INTERSECT: char = '+';
    const H_INNER_VERTICAL: char = '|';
    const H_OUTER_LEFT_INTERSECT: char = '+';
    const H_OUTER_LEFT_VERTICAL: char = '|';
    const H_OUTER_RIGHT_INTERSECT: char = '+';
    const H_OUTER_RIGHT_VERTICAL: char = '|';
    const INNER_HORIZONTAL: char = '-';
    const INNER_INTERSECT: char = '+';
    const INNER_VERTICAL: char = '|';
    const OUTER_BOTTOM_HORIZONTAL: char = '-';
    const OUTER_BOTTOM_INTERSECT: char = '+';
    const OUTER_BOTTOM_LEFT: char = '+';
    const OUTER_BOTTOM_RIGHT: char = '+';
    const OUTER_LEFT_INTERSECT: char = '+';
    const OUTER_LEFT_VERTICAL: char = '|';
    const OUTER_RIGHT_INTERSECT: char = '+';
    const OUTER_RIGHT_VERTICAL: char = '|';
    const OUTER_TOP_HORIZONTAL: char = '-';
    const OUTER_TOP_INTERSECT: char = '+';
    const OUTER_TOP_LEFT: char = '+';
    const OUTER_TOP_RIGHT: char = '+';

    // fn draw(&mut self, w: &mut impl Write) -> Result<()> {
    //     write!(
    //         w,
    //         "{}{}",
    //         AsciiTable::OUTER_TOP_LEFT,
    //         AsciiTable::OUTER_TOP_HORIZONTAL
    //     )?;
    //     let width: usize = self.table[0..self.table_cell_index]
    //         .iter()
    //         .map(|x| x.len())
    //         .sum();

    //     let row = 0;
    //     for col in 0..self.table_cell_index - 1 {
    //         let width = self.table[col].len();
    //         write!(
    //             w,
    //             "{}{}",
    //             iter::repeat(AsciiTable::OUTER_TOP_HORIZONTAL)
    //                 .take(width)
    //                 .collect::<String>(),
    //             AsciiTable::OUTER_TOP_INTERSECT
    //         )?;
    //     }

    //     write!(
    //         w,
    //         "{}{}",
    //         AsciiTable::OUTER_TOP_HORIZONTAL,
    //         AsciiTable::OUTER_TOP_RIGHT
    //     )?;

    //     Ok(())
    // }

    fn new() -> Self {
        AsciiTable {
            ..Default::default()
        }
    }

    fn push(&mut self, item: &str) {
        let len = self.table.len();
        if len == self.cur {
            self.table.push(String::from(item));
        } else {
            self.table[self.cur].push_str(&item);
        }
    }
}
