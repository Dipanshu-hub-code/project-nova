# Data Encryption

Encrypted storage is not implemented. The encryption APIs fail closed and do
not return ciphertext or plaintext. Nova currently has no persistent user-data
storage, key derivation, secure random salt generation, or key management.
