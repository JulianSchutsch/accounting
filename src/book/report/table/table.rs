pub enum TableAlignment {
    Left,
    Center,
    Right
}

pub enum TableEntry {
    Empty,
    String(TableAlignment, String),
    TitleRow(String),
    NewRow,
    RowSeparator,
}

type TableEntries = Vec<TableEntry>;
type TableEntriesIterator<'a> = std::slice::Iter<'a, TableEntry>;

pub struct Table {
    entries: TableEntries,
}

impl Table {
    pub fn new() -> Table {
        Table{
            entries: TableEntries::new()
        }
    }
    pub fn iter(&self) -> TableEntriesIterator {
        self.entries.iter()
    }

    pub fn insert_title_row(&mut self, s: &str) {
        self.insert(TableEntry::TitleRow(s.to_string()));
    }

    pub fn insert_str(&mut self, s: &str) {
        self.insert(TableEntry::String(TableAlignment::Left, s.to_string()));
    }

    pub fn insert_num<T: std::fmt::Display>(&mut self, s: T) {
        self.insert(TableEntry::String(TableAlignment::Right, format!("{}", s)));
    }

    pub fn new_line(&mut self) {
        self.insert(TableEntry::NewRow);
    }

    pub fn insert(&mut self, e:TableEntry) {
        self.entries.push(e);
    }

    pub fn insert_row_sep(&mut self) {
        self.insert(TableEntry::RowSeparator);
    }

    pub fn column_widths(&self) -> std::collections::HashMap<usize, usize> {
        let mut result_w = std::collections::HashMap::<usize, usize>::new();
        let mut column : usize = 0;
        for entry in self.iter() {
            match entry {
                TableEntry::NewRow => column = 0,
                TableEntry::Empty => column+=1,
                TableEntry::String(_, s) => {
                    result_w.entry(column).and_modify(|v| *v = std::cmp::max(*v, s.len())).or_insert(s.len());
                    column+=1;
                }
                TableEntry::TitleRow(_) => column = 0,
                _ => {}
            }
        }
        result_w
    }

    fn print_string(left: bool, column_width: usize, alignment: &TableAlignment, string: &String) {
        if !left {
            print!(" | ");
        }
        match alignment {
            TableAlignment::Left => print!("{:<width$}", string, width=column_width),
            TableAlignment::Center => print!("{:^width$}", string, width=column_width),
            TableAlignment::Right => print!("{:>width$}", string, width=column_width),
        }
    }

    pub fn print(&self) {
        let w = self.column_widths();
        let total_width : usize = w.len()*3+w.iter().fold(0, |acc, (_, value)| acc+value);
        let mut column : usize = 0;
        for entry in self.iter() {
            match entry {
                TableEntry::Empty => {
                    Self::print_string(column==0, w.get(&column).copied().unwrap_or(0), &TableAlignment::Left, &"".to_string());
                    column += 1;
                }
                TableEntry::String(a, s) => {
                    Self::print_string(column==0, w.get(&column).copied().unwrap_or(0), a, s);
                    column += 1;
                },
                TableEntry::NewRow => {
                    println!();
                    column = 0;
                }
                TableEntry::TitleRow(s) => {
                    println!("- {} -", s);
                }
                TableEntry::RowSeparator => {
                    println!("{:-<width$}", "", width=total_width);
                }
            }
        }
    }
}