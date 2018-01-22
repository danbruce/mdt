pub trait Table {
    type Output;
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
    fn draw(&mut self) -> Self::Output;
    fn push_back(&mut self, item: impl Into<Self::Output>);
}

pub struct AsciiTable {
    pub table: Vec<String>,
}

impl Table for AsciiTable {
    type Output = String;

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

    fn draw(&mut self) -> Self::Output {
        let mut res = String::new();
        res
    }

    fn new() -> Self {
        AsciiTable { table: Vec::new() }
    }

    fn push_back(&mut self, item: impl Into<Self::Output>) {
        self.table.push(item.into());
    }
}
