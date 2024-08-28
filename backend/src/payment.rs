use std::env;
use dotenv::dotenv;
use stripe::{CheckoutSession, Client, CreateCheckoutSession, CreateCheckoutSessionLineItems, CreatePrice, CreateProduct, Currency, IdOrCreate, Price, Product};

pub struct Payment {
    client: Client
}

impl Payment {
    pub fn new() -> Payment{
        dotenv().ok();

        let secret_key = env::var("STRIPE_KEY")
            .expect("Stripe Key is Not Set");

        let client = Client::new(secret_key);

        Payment {
            client
        }
    }

    pub async fn create_stripe_session(&self) -> String {
        let product = {
            let  create_product = CreateProduct::new("Test");
            Product::create(&self.client, create_product).await.unwrap()
        };

        let mut create_price = CreatePrice::new(Currency::INR);
        create_price.product  = Some(IdOrCreate::Id(&product.id));
        create_price.unit_amount = Some(1000);
        create_price.expand = &["product"];

        let price = Price::create(&self.client,create_price).await
            .unwrap();

        let success_page = env::var("SUCCESS_URL").ok();
        let failure_page = env::var("FAILURE_URL").ok();

        let mut session = CreateCheckoutSession::new();
        session.cancel_url = failure_page.as_deref();
        session.success_url = success_page.as_deref();
        session.customer = None;
        session.mode = Some(stripe::CheckoutSessionMode::Payment);
        session.line_items = Some(vec![CreateCheckoutSessionLineItems{
            quantity: Some(4),
            price: Some(price.id.to_string()),
            ..Default::default()
        }]);
        session.expand = &["line_items.data.price.product"];

        let checkout_session = CheckoutSession::create(&self.client, session)
            .await
            .unwrap();

        checkout_session.url.unwrap()
    }
}

