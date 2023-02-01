use crate::requests;
use crate::responses;
use aes_gcm::{
    aead::{generic_array::GenericArray, Aead},
    Aes256Gcm, KeyInit,
};
//use getrandom::getrandom;
use hex;
use reqwest::header;
use serde::{Deserialize, Serialize};
use std::error::Error;

pub async fn get_replays() -> Result<Vec<responses::Replay>, String> {
    let token = std::fs::read_to_string("token.txt").unwrap();
    let mut replays = Vec::new();
    for i in 0..10 {
        let request_data = requests::generate_replay_request(i, 127, &token);
        let request_data = encrypt_data(&request_data);
        let lol = request_data.clone();
        let client = reqwest::Client::new();
        let form = client
            .post("https://ggst-game.guiltygear.com/api/catalog/get_replay")
            .header(header::USER_AGENT, "GGST/Steam")
            .header(header::CACHE_CONTROL, "no-store")
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .header("x-client-version", "1")
            .form(&[("data", request_data)]);
        println!("{:?}", &form);

        let response = form.send().await.unwrap();
        let response_bytes = response.bytes().await.unwrap();
        if let Ok(r) = decrypt_response::<responses::Replays>(&response_bytes) {
            replays.extend_from_slice(&r.replays);
        }
    }

    Ok(replays)
}

fn encrypt_data<T: Serialize>(data: &T) -> String {
    let key =
        hex::decode("EEBC1F57487F51921C0465665F8AE6D1658BB26DE6F8A069A3520293A572078F").unwrap();

    let bytes = rmp_serde::to_vec(data).unwrap();
    //let mut nonce = [0u8; 12];
    //getrandom(&mut nonce).unwrap();
    let nonce: [u8; 12] = *b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";
    let nonce_ga = nonce.into();

    let aes_gcm = Aes256Gcm::new_from_slice(&key).unwrap();
    let encrypted = aes_gcm.encrypt(&nonce_ga, &bytes[..]).unwrap();

    let mut data: Vec<u8> = Vec::new();
    data.extend_from_slice(&nonce);
    data.extend_from_slice(&encrypted);

    let r = base64_url::encode(&data);

    r
}

fn decrypt_response<T: for<'a> Deserialize<'a>>(bytes: &[u8]) -> Result<T, Box<dyn Error>> {
    let key =
        hex::decode("EEBC1F57487F51921C0465665F8AE6D1658BB26DE6F8A069A3520293A572078F").unwrap();
    let aes_gcm = Aes256Gcm::new_from_slice(&key).unwrap();

    let mut nonce = [0; 12];
    for i in 0..12 {
        nonce[i] = bytes[i];
    }

    //let nonce: GenericArray<_, _> = todo!();// GenericArray::from(&response_bytes[..12]);
    let nonce = GenericArray::from(nonce);

    let decrypted = match aes_gcm.decrypt(&nonce, &bytes[12..]) {
        Ok(decrypted) => decrypted,
        Err(e) => {
            panic!("Error decrypting: {:?}", e);
        }
    };

    match rmp_serde::from_slice::<responses::Response<T>>(&decrypted) {
        Ok(r) => Ok(r.body),
        Err(e) => {
            for b in &decrypted {
                print!("{:02X}", b);
            }
            println!("\n\n");
            Err(Box::new(e))
        }
    }
}