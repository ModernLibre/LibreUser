

// #[test]
// fn test_generate_and_validate_jwt() {
//     let user = User::default();
//     let mut claims = Claims::from(&user);
//     claims.expiration(Duration::from_secs(3600));

//     let (private_key, public_key) = generate_key_pair(2048);
//     let token = generate_jwt(claims, &private_key, jsonwebtoken::Algorithm::RS256);

//     let decoding_key =
//         DecodingKey::from_rsa_pem(&public_key.to_pkcs1_pem(LineEnding::LF).unwrap().as_bytes())
//             .unwrap();
//     let token_data = validate_jwt(&token, &decoding_key, jsonwebtoken::Algorithm::RS256)
//         .expect("Failed to validate token");

//     assert_eq!(token_data.claims.sub, user.uid.to_string());
//     assert!(
//         token_data.claims.exp
//             > SystemTime::now()
//                 .duration_since(UNIX_EPOCH)
//                 .unwrap()
//                 .as_secs() as usize
//     );
// }
