# Update all
update:
    nix flake update
    cargo update

# Test with all features
test:
    cargo test --all-features

build__recp:
    nix build .#recp
