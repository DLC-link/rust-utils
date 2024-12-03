#![deny(unused_imports)]

use bdk::descriptor;
use bdk::keys::bip39::{Language, Mnemonic, WordCount};
use bdk::keys::{DerivableKey, ExtendedKey, GeneratableKey, GeneratedKey};
use bdk::miniscript::Segwitv0;
use bitcoin::util::bip32::{ChildNumber, DerivationPath, ExtendedPubKey};
use std::env;
use std::str::FromStr;

use secp256k1::{PublicKey, Secp256k1};

use serde_json::json;

fn main() {
    // Setup Blockchain Connection Object
    let network = match env::var("BITCOIN_NETWORK").as_deref() {
        Ok("bitcoin") => bitcoin::Network::Bitcoin,
        Ok("testnet") => bitcoin::Network::Testnet,
        Ok("signet") => bitcoin::Network::Signet,
        Ok("regtest") => bitcoin::Network::Regtest,
        _ => panic!(
            "Unknown Bitcoin Network, make sure to set BITCOIN_NETWORK in your env variables"
        ),
    };

    let secp = Secp256k1::new();
    let mnemonic: GeneratedKey<_, Segwitv0> =
        Mnemonic::generate((WordCount::Words24, Language::English))
            .expect("Mnemonic generation error");
    let mnemonic = mnemonic.into_key();
    let xkey: ExtendedKey = (mnemonic.clone(), None).into_extended_key().unwrap();
    let xprv = xkey
        .into_xprv(network)
        .expect("Privatekey info not found (should not happen)");
    let fingerprint = xprv.fingerprint(&secp);
    let phrase = mnemonic
        .words()
        .fold("".to_string(), |phrase, w| phrase + w + " ")
        .trim()
        .to_string();

    // Generating derived keys and first address
    let external_derivation_path = match network {
        bitcoin::Network::Bitcoin => "m/84h/0h/0h",
        bitcoin::Network::Testnet | bitcoin::Network::Signet | bitcoin::Network::Regtest => {
            "m/84h/1h/0h"
        }
    };

    let external_derivation_path =
        DerivationPath::from_str(external_derivation_path).expect("A valid derivation path");

    let signing_external_descriptor = descriptor!(wpkh((
        xprv,
        external_derivation_path.extend([ChildNumber::Normal { index: 0 }])
    )))
    .unwrap();

    let x = signing_external_descriptor.0.clone();

    let address = x.at_derivation_index(0).address(network).unwrap();
    let derived_ext_xpriv = xprv
        .derive_priv(
            &secp,
            &external_derivation_path.extend([
                ChildNumber::Normal { index: 0 },
                ChildNumber::Normal { index: 0 },
            ]),
        )
        .unwrap();
    let extended_pubkey = ExtendedPubKey::from_priv(&secp, &derived_ext_xpriv).public_key;
    let pub_key = PublicKey::from_secret_key(&secp, &derived_ext_xpriv.private_key);
    let secret_key = derived_ext_xpriv.private_key;

    println!(
        "{}",
        json!({ "mnemonic": phrase, "xprv": xprv.to_string(), "fingerprint": fingerprint.to_string(), "secret_key": secret_key, "public_key": pub_key, "extended_public_key": extended_pubkey, "network": network, "address": address })
    )
}
