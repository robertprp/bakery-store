enum Product {
    Bread,
    Cake,
    Cookie
}

impl From<Product> for String {
    fn from(product: Product) -> Self {
        match product {
            Product::Bread => "Bread".to_string(),
            Product::Cake => "Cake".to_string(),
            Product::Cookie => "Cookie".to_string(),
        }
    }
}

pub struct Bake {
    pub product_name: String,
}

impl Bake {
    pub fn new(product: Product) -> Self {
        Self {
            product_name: product.into(),
        }
    }

    pub fn bake(&self) {
        let (tx, rx) = std::sync::mpsc::channel::<Product>();

    }
}
