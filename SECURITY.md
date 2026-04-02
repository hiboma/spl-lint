# Security Policy

## Supported Versions

| Version | Supported |
|---------|-----------|
| latest  | Yes       |

## Reporting a Vulnerability

If you discover a security vulnerability in spl-lint, please report it responsibly.

**Do NOT open a public issue.**

Instead, please use [GitHub Security Advisories](https://github.com/hiboma/spl-lint/security/advisories/new) to report the vulnerability privately.

You will receive a response within 7 days acknowledging receipt. A fix will be prioritized based on severity.

## Verifying Release Artifacts

Release binaries are signed using [Sigstore cosign](https://www.sigstore.dev/) (keyless signing with GitHub Actions OIDC).

To verify a release:

```bash
# Download the release files
# checksums-sha256.txt, checksums-sha256.txt.sig, checksums-sha256.txt.pem

# Verify the signature
cosign verify-blob checksums-sha256.txt \
  --signature checksums-sha256.txt.sig \
  --certificate checksums-sha256.txt.pem \
  --certificate-identity-regexp "https://github.com/hiboma/spl-lint" \
  --certificate-oidc-issuer "https://token.actions.githubusercontent.com"

# Verify the checksum of your downloaded binary
sha256sum -c checksums-sha256.txt
```

## Supply Chain Security

This project employs the following measures:

- **Pinned dependencies**: All GitHub Actions are pinned to specific commit SHAs
- **Dependency auditing**: `cargo-audit` runs on every CI build to check for known vulnerabilities
- **Automated updates**: Dependabot monitors Cargo and GitHub Actions dependencies weekly
- **Signed releases**: Release checksums are signed with Sigstore cosign (keyless OIDC)
- **Minimal dependencies**: The project maintains a small dependency tree to reduce attack surface
- **Lock file committed**: `Cargo.lock` is committed to ensure reproducible builds
