#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Header,
    Colonies,
    Units,
    Nations,
    Map,
}

impl Tab {
    pub fn titles() -> [&'static str; 5] {
        ["Header", "Colonies", "Units", "Nations", "Map"]
    }

    pub fn index(self) -> usize {
        match self {
            Tab::Header => 0,
            Tab::Colonies => 1,
            Tab::Units => 2,
            Tab::Nations => 3,
            Tab::Map => 4,
        }
    }

    pub fn from_index(idx: usize) -> Self {
        match idx {
            0 => Tab::Header,
            1 => Tab::Colonies,
            2 => Tab::Units,
            3 => Tab::Nations,
            _ => Tab::Map,
        }
    }

    pub fn next(self) -> Self {
        Self::from_index((self.index() + 1) % 5)
    }

    pub fn prev(self) -> Self {
        Self::from_index((self.index() + 4) % 5)
    }
}
