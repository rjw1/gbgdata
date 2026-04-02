# Production Docker Build & Push Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Automate building and pushing the production Docker image to GHCR on pushes to `main`.

**Architecture:** A standalone GitHub Actions workflow using standard Docker actions (`login-action`, `metadata-action`, `build-push-action`) with GITHUB_TOKEN authentication.

**Tech Stack:** GitHub Actions, Docker, GitHub Container Registry (GHCR).

---

### Task 1: Create Docker Publish Workflow

**Files:**
- Create: `.github/workflows/docker-publish.yml`

- [ ] **Step 1: Create the workflow file**

```yaml
name: Docker Publish

on:
  push:
    branches: [ "main" ]

env:
  REGISTRY: ghcr.io
  IMAGE_NAME: ${{ github.repository }}

jobs:
  build-and-push:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      packages: write

    steps:
      - name: Checkout repository
        uses: actions/checkout@v4

      - name: Log into registry ${{ env.REGISTRY }}
        uses: docker/login-action@v3
        with:
          registry: ${{ env.REGISTRY }}
          username: ${{ github.actor }}
          password: ${{ secrets.GITHUB_TOKEN }}

      - name: Extract Docker metadata
        id: meta
        uses: docker/metadata-action@v5
        with:
          images: ${{ env.REGISTRY }}/${{ env.IMAGE_NAME }}
          tags: |
            type=raw,value=latest
            type=sha,prefix=sha-

      - name: Set up Docker Buildx
        uses: docker/setup-buildx-action@v3

      - name: Build and push Docker image
        uses: docker/build-push-action@v5
        with:
          context: .
          file: web-app/Dockerfile
          push: true
          tags: ${{ steps.meta.outputs.tags }}
          labels: ${{ steps.meta.outputs.labels }}
          cache-from: type=gha
          cache-to: type=gha,mode=max
```

- [ ] **Step 2: Verify the workflow syntax**

Run: `actionlint .github/workflows/docker-publish.yml` (if installed) or manually review the YAML structure.

- [ ] **Step 3: Commit the new workflow**

```bash
git add .github/workflows/docker-publish.yml
git commit -m "feat: add github action for docker build and push to ghcr"
```

### Task 2: Verification (Manual)

- [ ] **Step 1: Check the GitHub Actions tab**

After pushing to `main`, verify that the "Docker Publish" workflow is triggered.

- [ ] **Step 2: Verify the image in GHCR**

Go to the repository's "Packages" section on GitHub and confirm the image exists with `latest` and `sha-` tags.
