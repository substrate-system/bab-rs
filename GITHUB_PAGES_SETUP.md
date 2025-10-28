# GitHub Pages Setup

This repository includes a GitHub Action that automatically deploys the WASM demo to GitHub Pages.

## One-Time Setup

1. Go to your repository on GitHub
2. Navigate to **Settings** → **Pages**
3. Under **Source**, select **GitHub Actions**
4. Save the settings

That's it! The workflow will automatically deploy on every push to `main` or when you create a new tag.

## Manual Deployment

You can also trigger a deployment manually:

1. Go to **Actions** tab in your repository
2. Select **Deploy Demo to GitHub Pages** workflow
3. Click **Run workflow**
4. Select the branch and click **Run workflow**

## After Deployment

Your demo will be available at:
```
https://<username>.github.io/<repository-name>/
```

For this repository:
```
https://substrate-system.github.io/bab-rs/
```

## Deployment Triggers

The workflow deploys automatically when:
- You push to the `main` branch
- You create a new version tag (e.g., `v0.4.4`)
- You manually trigger it from the Actions tab

## What Gets Deployed

The GitHub Action runs `npm run build-example` which:
1. Builds the WASM module with wasm-pack
2. Builds the Vite demo app from the `example/` directory
3. Outputs everything to the `public/` directory with base path `/bab-rs`

The deployed site includes:
- Built Vite app with optimized assets
- WASM module and JavaScript bindings
- CSS styles

## Troubleshooting

If the deployment fails:
1. Check the Actions tab for error logs
2. Ensure GitHub Pages is enabled in repository settings
3. Verify the workflow has the correct permissions
