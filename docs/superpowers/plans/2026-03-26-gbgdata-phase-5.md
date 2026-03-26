# gbgdata Phase 5: PWA & Polish Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Transform the application into a full Progressive Web App (PWA) with offline capabilities and refined styling.

**Architecture:** Add a `manifest.json` and a Service Worker for resource caching. Refine CSS for mobile-first responsiveness.

**Tech Stack:** Leptos, Vanilla JS (Service Worker), PWA standards.

---

## File Structure
- `web-app/public/manifest.json`: Web app manifest.
- `web-app/public/service-worker.js`: Service worker for offline caching.
- `web-app/src/app.rs`: Register service worker and manifest in shell.
- `web-app/style/main.scss`: Final CSS polish.

## Tasks

### Task 1: PWA Manifest & Assets
- [ ] **Step 1: Create manifest.json**
File: `web-app/public/manifest.json`
- [ ] **Step 2: Add placeholder icons (SVGs or simple PNGs)**
- [ ] **Step 3: Commit**
```bash
git add web-app/public/
git commit -m "feat: add pwa manifest and assets"
```

### Task 2: Service Worker
- [ ] **Step 1: Create service-worker.js**
File: `web-app/public/service-worker.js`
Implement basic "Cache-First" or "Stale-While-Revalidate" strategy for static assets.
- [ ] **Step 2: Register Service Worker in app.rs**
- [ ] **Step 3: Commit**
```bash
git add web-app/public/service-worker.js web-app/src/app.rs
git commit -m "feat: implement service worker for offline caching"
```

### Task 3: Mobile-First Polish
- [ ] **Step 1: Refine SCSS for mobile responsiveness**
Adjust grid layouts and padding for smaller screens.
- [ ] **Step 2: Add "Add to Home Screen" instructions/hint**
- [ ] **Step 3: Commit**
```bash
git add web-app/style/main.scss
git commit -m "feat: refine mobile-first styling and responsiveness"
```

### Task 4: Final Cleanup
- [ ] **Step 1: Remove unused boilerplate**
- [ ] **Step 2: Final build check**
- [ ] **Step 3: Commit**
```bash
git commit -m "chore: final cleanup and project polish"
```
