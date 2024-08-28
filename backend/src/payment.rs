

pub struct Payment{
    client: Client
}


impl Payment{

    pub fn new() -> Payment{
        dotenv().ok();

        let secret_key = env::var("STRIPE_KEY").expect("STRIPE_KEY must be set");
        let client = Client::new(secret_key);
        Payment { client }
    }

    pub async fn create_payment_session(&self) -> String {
        let product = {
            let mut create_product = CreateProduct::new("test");
            Product::create(create_product, &self.client).await.unwrap();
        };

        let price = {
            let mut create_price = CreatePrice::new(Currency::Inr, 1000);
            create_price.product = Some(IdOrCreate::Id(&product.id));
        };
        
        
    }
}