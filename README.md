# Kustomize pre-commit hook

[Pre-commit](https://pre-commit.com/) hook for building [kustomization](https://kustomize.io/) files to make sure no faulty files are committed.

## Dev:

`pre-commit try-repo ../kustomize-precommit-hook kustomize --verbose --all-files`

## TODO:

- [ ] Asynchronous runs for larger repo's / with remote refs
- [ ] Nicer output/reporting
