# Boot Integrity

The current BIOS boot path does not verify signed bootloader or kernel images.
There is no trusted key store or signature-verification implementation.

The integrity APIs return an unavailable error for kernel and package
verification. UEFI Secure Boot and verified boot are not claimed.
