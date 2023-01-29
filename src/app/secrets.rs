use securestore::{KeySource, SecretsManager};
use std::{path::Path, collections::HashMap};
use once_cell::sync::Lazy;


// pub static SECRETS: Lazy<SecretsManager> = Lazy::new(|| {
//     let (store_path, key_path) = match std::env::var("MYWEBAPP_ENV").as_ref().map(|s| s.as_str()) {
//         Ok("STAGING") => ("secure/secrets.staging.json", Path::new("secure/secrets.staging.key")),
//         Ok("PRODUCTION") => ("secure/secrets.prod.json", Path::new("secure/secrets.prod.key")),
//         _ => ("secure/secrets.dev.json", Path::new("secure/secrets.dev.key")),
//     };

//     SecretsManager::load(store_path, KeySource::File(key_path))
//         .expect("Failed to load SecureStore vault!")
// });

static SECRETS: Lazy<SecretsManager> = Lazy::new(|| {
    let keyfile = Path::new("../secure/secrets.key");
    SecretsManager::load("../secure/secrets.json", KeySource::File(keyfile))
        .expect("Failed to load SecureStore vault!")
});

pub fn get_secret(secret_map: &mut HashMap<String, String>)
    -> Result<HashMap<&String, String>, securestore::Error> {
        let mut ret = HashMap::new();
        for (k, v) in secret_map.iter_mut() {
            ret.insert(k, SECRETS.get(k.as_str())?);
        }
    Ok(ret)
}
