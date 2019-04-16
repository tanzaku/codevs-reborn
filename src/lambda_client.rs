
#![allow(dead_code)]
// #![allow(unused_variables)]

extern crate codevs_reborn_lib;





extern crate rusoto_core;
extern crate rusoto_lambda;
extern crate serde;

// use rusoto_core::{Region, RusotoError};
use rusoto_core::{Region};
use rusoto_lambda::{InvocationRequest, InvokeError, Lambda, LambdaClient, ListFunctionsRequest};

// use serde::{Serialize, Deserialize};
// use serde::ser::Serializer;
use serde::ser::{Serialize, Serializer, SerializeSeq, SerializeMap};


use codevs_reborn_lib::types;


struct B {
    b: [u8; 100],
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Hello {
    firstName: String,
}

impl Serialize for B
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.b.len()))?;
        for e in self.b.iter() {
            seq.serialize_element(e)?;
        }
        seq.end()
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct PlanContext {
    packs: Vec<types::RawPack>,
    board: types::RawBoard,
    fire_action: u8,
}

pub fn test() {
    let region = Region::UsWest2;
    let client = LambdaClient::new(region);

    let input = B { b: [0_u8; 100] };
    let input = [[0; 3]; 3];
    let input = PlanContext { packs: vec![], board: [[0; 16]; 10], fire_action: 0, };
    let serialized = serde_json::to_string(&input).ok().map(|s| s.into_bytes());

    println!("{:?}", serialized);

    let input = Hello { firstName: "Test".to_string() };
    let serialized = serde_json::to_string(&input).ok().map(|s| s.into_bytes());

    println!("{:?}", serialized);

    let request = InvocationRequest {
        function_name: "lambda-rust-test".to_string(),
        invocation_type: Some("RequestResponse".to_string()),
        payload: serialized,
        ..Default::default()
    };

    let result = client.invoke(request).sync();
    let response = result.ok().unwrap();
    println!("{}", response.payload.unwrap().iter().map(|b| *b as char).collect::<String>());
}






// #[derive(Serialize, Deserialize, Debug)]
// struct Hello {
//     firstName: String,
// }


// pub fn test() {
//     let region = Region::UsWest2;

//     // DefaultProviderを使うと~/.aws/configを参照する
//     // http://rusoto.github.io/rusoto/rusoto_credential/struct.DefaultCredentialsProvider.html
//     let client = LambdaClient::new(region);

//     let input = r#"
//     {
//         "firstName": "Rustacean"
//     }
//     "#.to_string().into_bytes();

//     let input = Hello { firstName: "Test".to_string() };
//     let serialized = serde_json::to_string(&input).ok().map(|s| s.into_bytes());
//     // let serialized = serde_json::to_string(&input).ok().map(|s| s.bytes());

//     // http://rusoto.github.io/rusoto/rusoto_lambda/struct.InvocationRequest.html
//     let request = InvocationRequest {
//         function_name: "lambda-rust-test".to_string(),
//         invocation_type: Some("RequestResponse".to_string()),
//         // payload: Some(input),
//         payload: serialized,
//         ..Default::default()
//     };

//     let result = client.invoke(request).sync();

//     // assert!(result.is_err());
//     // if let Err(InvokeError::ResourceNotFound(resp)) = result {
//     //     assert!(resp.contains("Function not found:"));
//     // } else {
//     //     assert!(
//     //         false,
//     //         format!(
//     //             "expect Err(InvokeError::ResourceNotFound(_), found {:?}",
//     //             result
//     //         )
//     //     );
//     // }

//     // response: https://docs.rs/rusoto_lambda/0.33.0/rusoto_lambda/struct.InvocationResponse.html
//     let response = result.ok().unwrap();
//     println!("{}", response.payload.unwrap().iter().map(|b| *b as char).collect::<String>());
// }
