# Orion

Orion is a basic integration of Stripe payment processing using a Next.js frontend and a Rust backend with Actix Web. It showcases a simple checkout process where users can initiate a payment, which then redirects to Stripe's checkout page.

## Getting Started

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/KekmaTime/orion.git
   ```

2. Install dependencies:

```bash
cd orion/frontend
bun install


cd orion/backend
cargo build
```

### Running the Application

1. Start the backend server:

```bash
cd orion/backend
cargo run --quiet
```

2. Start the frontend server:

```bash
cd orion/frontend
bun run dev
```

Open your browser and navigate to http://localhost:3000 to access the application.  

### Testing

- Click the "Checkout" button on the homepage.
- You will be redirected to Stripe's checkout page.
- Use Stripe's test card numbers to simulate successful or failed payments.
- After completing the payment process, you will be redirected back to the success or failure page based on the outcome.

**Note**: Ensure both frontend and backend servers are running simultaneously for the full functionality to work.