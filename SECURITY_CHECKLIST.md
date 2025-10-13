# Security Checklist - Before Making Repository Public

Use this checklist before making the repository public.

## ✅ Pre-Public Checklist

### 1. Code Review

- [x] No hardcoded API keys or secrets in code
- [x] `.env` files in `.gitignore`
- [x] API keys stored locally (user-provided)
- [x] No personal information in commit history
- [ ] Review git history for accidentally committed secrets

### 2. GitHub Repository Settings

#### Branch Protection (Settings → Branches)

- [ ] Add branch protection rule for `main`:
  - [ ] Require pull request reviews (1 approval minimum)
  - [ ] Require status checks: `Build Frontend`, `Check Rust Code`, `Lint Code`
  - [ ] Require branches to be up to date
  - [ ] Do not allow bypassing rules (even for admins)
  - [ ] Restrict who can push to matching branches

#### Tag Protection (Settings → Tags)

- [ ] Add protected tag rule:
  - Pattern: `v*`
  - [ ] Only allow repository admins to create matching tags

#### Security & Analysis (Settings → Security & analysis)

- [ ] Enable **Dependency graph**
- [ ] Enable **Dependabot alerts**
- [ ] Enable **Dependabot security updates**
- [ ] Enable **Secret scanning**
- [ ] Enable **Code scanning** (CodeQL)

#### General Settings (Settings → General)

- [ ] Set repository visibility to **Public**
- [ ] Enable **Issues** (for bug reports)
- [ ] Enable **Discussions** (optional - for community)
- [ ] Disable **Wiki** (unless needed)
- [ ] Disable **Projects** (unless needed)
- [ ] Enable **Automatically delete head branches**
- [ ] Configure merge options:
  - [ ] Allow squash merging (recommended)
  - [ ] Allow rebase merging (recommended)
  - [ ] Disable merge commits (optional)

### 3. Documentation Review

- [ ] README has clear installation instructions
- [ ] LICENSE file is present and correct
- [ ] CONTRIBUTING.md explains contribution process
- [ ] Code of Conduct (optional but recommended)
- [ ] Security policy (SECURITY.md - optional)

### 4. GitHub Actions

- [ ] Workflows tested and passing
- [ ] No secrets exposed in workflow files
- [ ] GITHUB_TOKEN permissions are minimal
- [ ] Dependabot can create PRs for security updates

### 5. Final Security Scan

Run these commands locally:

```bash
# Check for accidentally committed secrets
git log --all --full-history --source -- .env

# Search for potential API keys in commit history
git log -p | grep -i "api.key\|secret\|password" | head -20

# Verify .gitignore is working
git status --ignored
```

### 6. Post-Public Actions

After making the repository public:

- [ ] Monitor first few issues/PRs for spam
- [ ] Set up email notifications for security alerts
- [ ] Pin important issues (installation guide, known issues)
- [ ] Add topics/tags to repository for discoverability
- [ ] Consider adding to Awesome lists if applicable

## 🔒 GitHub Settings Quick Links

Once public, configure these settings:

1. **Branch Protection**: `https://github.com/beneccles/crossover-mod-manager/settings/branches`
2. **Tag Protection**: `https://github.com/beneccles/crossover-mod-manager/settings/tag_protection`
3. **Security & Analysis**: `https://github.com/beneccles/crossover-mod-manager/settings/security_analysis`
4. **Manage Access**: `https://github.com/beneccles/crossover-mod-manager/settings/access`

## 🚨 Red Flags to Check Before Public

### Critical - Must Fix:

- ❌ Hardcoded API keys or credentials
- ❌ Passwords or tokens in code
- ❌ Private company information
- ❌ Personal contact details
- ❌ Production database URLs

### Warning - Should Review:

- ⚠️ TODO comments with sensitive context
- ⚠️ Debug endpoints or test credentials
- ⚠️ Internal system architecture details
- ⚠️ Unfinished/broken features in main branch

## ✅ Your Current Status

### Already Secure:

✅ API keys are user-provided (stored locally)
✅ `.env` files properly ignored
✅ No hardcoded secrets found
✅ CI/CD workflows use GitHub's built-in tokens
✅ License file present (MIT)

### Needs Action:

- [ ] Enable branch protection rules
- [ ] Enable tag protection
- [ ] Enable security features (Dependabot, CodeQL)
- [ ] Review git history for accidental commits

## 📋 Quick Command to Verify

Run this to check for common issues:

```bash
# Check git history for potential secrets
git log --all --oneline | head -20
git log -p -S "password" --all
git log -p -S "api_key" --all | grep -v "api_key:" | head -20

# Verify no .env files are tracked
git ls-files | grep "\.env$"

# Check for large files that shouldn't be there
git rev-list --objects --all | \
  git cat-file --batch-check='%(objecttype) %(objectname) %(objectsize) %(rest)' | \
  awk '/^blob/ {print substr($0,6)}' | sort -n -k 2 | tail -10
```

## 🎯 Recommended Order

1. **First**: Review code and git history for secrets
2. **Second**: Enable all security features in GitHub
3. **Third**: Set up branch and tag protection
4. **Fourth**: Make repository public
5. **Fifth**: Monitor for first 24 hours

## 📚 Additional Resources

- [GitHub Security Best Practices](https://docs.github.com/en/code-security)
- [Open Source Security Guide](https://opensource.guide/best-practices/)
- [OWASP Secure Coding Practices](https://owasp.org/www-project-secure-coding-practices-quick-reference-guide/)

---

**Last Updated**: October 13, 2025
**Repository**: crossover-mod-manager
**Status**: ⏳ Preparing for public release
