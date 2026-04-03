import { test, expect } from "@playwright/test";

test.describe("Private Mode", () => {
  // These tests verify that when Private Mode is active, the site is properly protected.
  // The server MUST be started with PRIVATE_MODE=true for these tests to be meaningful.

  test("homepage redirects to login when unauthenticated", async ({ page }) => {
    await page.goto("/");
    // Should redirect to /login
    await expect(page).toHaveURL(/\/login/);
  });

  test("explore page redirects to login when unauthenticated", async ({ page }) => {
    await page.goto("/explore");
    await expect(page).toHaveURL(/\/login/);
  });

  test("API calls return 401 when unauthenticated", async ({ request }) => {
    // Attempting to call a non-essential API should return 401
    // Note: We use POST because Leptos server functions are POST by default
    const response = await request.post("/api/GetPubs", {
        data: { query: "test", sort: null, open_only: null }
    });
    expect(response.status()).toBe(401);
  });

  test("essential API calls are still accessible when unauthenticated", async ({ request }) => {
    // GetSiteSettings MUST be allowed for the UI to function
    const settingsResp = await request.post("/api/GetSiteSettings", { data: {} });
    expect(settingsResp.status()).toBe(200);

    // Login MUST be allowed
    const loginResp = await request.post("/api/Login", {
      data: { username: "nonexistent", password: "wrong" }
    });
    // It should return 200 (Result::Ok(None)), not 401 Unauthorized from middleware
    expect(loginResp.status()).toBe(200);
    const body = await loginResp.json();
    // body should be { "Ok": null } or similar depending on Leptos serialization
    expect(body).toHaveProperty("Ok");
  });

  test("static assets are still accessible", async ({ request }) => {
    const robotsResp = await request.get("/robots.txt");
    expect(robotsResp.status()).toBe(200);
    
    const faviconResp = await request.get("/favicon.ico");
    expect(faviconResp.status()).toBe(200);
  });
});
