use tree_set::TreeSet;

struct TreeSetTester<T> where T : PartialOrd {
    set: TreeSet<T>,
    exp_values: Vec<T>,
    unexp_values: Vec<T>,
}

impl<T> TreeSetTester<T> where T : PartialOrd, T: Copy, T: std::fmt::Debug {
    pub fn new(sample_values: Vec<T>) -> TreeSetTester<T> {
        TreeSetTester{set:TreeSet::new(), exp_values:vec![], unexp_values:sample_values}
    }

    pub fn add_and_check(&mut self, value: T) {
        self.set.add(value);
        self.exp_values.push(value);
        self.unexp_values.retain(|&x| x != value);
        self.check();
    }

    pub fn remove_and_check(&mut self, value: T) {
        self.set.remove(&value);
        self.exp_values.retain(|&x| x != value);
        self.unexp_values.push(value);
        self.check();
    }

    fn check(&self) {
        println!("Set = {}", self.set.to_string());

        for v in &self.exp_values {
            assert!(self.set.contains(v), "must contains {:?}, Set = {}", v, self.set.to_string());
        }
        for v in &self.unexp_values {
            assert!(!self.set.contains(v), "must not contains {:?}, Set = {}", v, self.set.to_string());
        }
    }
}

#[test]
fn test_add() {
    let mut tester = TreeSetTester::new((0..1000).collect::<Vec<u32>>());

    tester.add_and_check(20);
    tester.add_and_check(10);
    tester.add_and_check(5);
    tester.add_and_check(15);
    tester.add_and_check(30);
    tester.add_and_check(25);
    tester.add_and_check(35);
}

#[test]
fn test_remove_leaf() {
    let mut tester = TreeSetTester::new((0..1000).collect::<Vec<u32>>());

    tester.add_and_check(40);
    tester.add_and_check(20);
    tester.add_and_check(60);
    tester.add_and_check(10);
    tester.add_and_check(30);
    tester.add_and_check(50);
    tester.add_and_check(70);

    tester.remove_and_check(10);
    tester.remove_and_check(30);
    tester.remove_and_check(50);
    tester.remove_and_check(70);
}

#[test]
fn test_remove_middle1() {
    let mut tester = TreeSetTester::new((0..1000).collect::<Vec<u32>>());

    tester.add_and_check(40);
    tester.add_and_check(20);
    tester.add_and_check(60);
    tester.add_and_check(10);
    tester.add_and_check(30);
    tester.add_and_check(50);
    tester.add_and_check(70);

    tester.remove_and_check(20);
    tester.remove_and_check(60);
}

#[test]
fn test_remove_middle2() {
    let mut tester = TreeSetTester::new((0..1000).collect::<Vec<u32>>());

    tester.add_and_check(40);
    tester.add_and_check(20);
    tester.add_and_check(60);
    tester.add_and_check(10);
    tester.add_and_check(30);
    tester.add_and_check(50);
    tester.add_and_check(70);

    for x in (0..8).map(|v| v * 10 + 5) {
        tester.add_and_check(x);
    }

    tester.remove_and_check(20);
    tester.remove_and_check(60);
}

#[test]
fn test_remove_root() {
    let mut tester = TreeSetTester::new((0..1000).collect::<Vec<u32>>());

    tester.add_and_check(40);
    tester.add_and_check(20);
    tester.add_and_check(10);
    tester.add_and_check(30);
    tester.add_and_check(60);
    tester.add_and_check(50);
    tester.add_and_check(70);

    tester.remove_and_check(40);
}

#[test]
fn test_remove_single_root() {
    let mut tester = TreeSetTester::new((0..1000).collect::<Vec<u32>>());

    tester.add_and_check(40);
    tester.remove_and_check(40);
}
