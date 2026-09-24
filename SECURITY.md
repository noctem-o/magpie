# Security policy

Magpie is experimental. It has no production deployments that the project
supports, no stable release line, and no guaranteed response time.

## Supported versions

Only the current `main` branch receives security fixes. The `v0.1.0` and
`v0.2.0` source releases are historical and are not patched.

## Reporting a vulnerability

Please don't open a public issue or pull request for a suspected
vulnerability. That includes anything involving:

- signing keys or key files;
- a history that verifies but shouldn't: a forged, reordered, truncated, or
  tampered log accepted by any verifier (Rust, Python, or Go);
- a way to append, mutate, or delete history other than `LogWriter`, or for a
  projection to write;
- a way around the CLI's rollback checkpoint, locks, or file protections;
- credentials or other secrets exposed in the repository or its CI.

Report it privately through GitHub's **Report a vulnerability** button on this
repository's **Security** tab. If that button isn't shown, open an issue that
says only that you have a security report and asks for a private channel;
put no details in it.

Include the commit you tested, the steps or input that reproduce it, and what
you expected to be refused. Please give the maintainer a reasonable chance to
fix it before disclosing.

## Limitations that are not vulnerabilities

Magpie states what it does not establish, and those limits are design
boundaries, not bugs. A verified history is verified only against the
verifying key the caller supplies; Magpie does not establish who owns that
key, whether the history is current or complete, or that a claim is true.
Standing is whatever the selected policy computes. Anyone who can read a
store's `signing.key` can sign as its owner. The README's
[current boundary](README.md#current-boundary) lists the rest.

Reports that one of these documented limits exists are welcome as ordinary
issues. A report that Magpie claims more than it establishes, or that code
crosses one of these boundaries, is a security report.
