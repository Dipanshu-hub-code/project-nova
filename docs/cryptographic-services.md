# Cryptographic Services

Cryptography is unavailable. The project does not implement a random number
generator, password hash, constant-time primitive, key derivation, or
signature-verification algorithm. Requests to use the current crypto,
encryption, or integrity interfaces return explicit unavailable errors.

Future cryptographic functionality must use maintained, audited libraries and
a verified entropy source. Do not store or compare raw passwords and do not
invent custom cryptographic algorithms.
