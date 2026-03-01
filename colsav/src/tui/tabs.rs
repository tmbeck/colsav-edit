#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Header,
    Colonies,
    Units,
    Nations,
    TradeRoutes,
    Tribes,
    Map,
}

impl Tab {
    pub fn titles() -> [&'static str; 7] {
        [
            "Header",
            "Colonies",
            "Units",
            "Nations",
            "Trade Routes",
            "Tribes",
            "Map",
        ]
    }

    pub fn index(self) -> usize {
        match self {
            Tab::Header => 0,
            Tab::Colonies => 1,
            Tab::Units => 2,
            Tab::Nations => 3,
            Tab::TradeRoutes => 4,
            Tab::Tribes => 5,
            Tab::Map => 6,
        }
    }

    pub fn from_index(idx: usize) -> Self {
        match idx {
            0 => Tab::Header,
            1 => Tab::Colonies,
            2 => Tab::Units,
            3 => Tab::Nations,
            4 => Tab::TradeRoutes,
            5 => Tab::Tribes,
            _ => Tab::Map,
        }
    }

    pub fn next(self) -> Self {
        Self::from_index((self.index() + 1) % 7)
    }

    pub fn prev(self) -> Self {
        Self::from_index((self.index() + 6) % 7)
    }
}
