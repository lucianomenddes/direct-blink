# DirectBlink Protocol

**Frictionless Social Commerce on Solana.**

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)](https://github.com/lucianomendes/direct-blink)
[![License: ISC](https://img.shields.io/badge/License-ISC-blue.svg)](https://opensource.org/licenses/ISC)
[![Solana](https://img.shields.io/badge/Solana-Powered-blueviolet)](https://solana.com)
[![Anchor](https://img.shields.io/badge/Anchor-Framework-red)](https://www.anchor-lang.com/)

---

**DirectBlink** is a social commerce infrastructure built on the Solana network. It enables sellers to transform social media posts into native checkouts (*Blinks*), eliminating redirects to external websites and significantly reducing cart abandonment rates.

The protocol's core mission is to create a zero-friction shopping experience directly within social feeds (like X, Instagram, or WhatsApp) while ensuring trust and transparency through smart contracts.

## 🚀 Key Features

*   **Zero-Friction Checkout:** Allow customers to purchase products without ever leaving their social media feed.
*   **Permissionless Affiliate Onboarding:** A viral growth engine where anyone with a Solana wallet can become a promoter for any product and earn commissions instantly. No manual approval is needed.
*   **Atomic & Instant Payments:** Utilizes Cross-Program Invocation (CPI) to automatically split fees and commissions at the moment of purchase. Affiliates and sellers get paid in real-time.
*   **Programmatic Trust:** Prices, stock levels, and affiliate commissions are immutably stored and enforced by on-chain programs, removing the need for trusted intermediaries.

## 🛠️ Tech Stack & Architecture

The project is a monorepo combining an on-chain program with a web-based frontend.

### On-chain (Solana / Anchor)

*   **Framework:** [Anchor (Rust)](https://www.anchor-lang.com/)
*   **Program (`dblk_engine`):** Manages all core logic, including product registration, affiliate state, and atomic checkouts.
*   **PDAs:** `ProductState` and `AffiliateState` PDAs store the state for products and affiliate relationships efficiently.
*   **Token Program:** Interacts with the SPL Token program for USDC transfers.

### Off-chain (Web App)

*   **Framework:** [Next.js 15](https://nextjs.org/)
*   **UI:** [Tailwind CSS](https://tailwindcss.com/) & [shadcn/ui](https://ui.shadcn.com/)
*   **Solana Actions API:** Implements the Solana Actions specification to render the native checkout buttons in social media feeds.
*   **Wallet Integration:** [Solana Wallet Adapter](https://github.com/solana-labs/wallet-adapter) for connecting user wallets.

## 🏁 Getting Started

### Prerequisites

*   [Node.js](https://nodejs.org/en/) (v18 or higher)
*   [NPM](https://www.npmjs.com/)
*   [Rust](https://www.rust-lang.org/tools/install)
*   [Solana CLI](https://docs.solana.com/cli/install)
*   [Anchor CLI](https://www.anchor-lang.com/docs/installation)

### Installation

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/lucianomendes/direct-blink.git
    cd direct-blink
    ```

2.  **Install frontend dependencies:**
    ```bash
    npm install
    ```

3.  **Build the Anchor program:**
    ```bash
    anchor build
    ```

## ⚙️ Usage

### Running Tests

To run the integration tests, use the following command:

```bash
anchor test
```

This will deploy the program to a local validator and run the test suite defined in `tests/`.

## 🗺️ Roadmap (MVP)

*   ✅ **Core Engine:** Finalize the `dblk_engine` Anchor program.
*   ⏳ **Merchant Dashboard:** Build the Next.js dashboard for product and affiliate management.
*   ⏳ **Viral Loop:** Implement and test the permissionless affiliate onboarding flow.
*   🚀 **Launch:** Deploy to Devnet and Mainnet.

## 🤝 Contributing

Contributions are welcome! Please feel free to open an issue or submit a pull request.

## 📄 License

This project is licensed under the ISC License. See the [LICENSE](LICENSE) file for details.
