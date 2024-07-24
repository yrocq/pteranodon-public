pub struct Handle {
    pub account: String,
    pub domain: String,
}

impl Handle {
    pub fn new(account: String, domain: String) -> Handle {
        Handle {
            account,
            domain,
        }
    }

    pub fn from_string(handle: &String) -> Result<Handle, String> {
        let url = url::Url::parse(handle);

        match url {
            Ok(url) => {
                let path = url.path();
                let parts = path.split("/").collect::<Vec<&str>>();
                let domain = url.host_str().unwrap();

                if parts[1] == "users" {
                    let account = parts[2];

                    Ok(Handle {
                        account: account.to_string(),
                        domain: domain.to_string(),
                    })
                }
                else {
                    if parts.len() == 2 {
                        match parts[1].strip_prefix('@') {
                            Some(account) => {
                                Ok(Handle {
                                    account: account.to_string(),
                                    domain: domain.to_string(),
                                })
                            },
                            None => {
                                Err(format!("Invalid handle : {}", url.to_string()))
                            }
                        }
                    }
                    else {
                        Err("Invalid handle".to_string())
                    }
                }
            },
            Err(error) => {
                return Err(error.to_string());
            }
        }
    }

    pub fn get_account(&self) -> String {
        format!("{}@{}", &self.account, self.domain)
    }

    pub fn to_url(&self) -> String {
        format!("https://{}/users/{}", self.domain, self.account)
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]

    fn with_users_prefix() {
        let handle = Handle::from_string(&"https://quebec.social/users/jean".to_string()).unwrap();
        assert_eq!(handle.domain, "quebec.social");
        assert_eq!(handle.account, "jean");
    }

    #[test]

    fn with_arobase_prefix() {
        let handle = Handle::from_string(&"https://quebec.social/@jean".to_string()).unwrap();
        assert_eq!(handle.domain, "quebec.social");
        assert_eq!(handle.account, "jean");
    }

    #[test]

    fn to_url() {
        let handle = Handle::from_string(&"https://quebec.social/users/jean".to_string()).unwrap();
        assert_eq!(handle.to_url(), "https://quebec.social/users/jean");
    }          
}