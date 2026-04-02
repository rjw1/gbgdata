# Design Spec: Production Docker Build & Push Workflow

This document outlines the design for a new GitHub Actions workflow to automate building and pushing the GBG Data Explorer production Docker image to the GitHub Container Registry (GHCR).

## 1. Objective
Automate the production release process by building a Docker image on every push to the `main` branch and publishing it to GHCR with consistent tagging.

## 2. Approach
We will implement a standalone GitHub Actions workflow (`docker-publish.yml`) that focuses on the build and push process. This separation ensures that production-related deployment logic is isolated from general CI (linting/testing).

- **Registry**: GitHub Container Registry (`ghcr.io`).
- **Image Name**: Derived from the repository name (e.g., `ghcr.io/rjw1/gbgdata`).
- **Tagging Strategy**: 
    - `latest`: For the most recent build on `main`.
    - `sha-<commit-hash>`: For traceability and version pinning.

## 3. Workflow Details

### 3.1 Trigger
- **Event**: `push`
- **Branch**: `main`

### 3.2 Permissions
The workflow requires the following GITHUB_TOKEN permissions:
- `contents: read`: To checkout the source code.
- `packages: write`: To authenticate and push images to GHCR.

### 3.3 Steps
1. **Checkout**: Retrieve the repository content.
2. **Log in to GHCR**: Use `docker/login-action` with the automated `${{ github.token }}`.
3. **Extract Metadata**: Use `docker/metadata-action` to generate the `latest` and `sha-` tags based on the git state.
4. **Set up Buildx**: Prepare the Docker Buildx environment for advanced features like caching.
5. **Build and Push**: 
    - **Context**: The repository root (to allow access to `migrations/` as required by the `web-app/Dockerfile`).
    - **File**: `web-app/Dockerfile`.
    - **Caching**: Use the `gha` (GitHub Actions) cache type to optimize build performance for Rust dependencies.

## 4. Testing & Verification
- **Manual Trigger**: Verify that pushing to a test branch (if temporarily modified) or `main` initiates the workflow.
- **Registry Check**: Confirm the image appears in the repository's "Packages" section on GitHub with the expected tags.
- **Pull Test**: Verify that the image can be pulled locally using `docker pull ghcr.io/<owner>/gbgdata:latest`.

## 5. Security Considerations
- **No Secrets**: Use only the built-in `${{ github.token }}` for GHCR access.
- **Provenance**: Enable Docker metadata labels to link the image back to the specific commit and workflow run.
