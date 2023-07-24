#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod counter {

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Counter {
        /// Stores a single `bool` value on the storage.
        value: u64,
    }

    impl Counter {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { value: 0 }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self { value: 0 }
        }

        #[ink(message)]
        pub fn increment(&mut self) {
            self.value += 1;
        }

        #[ink(message)]
        pub fn get(&self) -> u64 {
            self.value
        }
    }

    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let counter_default = Counter::default();
            let counter_new = Counter::new();
            assert_eq!(counter_new.get(), 0);
            assert_eq!(counter_default.get(), 0);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut counter = Counter::new();
            assert_eq!(counter.get(), 0);
            counter.increment();
            assert_eq!(counter.get(), 1);
            counter.increment();
            assert_eq!(counter.get(), 2);
        }
    }
}
