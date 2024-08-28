

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

        let mut create_price = CreatePrice::new(Currency::Inr, 1000);
        create_price.product = Some(IdOrCreate::Id(&product.id));
        create_price.unit_amount = Some(1000);
        create_price.expand = &["product"];

        let price = Price::create(create_price, &self.client).await.unwrap();

        let success_url = env::var("SUCCESS_URL").ok();
        let failure_url = env::var("FAILURE_URL").ok();

        let mut sesssion = CreateCheckoutSession::new();
        session.cancel_url = success_url.as_deref();
        session.success_url = success_url.as_deref();
        session.customer = None;
        session.mode = Some(CheckoutSessionMode::Payment);
        session.line_items = Some(vec![CreateCheckoutSessionLineItem{
            ..Default::default()
        }]);
        session.expand = &["line_items", "line_items.data.price.product"];

        let checkout_session = CheckoutSession::create(session, &self.client).await.unwrap();
        checkout_session.url.unwrap()
    }
}