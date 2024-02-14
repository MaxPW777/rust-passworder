#[cfg(test)]
mod password_manipulating {
    use crate::models::{Credential, PasswordManager};

    // Function to create a test instance of PasswordManager with predefined credentials
    fn setup_password_manager() -> PasswordManager {
        PasswordManager {
            credentials: vec![
                Credential {
                    encrypted_password: "sheesh".to_string(),
                    service_name: "linux".to_string(),
                },
                Credential {
                    encrypted_password: "mac<windows".to_string(),
                    service_name: "mac".to_string(),
                },
                Credential {
                    encrypted_password: "windowsiscool".to_string(),
                    service_name: "windows".to_string(),
                },
                Credential {
                    encrypted_password: "tf2 best game".to_string(),
                    service_name: "steam".to_string(),
                },
                Credential {
                    encrypted_password: "12345".to_string(),
                    service_name: "iphone".to_string(),
                },
            ],
        }
    }

    #[test]
    fn test_delete_existing_credentials() {
        let mut password_manager = setup_password_manager();
        let service_name = "linux"; // This is &str

        // Assert that credentials for the service exist before deletion
        assert!(password_manager
            .credentials
            .iter()
            .any(|c| c.service_name == service_name));

        let result = password_manager.delete_credentials(service_name);
        assert!(result.is_ok());

        // Assert that no credentials for the service exist after deletion
        assert!(!password_manager
            .credentials
            .iter()
            .any(|c| c.service_name == service_name));
    }

    #[test]
    fn test_delete_non_existing_credentials() {
        let mut password_manager = setup_password_manager();
        let service_name = "non_existing_service"; // This is &str

        // Assert that no credentials for the service exist before deletion
        assert!(!password_manager
            .credentials
            .iter()
            .any(|c| c.service_name == service_name));

        let result = password_manager.delete_credentials(service_name);
        assert!(result.is_err());

        // The total number of credentials should remain unchanged
        assert_eq!(password_manager.credentials.len(), 5);
    }

    #[test]
    fn test_new_password() {
        let mut password_manager = setup_password_manager();
        let service_name = String::from("test");
        password_manager.add_credentials(Credential {
            service_name: service_name.clone(),
            encrypted_password: "test".to_string(),
        });
        assert!(password_manager.get_credentials(&service_name).is_some())
    }

    #[test]
    fn test_remove_password() {
        let mut password_manager = setup_password_manager();
        let service_name = String::from("steam");
        let _ = password_manager.delete_credentials(&service_name);
        assert!(password_manager.get_credentials(&service_name).is_none())
    }
}
