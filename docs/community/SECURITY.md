# Security Policy

## Supported Versions

| Version | Supported          | Security Updates |
| ------- | ------------------ | ---------------- |
| 0.1.x   | ✅ Yes            | Until 0.2.0     |
| < 0.1   | ❌ No             | N/A             |

## Reporting a Vulnerability

The leptos-next-metadata team takes security vulnerabilities seriously. We appreciate your efforts to responsibly disclose your findings.

### How to Report

**Please do not report security vulnerabilities through public GitHub issues.**

Instead, please report vulnerabilities by emailing: **security@leptos-next-metadata.dev**

Include the following information:
- Description of the vulnerability
- Steps to reproduce the issue
- Potential impact assessment
- Suggested fix (if any)

### What to Expect

1. **Acknowledgment**: We'll acknowledge receipt within 48 hours
2. **Investigation**: Initial assessment within 5 business days
3. **Updates**: Regular updates on our progress
4. **Resolution**: Timeline depends on severity and complexity
5. **Credit**: Public acknowledgment (if desired) after fix is released

### Response Timeline

| Severity | Response Time | Fix Timeline |
|----------|---------------|--------------|
| Critical | 24 hours     | 7 days       |
| High     | 48 hours     | 14 days      |
| Medium   | 5 days       | 30 days      |
| Low      | 10 days      | Next release |

## Security Best Practices

### For Users

1. **Keep Updated**
   ```toml
   [dependencies]
   leptos-next-metadata = "0.1"  # Always use latest patch version
   ```

2. **Review Dependencies**
   ```bash
   cargo audit  # Check for known vulnerabilities
   ```

3. **Secure Configuration**
   ```rust
   // Don't expose sensitive data in metadata
   metadata! {
       title: "Public Title",
       // ❌ Don't include secrets
       // description: format!("API Key: {}", api_key),
       
       // ✅ Use public information only
       description: "Public description"
   }
   ```

4. **Validate User Input**
   ```rust
   // ✅ Always validate and sanitize user input
   let safe_title = sanitize_html(&user_input);
   metadata! {
       title: safe_title,
   }
   ```

### For Contributors

1. **Input Validation**
   - All user input must be validated and sanitized
   - Use type-safe APIs to prevent injection attacks
   - Implement proper error handling

2. **Dependency Security**
   - Regularly audit dependencies with `cargo audit`
   - Minimize dependency surface area
   - Pin dependencies to specific versions in CI

3. **Template Security**
   - OG image templates must sanitize dynamic content
   - Prevent script injection in SVG templates
   - Validate template parameters

4. **File System Safety**
   - Prevent path traversal attacks in file conventions
   - Validate file paths and extensions
   - Use safe file system operations

## Known Security Considerations

### OG Image Generation

**Risk**: SVG injection attacks through template parameters

**Mitigation**: 
- All template parameters are escaped
- SVG parsing validates structure
- No external resource loading in templates

```rust
// ✅ Safe - parameters are automatically escaped
og_image! {
    template: "default",
    data: {
        "title": user_provided_title,  // Automatically sanitized
    }
}
```

### File System Operations

**Risk**: Path traversal in file convention scanning

**Mitigation**:
- All paths are validated against project root
- No symlink following outside project
- Whitelist-based file type validation

### Server-Side Rendering

**Risk**: Information disclosure through metadata

**Mitigation**:
- Clear separation of server/client metadata
- No sensitive data in client-accessible metadata
- Proper context isolation

### JSON-LD Generation

**Risk**: Script injection through structured data

**Mitigation**:
- All JSON-LD is properly escaped
- Schema validation prevents malicious data
- Type-safe builders prevent injection

## Vulnerability Categories

### High Priority
- Code execution vulnerabilities
- Path traversal attacks
- Injection vulnerabilities (XSS, script injection)
- Information disclosure of sensitive data

### Medium Priority
- Denial of service attacks
- Resource exhaustion
- Improper input validation
- Configuration vulnerabilities

### Low Priority
- Information leakage (non-sensitive)
- Performance degradation
- Cosmetic security issues

## Secure Development Practices

### Code Review
- All code changes require security review
- Focus on input validation and sanitization
- Review dependency changes for security implications

### Testing
- Include security test cases in test suite
- Fuzz testing for input validation
- Static analysis with clippy security lints

### CI/CD Security
- Automated vulnerability scanning
- Dependency audit in CI pipeline
- Security-focused linting rules

## Responsible Disclosure

We practice responsible disclosure:

1. **Private Notification**: Security issues are handled privately until fixed
2. **Coordinated Release**: Fixes are released in coordination with disclosure
3. **Public Advisory**: Security advisories published after fixes are available
4. **Credit**: Security researchers receive appropriate credit

## Contact

For security-related questions or concerns:
- **Email**: security@leptos-next-metadata.dev
- **PGP Key**: [Available on request]
- **GitHub**: @leptos-next-metadata-security (for non-sensitive coordination)

---

**Note**: This security policy is living document and may be updated as the project evolves.