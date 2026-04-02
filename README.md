# ⚡ Sparc Energy — Carbon Credit Market Platform

<div align="center">
  <img src="https://img.shields.io/badge/Rust-Backend-orange?logo=rust" />
  <img src="https://img.shields.io/badge/Frontend-GitHub%20Pages-blue?logo=github" />
  <img src="https://img.shields.io/badge/Database-PostgreSQL-blue?logo=postgresql" />
  <img src="https://img.shields.io/badge/Deploy-Render.com-maroon?logo=render" />
  <img src="https://img.shields.io/badge/Standard-Verra%20VCS%20%7C%20Gold%20Standard-gold" />
</div>

---

India's most trusted **Carbon Credit Marketplace** — inspired by Binance, Groww, and Verra. 
Trade verified carbon credits. Offset your CO₂. Go Net Zero.

## 🌟 Features

- 📈 **Live Trading** — Binance-style marketplace with real-time order book
- 🌿 **Verified Credits** — Verra VCS & Gold Standard certified
- 📊 **Portfolio Dashboard** — Groww/Angel One-style portfolio tracker
- 🏅 **CO₂ Retirement** — Retire credits and receive digital certificates
- 🔐 **JWT Authentication** — Secure login with role-based access
- 🌍 **Project Registry** — Browse reforestation, solar, wind, blue carbon projects
- ⚡ **Full Rust Backend** — Blazing fast Actix-Web API
- 🗃️ **PostgreSQL Database** — Robust & scalable relational database

## 🔗 Live URLs

- **Frontend**: https://sparcenergy.in/
- **Backend API**: https://sparc-energy.onrender.com/api

## 🚀 Quick Start (Local Development)

### Prerequisites
- [Rust](https://rustup.rs/) installed (check: `rustup --version`)
- [Git](https://git-scm.com/) installed

### Step 1: Run the Backend

```bash
cd backend
cargo run
```
Backend will start at: `http://localhost:8080`

### Step 2: Open the Frontend

Just open `frontend/index.html` in your browser. Or use Live Server in VS Code.

### Demo Accounts (pre-seeded)

| Role | Email | Password |
|------|-------|----------|
| Admin | admin@sparcenergy.com | Admin@123 |
| Demo Buyer | demo@sparcenergy.com | Demo@123 |

---

## 🌐 Deployment Guide

### Frontend → Vercel (PRO)

1. Connect your GitHub repository to [Vercel](https://vercel.com/).
2. Vercel will auto-detect the configuration from `vercel.json`.
3. Set your custom domain `www.sparcenergy.in` in Vercel settings.
4. Your site will be live at: `https://www.sparcenergy.in/`

### Backend → Render.com (PRO)

1. Create a "Web Service" on [Render](https://render.com/).
2. Select the `backend` directory as the root.
3. Configure Environment Variables:
   - `DATABASE_URL`: Your PostgreSQL connection string.
   - `JWT_SECRET`: A secure random string for signing tokens.
4. Render will build and deploy your Rust API automatically.

### Connect Frontend to Backend

Ensure `frontend/js/api.js` points to your Render URL:

```javascript
const API_BASE = 'https://sparc-energy.onrender.com/api';
```

---

## 📁 Project Structure

```
├── backend/                    ← Rust Actix-Web API
│   ├── Cargo.toml
│   ├── Shuttle.toml            ← Shuttle deployment config
│   ├── .env                    ← Environment variables (local)
│   ├── src/
│   │   ├── main.rs             ← Server entry point & routes
│   │   ├── db.rs               ← Database setup & seed data
│   │   ├── models.rs           ← All data structs
│   │   ├── auth.rs             ← JWT utilities
│   │   └── handlers/           ← Route handlers
│   │       ├── auth.rs         ← Login, register
│   │       ├── credits.rs      ← Buy/sell credits
│   │       ├── projects.rs     ← Project registry
│   │       ├── dashboard.rs    ← Portfolio & retirement
│   │       ├── market.rs       ← Stats & trades
│   │       └── admin.rs        ← Admin functions
│   └── migrations/
│       └── 001_init.sql        ← Database schema
│
├── frontend/                   ← Static HTML/CSS/JS
│   ├── index.html              ← Landing page
│   ├── marketplace.html        ← Trading platform
│   ├── projects.html           ← Project registry
│   ├── dashboard.html          ← User portfolio
│   ├── auth.html               ← Login / Register
│   ├── css/
│   │   └── styles.css          ← Premium dark design system
│   └── js/
│       ├── api.js              ← API client + mock data
│       ├── auth.js             ← Auth utilities
│       ├── marketplace.js      ← Trading UI logic
│       ├── projects.js         ← Projects page logic
│       └── dashboard.js        ← Dashboard & charts
│
└── .github/
    └── workflows/
        └── deploy.yml          ← Auto-deploy to GitHub Pages
```

---

## 🔌 API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/api/auth/register` | Register new user |
| POST | `/api/auth/login` | Login, get JWT token |
| GET | `/api/auth/me` | Get current user |
| GET | `/api/credits` | List carbon credits (filterable) |
| GET | `/api/credits/:id` | Single credit detail |
| GET | `/api/credits/:id/history` | Price history |
| POST | `/api/credits` | List new credits (seller) |
| POST | `/api/credits/buy` | Purchase credits |
| GET | `/api/projects` | List all projects |
| GET | `/api/projects/:id` | Project + linked credits |
| POST | `/api/projects` | Submit new project |
| GET | `/api/dashboard` | Portfolio + transactions |
| POST | `/api/dashboard/retire` | Retire credits |
| GET | `/api/market/stats` | Live market statistics |
| GET | `/api/market/trades` | Recent transactions |
| GET | `/api/market/leaderboard` | Top credit holders |
| GET | `/api/admin/users` | List all users (admin) |
| POST | `/api/admin/projects/:id/approve` | Verify project (admin) |

---

## 🏗 Technology Stack

| Layer | Technology |
|-------|-----------|
| Language | Rust (100%) |
| Backend | Actix-Web 4 |
| Database | PostgreSQL via SQLx |
| Authentication | JWT (jsonwebtoken) |
| Passwords | bcrypt |
| Frontend | HTML5 + CSS3 + Vanilla JS |
| Charts | Chart.js |
| Free Hosting (Backend) | Render.com |
| Storage (Assets) | Mega.nz |
| Infrastructure | Supabase BaaS (Auth/Realtime) |
| Free Hosting (Frontend) | Vercel |
| CI/CD | Vercel + Render Auto-deploy |

---

## 🌿 Carbon Credits Certified By

- 🌿 **Verra VCS** (Verified Carbon Standard) — World's leading carbon program
- 🥇 **Gold Standard** — Highest quality carbon + SDG impact
- 🌐 **CDM** — UN Clean Development Mechanism
- 🇮🇳 **BEE India** — Bureau of Energy Efficiency

---

## 📄 License

Copyright © 2024 **Sparc Energy Pvt. Ltd.** All rights reserved.

---

*Built with ⚡ by Sparc Energy — Powering India's Net Zero Future*
