# 🌹 Valentine Quest: Aspirant Edition

> **"Code is easy. Romance is a merge conflict."**

A high-performance, full-stack Rust implementation of the viral Valentine's Week coding quest. This version is designed specifically for **Aspirants**—tailoring challenges to Techies, MBA candidates, UPSC warriors, JEE dreamers, and CA professionals.

![Rust](https://img.shields.io/badge/built%20with-Rust-black?style=for-the-badge&logo=rust)
![Yew](https://img.shields.io/badge/Framework-Yew%20(WASM)-662d91?style=for-the-badge&logo=webassembly)
![Tailwind](https://img.shields.io/badge/UI-Tailwind%20CSS-38B2AC?style=for-the-badge&logo=tailwind-css)

---

## 🎨 The Aesthetic: "Romantic-Tech"
The app features a unique **Deep Wine & Rose Gold** palette. It combines the structured, windowed feel of a terminal with the warmth of Valentine's Day.

- **Theme**: Burgundy Background (`#1a0a0d`), Rose Containers (`#2d0f14`), and Vibrant Red Highlights (`#ff4d6d`).
- **UI**: Responsive terminal windows, neon glows, and interactive typing inputs.

---

## 🎭 The 5 Personas
This isn't just a coding game. It’s a quest for every type of high-achiever:

1. **💻 The Techie**: Solve Git merges, SQL queries, and Rust logic.
2. **📈 MBA Aspirant**: Navigate Marketing 4Ps, Finance ratios, and SWOT analysis.
3. **🏛️ UPSC Aspirant**: Recall Constitutional Articles, History milestones, and Geography.
4. **⚛️ JEE Aspirant**: Balance Organic Chemistry, Kinematics, and Calculus.
5. **📊 CA Aspirant**: Audit the books, calculate GST, and balance the ledger.

---

## 🛠️ Tech Stack
- **Frontend**: [Yew.rs](https://yew.rs/) (Rust framework for WASM).
- **Routing**: `yew-router` for multi-persona navigation.
- **Persistence**: `localStorage` via `gloo-storage` (progress is saved per-persona).
- **Styling**: Tailwind CSS via CDN.
- **Build Tool**: [Trunk](https://trunkrs.dev/).

---

## 📦 Folder Structure
```text
.
├── src/
│   └── main.rs         # The entire app logic (Routing, UI, Challenge Data)
├── index.html          # Global styles, Tailwind CDN, and WASM entry
└── Cargo.toml          # Rust Dependencies
