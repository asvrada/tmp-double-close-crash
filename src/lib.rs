#[cfg(test)]
mod tests {
    use windows::Win32::Security::Cryptography::*;
    use windows::core::{w, HRESULT};
    use winapi::shared::winerror::NTE_INVALID_HANDLE;

    // Close a already closed provider
    #[test]
    fn test_double_close_provider() {
        let mut mcr_provider: NCRYPT_PROV_HANDLE = NCRYPT_PROV_HANDLE(0);

        unsafe {
            let result = NCryptOpenStorageProvider(&mut mcr_provider, w!("Microsoft Software Key Storage Provider"), 0);
            assert!(result.is_ok());

            let result = NCryptFreeObject(mcr_provider);
            assert!(result.is_ok());

            let result: Result<(), windows::core::Error> = NCryptFreeObject(mcr_provider);
            assert!(result.is_err());
            assert_eq!(result.unwrap_err().code(), HRESULT(NTE_INVALID_HANDLE));
        }
    }
}
