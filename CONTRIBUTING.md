# Contributing to Hoolia Invest

## Getting started

If you want to help but don't know where to start, here are some
low-risk/isolated tasks:

- Try a [complexity:low] issue.
- check out [resources](#Developer-guidelines)

## Reporting issues

- [Search existing issues][github-issues] (including closed!)

## Developer guidelines

Read the following resources to learn more about Hoolia Invest:

- [comment styles](https://hoolia.github.io/manuals/developer_manual/#comments)
- [Version Control and Collaboration](https://hoolia.github.io/manuals/developer_manual/#version-control-and-collaboration)
- pull the latest version to see if your problem persists.

## Pull requests (PRs)

- To avoid duplicate work, create a draft pull request.
- Your PR must include [test coverage][run-tests].
- Avoid cosmetic changes to unrelated files in the same commit.
- Use a **feature branch** instead of the master branch.
  ```bash
  git checkout -b feature/my-feature
  ```
- Use a **hotfix branch** for critical bug fixes.
  ```bash
  git checkout -b hotfix/my-feature
  ```
- Merge the latest develop into your branch before submitting a PR.
  ```bash
  git checkout develop
  git pull
  git checkout feature/my-feature
  git merge develop
  ```
- Open a PR against the `develop` branch.

## Merging to master

For maintainers: when a PR is ready to merge to master,

- prefer Squash Merge for "single-commit PRs" (when the PR has only one
  meaningful commit).
- prefer Merge for "multi-commit PRs" (when the PR has multiple meaningful
  commits).

## Commit messages

Follow the
[Conventional Commits](https://hoolia.github.io/manuals/developer_manual/#workflow)
specification to make reviews easier and to make the git logs more valuable. The
general structure of a commit message is:

```
<type>([optional scope]): <description>

[optional body]

[optional footer(s)]
```

## Coding

### Lint

You can run the linter locally by:

```bash
pnpm lint
```
