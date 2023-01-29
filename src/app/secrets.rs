use securestore::{KeySource, SecretsManager};
use std::{path::Path, collections::HashMap};
use once_cell::sync::Lazy;


pub static SECRETS: Lazy<SecretsManager> = Lazy::new(|| {
    let (store_path, key_path) = match std::env::var("ENV").as_ref().map(|s| s.as_str()) {
        Ok("DEV") => ("../secure/secrets.dev.json", Path::new("../secure/secrets.dev.key")),
        Ok("STG") => ("../secure/secrets.stg.json", Path::new("../secure/secrets.stg.key")),
        Ok("PRD") => ("../secure/secrets.prod.json", Path::new("../secure/secrets.prod.key")),
        _ => ("../secure/secrets.dev.json", Path::new("../secure/secrets.dev.key")),
    };

    SecretsManager::load(store_path, KeySource::File(key_path))
        .expect("Failed to load SecureStore vault!")
});

pub fn get_secret(secret_map: &mut HashMap<String, String>)
    -> Result<HashMap<&String, String>, securestore::Error> {
        let mut ret = HashMap::new();
        for (k, _) in secret_map.iter_mut() {
            ret.insert(k, SECRETS.get(k.as_str())?);
        }
    Ok(ret)
}
