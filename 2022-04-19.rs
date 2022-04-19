struct Warehouse {
    widgets: Vec<Widget>
}

#[derive(Debug)]
struct Widget(u32);

impl Warehouse {
    pub fn iter(&self) -> impl Iterator<Item = &Widget> {
        WarehouseIter { warehouse: self, index: 0 }
    }
}

struct WarehouseIter<'v> {
    warehouse: &'v Warehouse,
    index: usize,
}

// Iterator: https://doc.rust-lang.org/std/iter/trait.Iterator.html
impl<'v> Iterator for WarehouseIter<'v> {
    type Item = &'v Widget;
    
    fn next<'a>(&'a mut self) -> Option<&'v Widget> {
        let i = self.index;
        if i >= self.warehouse.widgets.len() {
            None
        } else {
            self.index += 1;
            Some(&self.warehouse.widgets[i])
        }
    }
}

fn main() {
    let w = Warehouse { widgets: vec![Widget(1), Widget(2)] };
    for x in w.iter() {
        println!("{x:?}");
    }
}

