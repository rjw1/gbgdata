# Improved Stats Display Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement a visual stats dashboard with circular progress rings and a hero streak counter.

**Architecture:** Create a reusable `StatRing` component and update `PubDetail` to use a flexbox-based dashboard layout.

**Tech Stack:** Rust, Leptos, SVG, SCSS.

---

## File Structure
- `web-app/src/components/stat_ring.rs`: New reusable SVG ring component.
- `web-app/src/components/mod.rs`: Register the new component.
- `web-app/src/components/pub_detail.rs`: Update to use the new dashboard layout.
- `web-app/style/main.scss`: Add styles for the dashboard and rings.

## Tasks

### Task 1: StatRing Component
- [ ] **Step 1: Create StatRing component**
File: `web-app/src/components/stat_ring.rs`
```rust
use leptos::prelude::*;

#[component]
pub fn StatRing(
    value: i64,
    max: i64,
    label: String,
) -> impl IntoView {
    let radius = 40.0;
    let circumference = 2.0 * std::f64::consts::PI * radius;
    let offset = circumference - (value as f64 / max as f64) * circumference;

    view! {
        <div class="stat-ring-container">
            <svg width="100" height="100" viewBox="0 0 100 100">
                <circle
                    cx="50" cy="50" r=radius
                    fill="transparent"
                    stroke="#e9ecef"
                    stroke-width="8"
                />
                <circle
                    cx="50" cy="50" r=radius
                    fill="transparent"
                    stroke="var(--amber)"
                    stroke-width="8"
                    stroke-dasharray=circumference
                    stroke-dashoffset=offset
                    stroke-linecap="round"
                    transform="rotate(-90 50 50)"
                />
                <text
                    x="50" y="55"
                    text-anchor="middle"
                    font-size="18"
                    font-weight="bold"
                    fill="var(--forest-green)"
                >
                    {value} "/" {max}
                </text>
            </svg>
            <span class="stat-label">{label}</span>
        </div>
    }
}
```
- [ ] **Step 2: Register module**
Modify: `web-app/src/components/mod.rs`
```rust
pub mod pub_list;
pub mod pub_detail;
pub mod near_me;
pub mod stat_ring;
```
- [ ] **Step 3: Commit**
```bash
git add web-app/src/components/stat_ring.rs web-app/src/components/mod.rs
git commit -m "feat: implement StatRing SVG component"
```

### Task 2: Dashboard Styles
- [ ] **Step 1: Update styles**
File: `web-app/style/main.scss`
```scss
.stats-dashboard {
  display: flex;
  justify-content: space-around;
  align-items: center;
  flex-wrap: wrap;
  gap: 2rem;
  padding: 2rem;
  background: white;
  border-radius: 16px;
  box-shadow: 0 4px 12px rgba(0,0,0,0.08);
  margin: 2rem 0;

  @media (max-width: 600px) {
    flex-direction: column;
  }
}

.hero-streak {
  text-align: center;
  
  .streak-number {
    display: block;
    font-size: 4rem;
    font-weight: 900;
    color: var(--amber);
    line-height: 1;
  }
  
  .streak-label {
    font-size: 1.1rem;
    font-weight: bold;
    color: var(--forest-green);
    text-transform: uppercase;
    letter-spacing: 1px;
  }
}

.stat-ring-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.5rem;

  .stat-label {
    font-weight: bold;
    color: #6c757d;
  }
}
```
- [ ] **Step 2: Commit**
```bash
git add web-app/style/main.scss
git commit -m "feat: add stats dashboard and ring styles"
```

### Task 3: PubDetail Integration
- [ ] **Step 1: Update PubDetail to use StatRing and Dashboard**
Modify: `web-app/src/components/pub_detail.rs`
Import `StatRing` and replace the existing `.stats-card` content with the new `.stats-dashboard`.
- [ ] **Step 2: Commit**
```bash
git add web-app/src/components/pub_detail.rs
git commit -m "feat: integrate visual stats dashboard into pub detail page"
```
