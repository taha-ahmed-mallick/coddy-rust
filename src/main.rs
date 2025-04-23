fn main() {
    let mut prices = [2.99, 1.50, 5.00, 3.75, 4.20, 2.25, 7.91];

    println!("Original Prices:");
    // TODO: Use enumerate to print each item number and price
    // Expected: "Item 1: $2.99" etc.
    for (i, value) in prices.iter().enumerate() {
        println!("Item {}: ${}", i+1, value)
    }
    println!("\nBundle Deals:");
    // TODO: Use chunks to print pairs of prices and their sums
    // Expected: "Bundle 1: $2.99 + $1.50 = $4.49" etc.
    let mut count = 1;
    for chunk in prices.chunks(2) {
        if chunk.len() == 2 {
            println!(
                "Bundle {}: ${} + ${} = ${}",
                count,
                chunk[0],
                chunk[1],
                chunk[0] + chunk[1]
            );
        } else {
            println!("Bundle {}: ${}", count, chunk[0]);
        }
        count += 1;
    }
    // TODO: Use iter_mut to apply 10% discount to all prices

    println!("\nPrices after 10% discount:");
    // TODO: Print final prices after discount
    // Expected: "$2.69" etc.
    for price in prices.iter_mut() {
        *price *= 0.9;
        println!("${}", price);
    }
}
