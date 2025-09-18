use std::collections::HashMap;
pub(crate) fn hashmap() {
    let mut currencies = HashMap::new();

    currencies.insert("India", "Rupees");
    currencies.insert("United States", "USD");
    currencies.insert("United Kingdom", "GBP");
    currencies.insert("India", "INR");

    println!("{:?}", currencies);

    let mut currencies = HashMap::new();

    currencies.insert("India", "INR");
    currencies.insert("United States", "USD");
    currencies.insert("United Kingdom", "GBP");

    let currency_usa = currencies.get("United States");
    let currency_china = currencies.get("China");
    println!("Currency of USA is: {:?}", currency_usa);
    println!("Currency of China is: {:?}", currency_china);

    let mut currencies = HashMap::new();

    currencies.insert("India", "INR");
    currencies.insert("United States", "USD");
    currencies.insert("United Kingdom", "GBP");

    for (key, value) in &currencies {
        println!("{} : {}", key, value);
    }

    let mut currencies = HashMap::new();

    currencies.insert("India", "INR");
    currencies.insert("United States", "USD");
    currencies.insert("United Kingdom", "GBP");

    println!("{:?}", currencies);
    currencies.remove("United Kingdom");
    println!("{:?}", currencies);
}