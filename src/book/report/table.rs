pub struct Row {
    header: String,
    data: Vec<String>,
}

pub struct Table {
    columns_widths: Vec<usize>,
    columns_headers: Vec<String>,
    rows: Vec<Row>,
}

pub struct HeaderIterator<'l> {
    pos: usize,
    table: &'l Table,
}

impl<'l> Iterator for HeaderIterator<'l> {
    type Item = (usize, &'l String);
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos<self.table.columns_widths.len() {
            unsafe {
                let width : usize = self.table.columns_widths.get_unchecked(self.pos).clone();
                let string : &'l String = self.table.columns_headers.get_unchecked(self.pos);
                let result = Some((width, string));
                self.pos += 1;
                result
            }
        } else {
            None
        }
    }
}

impl Table {
    fn new(column_headers: Vec<String>) -> Table {
        Table{
            columns_widths: column_headers.iter().map(|v|v.len()).collect(),
            columns_headers: column_headers,
            rows: Vec::<Row>::new()
        }
    }
    
    fn iter_header(&self) -> HeaderIterator {
        HeaderIterator{
            pos: 0,
            table: self,
        }
    }
    
    fn max_row_header_width(&self) -> usize {
        return self.rows.iter().fold(0, |v,e| std::cmp::max(e.header.len(), v));
    }

    pub fn print(&self) {
        let headers : String = self.iter_header().map(
            |(width, string)| format!(" {:width$}", string, width=width)).collect();
        println!("{:rhw$} | {}",
            "", headers, rhw =self.max_row_header_width());
    }
}
