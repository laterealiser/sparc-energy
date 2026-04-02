# Supabase Project Configuration Guide

To enable **BaaS (Backend-as-a-Service)** for the Sparc Energy marketplace, follow these setup steps in your [Supabase Dashboard](https://supabase.com/dashboard).

## 1. Authentication (Auth)
Enable the following settings in **Authentication > Providers**:
- **Email/Password**: (Enabled)
- **Email OTP (2FA)**: (Recommended) — Turn on for security.
- **Site URL**: `https://sparcenergy.in` (Vercel URL).

## 2. Storage Buckets
Create the following **Public Buckets** in **Storage**:
- `project-docs`: For PDFs, audit reports, and satellite images.
- `kyc-documents`: For identity proof and business verification docs.
- `certificates`: For generated retirement PDF certificates.

## 3. Realtime
Enable Realtime for the matching engine to push live orders and prices to the frontend:
- Go to **Database > Replication**.
- Enable **Supabase Realtime** for your `trades` and `market_orders` tables (once synced).

---

## 4. Environment Variables (`.env`)
Copy these keys from **Project Settings > API** into your `.env` file and Render/Vercel dashboards:

```bash
# Supabase Keys
SUPABASE_URL=https://your-project-id.supabase.co
SUPABASE_ANON_KEY=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...

# Oracle Autonomous DB (Connect via Oracle Wallet)
TNS_ADMIN=/etc/secrets/wallet_directory
DB_USER=ADMIN
DB_PASSWORD=YourPassword123
DB_TNS_NAME=sparc_atp_high

# Payments (Razorpay/Crypto)
RAZORPAY_KEY_ID=rzp_live_...
RAZORPAY_KEY_SECRET=...
CRYPTO_WALLET_ADDRESS=0x...
```
