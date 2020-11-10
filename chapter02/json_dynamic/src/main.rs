use serde_json::{Number, Value};

fn main() {
    let input_path = std::env::args().nth(1).unwrap();
    let output_path = std::env::args().nth(2).unwrap();

    let mut sales_and_products = {
        let sales_and_products_text = std::fs::read_to_string(&input_path).unwrap();
        serde_json::from_str::<Value>(&sales_and_products_text).unwrap()
    };

    if let Value::Number(n) = &sales_and_products["sales"][1]["quantity"] {
        sales_and_products["sales"][1]["quantity"] =
            Value::Number(Number::from_f64(n.as_f64().unwrap() + 1.5).unwrap());
    }

    std::fs::write(
        output_path,
        serde_json::to_string_pretty(&sales_and_products).unwrap(),
    )
    .unwrap();
}
