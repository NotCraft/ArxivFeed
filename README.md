[![Crates.io](https://img.shields.io/crates/v/arxivfeed.svg)](https://crates.io/crates/arxivfeed)
[![license](https://img.shields.io/github/license/notcraft/arxivfeed.svg?maxAge=86400)](LICENSE)
![GitHub Workflow Status](https://img.shields.io/github/workflow/status/notcraft/arxivfeed/CICD)

# [NotCraft::ArxivFeed](https://notcraft.alongwy.top/ArxivFeed/)

An Arxiv reader running entirely from your GitHub repo.

- Free hosting on [GitHub Pages](https://pages.github.com/). No ads. No third party tracking.
- No need for backend. Content updates via [GitHub Actions](https://github.com/features/actions).
- Customizable layouts and styles via templating and theming API. Just bring your HTML and CSS.
- Free and open source. No third-party tracking.

## How to use it?

### Github Pages

1. Use the [ArxivFeed-Template](https://github.com/NotCraft/ArxivFeed-Template) generate your own repository.
2. In the repository root, open `config.toml` file, click the "Pencil (Edit this file)" button to edit.
3. Remove `# ` to uncommend the `cacheUrl` property, replace `<github_username>` with your GitHub username, and
   replace `<repo>` with your GitHub repo name.
4. In the sources, update the items to the sources you want to follow. The final content of the file should look similar
   to this:

   ```toml
   site_title = "ArxivDaily"
   limit_days = 7
   # statics_dir   = "statics"       ## Optional: default is "statics"
   # templates_dir = "includes"      ## Optional: default is "includes"
   # cache_url = "https://GITHUB_USERNAME.github.io/REPO_NAME/cache.json"

   [[sources]]
   limit = 1                               # Num Limit
   category = "cs.CL"                      # Subject Category
   title = "Computation and Language"      # Subject Title

   # [scripts]
   # highlight = "scripts/highlight.rhai"
   ```

5. Scroll to the bottom of the page, click "Commit changes" button.
6. Once the rebuild finishes, your feed will be available at `https://<github_username>.github.io/<repo>`

### Localhost

1. Clone the [ArxivFeed-Template](https://github.com/NotCraft/NotFeed-Template) repository.
2. Edit `config.toml` file.
3. Run `arxivfeed`
