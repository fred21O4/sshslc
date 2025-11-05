use serde::Deserialize;
use ssh_key::{PrivateKey, PublicKey, rand_core::OsRng};
use std::{
    env::{self, VarError},
    net::SocketAddr,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};
use thiserror::Error;
use tokio::io;
use tokio::{fs::File, io::AsyncReadExt};

#[derive(Error, Debug)]
pub enum Error {
    #[error("missing cert config file at path: {0}")]
    MissingCertConfig(PathBuf),
    #[error("io error: {0}")]
    IO(#[from] io::Error),
    #[error("failed to deserialize cert config: {0}")]
    DeserializeConfig(#[from] serde_json::Error),
    #[error("missing principles for cert config at {0}")]
    MissingPrinciples(PathBuf),
    #[error("ssh_key error: {0}")]
    SSHKey(#[from] ssh_key::Error),
    #[error("system time error: {0}")]
    Time(#[from] std::time::SystemTimeError),
    #[error("SSHSLC_PREFIX is unset or invalid: {0}")]
    InvalidPrefix(#[from] VarError),
    #[error("SSHSLC_PREFIX: {0} is missing")]
    MissingPrefix(PathBuf),
    #[error("config file missing at {0}")]
    MissingConfigFile(PathBuf),
    #[error("missing ca key at {0}")]
    MissingCAKey(PathBuf),
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(skip)]
    pub prefix: PathBuf,
    pub bind_addr: SocketAddr,
    pub host_key: PathBuf,
    pub user_key: PathBuf,
}

impl Config {
    pub async fn load() -> Result<Self, Error> {
        let prefix = env::var("SSHSLC_PREFIX")?;

        let prefix = PathBuf::from(prefix);

        if !prefix.exists() {
            return Err(Error::MissingPrefix(prefix));
        }

        let config_file_path = prefix.join("config.json");

        if !config_file_path.exists() {
            return Err(Error::MissingConfigFile(config_file_path));
        }

        let mut file = File::open(config_file_path).await?;
        let mut config_string = String::new();
        file.read_to_string(&mut config_string).await?;
        let mut config: Config = serde_json::from_str(&config_string)?;
        config.prefix = prefix;
        Ok(config)
    }
}

fn one_min() -> u64 {
    60
}

#[derive(Debug, Deserialize)]
enum CertType {
    User,
    Host,
}

#[derive(Debug, Deserialize)]
struct CertConfig {
    pub_key: String,
    cert_type: CertType,
    comment: Option<String>,
    key_id: Option<String>,
    #[serde(default = "one_min")] // Serde is annoying and doesnt let us just put 60 here
    valid_for: u64, // Length of time for each cert to be valid for in seconds defaults to 1min
    principles: Vec<String>,
}

pub async fn get_cert_for_host_key(
    config: &Config,
    host: String,
    key: String,
) -> Result<String, Error> {
    let path = config.prefix.join(host);

    let cert_config_path = path.join(key + ".json");
    if !cert_config_path.exists() {
        return Err(Error::MissingCertConfig(cert_config_path));
    }

    let mut cert_config_file = File::open(&cert_config_path).await?;
    let mut cert_config_string = String::new();
    cert_config_file
        .read_to_string(&mut cert_config_string)
        .await?;
    let cert_config: CertConfig = serde_json::from_str(&cert_config_string)?;

    let public_key = PublicKey::from_openssh(&cert_config.pub_key)?;

    let valid_after = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let valid_before = valid_after + cert_config.valid_for;
    let mut cert_builder = ssh_key::certificate::Builder::new_with_random_nonce(
        &mut OsRng,
        public_key,
        valid_after,
        valid_before,
    )?;

    if cert_config.principles.len() == 0 {
        return Err(Error::MissingPrinciples(cert_config_path));
    }

    for principle in cert_config.principles {
        cert_builder.valid_principal(principle)?;
    }

    match cert_config.cert_type {
        CertType::User => cert_builder.cert_type(ssh_key::certificate::CertType::User)?,
        CertType::Host => cert_builder.cert_type(ssh_key::certificate::CertType::Host)?,
    };

    if let Some(comment) = cert_config.comment {
        cert_builder.comment(comment)?;
    }

    if let Some(id) = cert_config.key_id {
        cert_builder.key_id(id)?;
    }

    let ca_key_path = match cert_config.cert_type {
        CertType::User => config.prefix.join(&config.user_key),
        CertType::Host => config.prefix.join(&config.host_key),
    };

    if !ca_key_path.exists() {
        return Err(Error::MissingCAKey(ca_key_path));
    }

    let mut ca_key_file = File::open(ca_key_path).await?;
    let mut ca_key_string = String::new();
    ca_key_file.read_to_string(&mut ca_key_string).await?;
    let ca_key = PrivateKey::from_openssh(&ca_key_string)?;

    let cert = cert_builder.sign(&ca_key)?;

    Ok(cert.to_openssh()?)
}
