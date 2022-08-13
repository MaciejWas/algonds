use std::collections::HashMap;

const N_LOG_N_TABLE: [(u32, u32); 40] = [
    (10, 30),
    (20, 80),
    (30, 120),
    (40, 200),
    (50, 250),
    (60, 300),
    (70, 420),
    (80, 480),
    (90, 540),
    (100, 600),
    (110, 660),
    (120, 720),
    (130, 910),
    (140, 980),
    (150, 1050),
    (160, 1120),
    (170, 1190),
    (180, 1260),
    (190, 1330),
    (200, 1400),
    (210, 1470),
    (220, 1540),
    (230, 1610),
    (240, 1680),
    (250, 1750),
    (260, 2080),
    (270, 2160),
    (280, 2240),
    (290, 2320),
    (300, 2400),
    (310, 2480),
    (320, 2560),
    (330, 2640),
    (340, 2720),
    (350, 2800),
    (360, 2880),
    (370, 2960),
    (380, 3040),
    (390, 3120),
    (400, 3200)
];

pub trait Function {
    fn max_y(&self) -> u32;

    fn inverse(&self, y: u32) -> Option<u32>;

    fn closest_inverse(&self, y: &u32) -> u32 {
        let mut corrected_y = y.clone();
        while corrected_y <= self.max_y() {
            if let Some(x) = self.inverse(corrected_y) {
                return x
            };
            corrected_y += 1;
        }  
        panic!("Inverse not found :(")
    }
}


pub struct NLogN {
    map: HashMap<u32, u32>
} impl NLogN {
    fn init() -> Self {
        let invert = |(x, y)| (y, x);
        let map: HashMap<u32, u32> = N_LOG_N_TABLE.clone().into_iter().map(invert).collect();
        Self { map }
    }
}

impl Function for NLogN {    
    fn inverse(&self, y: u32) -> Option<u32> { 
        self.map.get(&y).cloned()
    }

    fn max_y(&self) -> u32 { N_LOG_N_TABLE.len() as u32 }
}


pub struct N;

impl Function for N {    

    fn inverse(&self, y: u32) -> Option<u32> { 
        Some(y)
    }

    fn max_y(&self) -> u32 { u32::MAX }
}

impl From<String> for Box<dyn Function> {
    fn from(x: String) -> Box<dyn Function> { 
        if x.eq("nlogn") {
            return Box::new(NLogN::init());
        }

        if x.eq("n") {
            return Box::new(N);
        }

        panic!()
    } 
}

