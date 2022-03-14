pub struct List {
    data: Vec<i64>,
}

impl List {
    pub fn new() -> List {
        Self { data: vec![] }
    }
    pub fn insert(&mut self, num: i64) {
        let mut index = 0;
        let mut past = num;
        while index < self.data.len() {
            if past < self.data[index] {
                let copy = self.data[index];
                self.data[index] = past;
                past = copy;
            }
            index += 1;
        }
        self.data.push(past);
    }
    pub fn includes(&self, num: i64) -> bool {
        let mut i = 0;
        let mut j = self.data.len() - 1;
        let mut m;
        while i < j {
            m = i + ((j - i) / 2);
            if self.data[m] == num {
                return true;
            }
            if num > self.data[m] {
                i = m + 1;
                continue;
            } 
            if num < self.data[m] {
                if m == 0 {
                    return false;
                }
                j = m - 1;
            }
        }
        return self.data[i] == num;
    }
}
