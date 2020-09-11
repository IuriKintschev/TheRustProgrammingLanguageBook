pub mod cache {
    use std::{collections::HashMap, thread, time};
    struct Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        calculation: T,
        value: HashMap<u32, u32>,
    }

    impl<T> Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: HashMap::new(),
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            if self.value.contains_key(&arg) {
                let res = self.value.get(&arg).expect("Nenhum tratamento");

                return *res;
            }
            let v = (self.calculation)(arg);
            let res = self.value.entry(v).or_insert(v);
            return *res;
        }
    }

    pub fn generate_workout(intensity: u32, random_number: u32) {
        let mut expressive_closure = Cacher::new(|num| {
            println!("Calculation Slowy...");
            thread::sleep(time::Duration::from_secs(2));
            num
        });

        if intensity < 25 {
            println!("Today, do {} pushups!", expressive_closure.value(intensity));
            println!("Next, do {} situps!", expressive_closure.value(intensity));
        } else {
            if random_number == 3 {
                println!("Take a break today! Remember to stay hydrated!");
            } else {
                println!(
                    "Today, run for {} minutes!",
                    expressive_closure.value(intensity)
                );
            }
        }
    }

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        let v1 = c.value(1);
        let v2 = c.value(2);
        println!("{}", v1);

        assert_eq!(v2, 2);
    }
}
