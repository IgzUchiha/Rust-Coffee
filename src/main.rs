#[macro_use]
extern crate maplit;



use std::collections::HashMap;
use std::io;

fn main() {
    let mut menu: HashMap<String, HashMap<String, HashMap<String, i32>>> = HashMap::new();
    menu.insert("espresso".to_string(), hashmap!{
        "ingredients".to_string() => hashmap!{"water".to_string() => 50, "coffee".to_string() => 18},
        "cost".to_string() => hashmap!{"$".to_string() => 150}
    });
    menu.insert("latte".to_string(), hashmap!{
        "ingredients".to_string() => hashmap!{"water".to_string() => 200, "milk".to_string() => 150, "coffee".to_string() => 24},
        "cost".to_string() => hashmap!{"$".to_string() => 250}
    });
    menu.insert("cappuccino".to_string(), hashmap!{
        "ingredients".to_string() => hashmap!{"water".to_string() => 250, "milk".to_string() => 100, "coffee".to_string() => 24},
        "cost".to_string() => hashmap!{"$".to_string() => 300}
    });

    let mut profit = 0;
    let mut resources: HashMap<String, i32> = HashMap::new();
    resources.insert("water".to_string(), 300);
    resources.insert("milk".to_string(), 200);
    resources.insert("coffee".to_string(), 100);

    let mut is_on = true;

    while is_on {
        println!("What would you like? (espresso/latte/cappuccino):");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim().to_string();

        if choice == "off" {
            is_on = false;
        } else if choice == "report" {
            println!("Milk: {}ml", resources["milk"]);
            println!("Water: {}ml", resources["water"]);
            println!("Coffee: {}g", resources["coffee"]);
            println!("Money: ${}", profit);
        } else {
            let drink = &menu[&choice];
            if is_resource_sufficient(&drink["ingredients"], &resources) {
                let payment = process_coins();
                if is_transaction_successful(payment, drink["cost"]["$"], &mut profit) {
                    make_coffee(&choice, &drink["ingredients"], &mut resources);
                }
            }
        }
    }
}

fn is_resource_sufficient(order_ingredients: &HashMap<String, i32>, resources: &HashMap<String, i32>) -> bool {
    for (item, amount) in order_ingredients {
        if *amount >= resources[item] {
            println!("Sorry, there is not enough {}.", item);
            return false;
        }
    }
    true
}

fn process_coins() -> i32 {
    println!("Please insert coins.");
    let mut total = 0;
    total += get_coin_input("quarters") * 25;
    total += get_coin_input("dimes") * 10;
    total += get_coin_input("nickels") * 5;
    total += get_coin_input("pennies") * 1;
    total
}

fn get_coin_input(coin: &str) -> i32 {
    let mut input = String::new();
    println!("How many {}?: ", coin);
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn is_transaction_successful(money_received: i32, drink_cost: i32, profit: &mut i32) -> bool {
    if money_received >= drink_cost {
        let change = money_received - drink_cost;
        println!("Here is ${} in change.", change);
        *profit += drink_cost;
        true
    } else {
        println!("Sorry, that's not enough money. Money refunded.");
        false
    }
}

fn make_coffee(drink_name: &str, order_ingredients: &HashMap<String, i32>, resources: &mut HashMap<String, i32>) {
    for (item, amount) in order_ingredients {
        resources.insert(item.to_string(), resources[item] - amount);
    }
    println!("Here is your {}☕️", drink_name);
}