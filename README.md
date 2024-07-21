# Serene

## Setup Pre-Commit Hooks

```shell
cd .git/hooks
ln -s ../../pre-commit pre-commit
```

```shell
brew install pre-commit
pre-commit install
pre-commit run --all-files  # Runs pre-commit hooks manually
```
