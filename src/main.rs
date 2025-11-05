use once_cell::sync::OnceCell;
use sshslc::Config;
use warp::Filter;

static CONFIG: OnceCell<sshslc::Config> = OnceCell::new();

#[tokio::main]
async fn main() -> Result<(), String> {
    match Config::load().await {
        Ok(config) => CONFIG.set(config).unwrap(),
        Err(err) => return Err(err.to_string()),
    };

    let hello = warp::path!(String / String).and_then(|host, key| async move {
        match sshslc::get_cert_for_host_key(CONFIG.get().unwrap(), host, key).await {
            Ok(cert) => Ok(cert),
            Err(err) => {
                eprintln!("{}", err.to_string());
                Err(warp::reject::not_found())
            }
        }
    });

    println!("Listening on: {}", CONFIG.get().unwrap().bind_addr);
    warp::serve(hello)
        .run(CONFIG.get().unwrap().bind_addr)
        .await;

    Ok(())
}
