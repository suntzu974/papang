use gloo_net::http::{Request, RequestBuilder};

pub struct ApiService;

impl ApiService {
    pub fn backend_url() -> String {
        option_env!("BACKEND_URL").unwrap_or("http://localhost:3001").to_string()
    }

    pub fn url(path: &str) -> String {
        let base = Self::backend_url();
        if path.starts_with('/') {
            format!("{}{}", base, path)
        } else {
            format!("{}/{}", base, path)
        }
    }

    pub fn get(path: &str) -> RequestBuilder {
        Request::get(&Self::url(path))
    }

    pub fn post(path: &str) -> RequestBuilder {
        Request::post(&Self::url(path))
    }

    pub fn put(path: &str) -> RequestBuilder {
        Request::put(&Self::url(path))
    }

    pub fn delete(path: &str) -> RequestBuilder {
        Request::delete(&Self::url(path))
    }
}
