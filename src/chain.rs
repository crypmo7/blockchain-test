pub mod chain {
    use chrono::prelude::*;
    use sha2::{Digest, Sha256};
    #[derive(Debug, PartialEq)]
    //---------------------------------------structs---------------------------------------
    pub struct App<T> {
        pub bloks: Vec<T>,
    }
    #[derive(Debug, PartialEq)]
    pub struct Block {
        pub id: usize,
        pub hash: String,
        pub previous_hash: String,
        pub data: String,
        pub nonce: u64,
        pub time: String,
    }
//---------------------------------------get-time---------------------------------------
    fn get_time_as_string(s: &str) -> &str {
        let byte = s.as_bytes();
        for (i, &item) in byte.iter().enumerate() {
            if item == b'.' {
                return &s[0..i];
            }
        }
        &s[..]
    }

    fn time_to_string() -> String {
        let new_time = Utc::now().to_string();
        let a = get_time_as_string(&new_time[..]);
        a.to_string() + " Utc"
    }
//---------------------------------------check-valid-chain---------------------------------------
    fn valid_block(chain: &mut App<Block>) -> bool {
        let mut chain_valid = false;
        let mut last_hash = false;

        for i in 0..chain.bloks.len() {
            if chain.bloks[i].id > 0 && chain.bloks[i].previous_hash == chain.bloks[i -1].hash { //if previos hash is ok and app index > 0
                last_hash = true;
            } else if chain.bloks[i].id == 0 { //if block number is 1 its mean is our app has 1index
                last_hash = true;
            } else {
                last_hash = false;
                println!("there is a problem with last hash!");
            }
        }

        if chain.bloks[0].previous_hash == "genesis".to_string() && last_hash {
            chain_valid = true;
            println!("chain is valid.");
        } else {
            chain_valid = false;
            println!("chain is not valid!");
        }

        chain_valid
    }
//---------------------------------------implementations---------------------------------------***********************************************
    impl App<Block> {
        pub fn new() -> Self {
            Self { bloks: vec![] }
        }

        pub fn genesis(&mut self) { //create-genesis-block
            let genesis_block = Block {
                id: 0,
                previous_hash: String::from("genesis"),
                data: String::from("genesis!"),
                nonce: 2836,
                hash: String::new(),
                time: time_to_string(),
            };
            self.bloks.push(genesis_block);
            let mut hasher = Sha256::new();
            hasher.update(format!("{:#?}", self.bloks[0]));
            let hash_new = format!("{:X}", hasher.finalize());
            self.bloks[0].hash.push_str(&hash_new);
            println!("genesis block created");
        }

        pub fn add_new_block(&mut self, data: String) { //create-new-block
            let latest_block = self
                .bloks
                .last()
                .expect(
                    "there isn't genesis block for create last hash or another thing like last id!",
                )
                .clone();

            let last_hash = latest_block.hash.clone();
            let last_id = latest_block.id;
            let new_block = Block {
                id: last_id + 1,
                previous_hash: last_hash,
                data,
                nonce: 2837,
                hash: String::new(),
                time: time_to_string(),
            };
            if valid_block(self) {
                self.bloks.push(new_block);
                let mut hasher = Sha256::new();
                hasher.update(format!("{:#?}", self.bloks.last().unwrap().clone()));
                let new_hash = format!("{:X}", hasher.finalize());
                self.bloks[last_id + 1].hash.push_str(&new_hash);
            }
        }
    }
//---------------------------------------tests---------------------------------------****************************************************
    #[test]
    fn test() {
        let test_block = Block {
            id: 0,
            hash: String::new(),
            previous_hash: "genesis".to_string(),
            data: "genesis!".to_string(),
            nonce: 2836,
            time: "2022-07-28 07:57:04 Utc".to_string(),
        };
        let mut hasher = Sha256::new();
        hasher.update(format!("{:#?}", test_block));
        let test_hash = format!("{:X}", hasher.finalize());
        assert_eq!(
            "8DCB8013E02269BF4FFBC4C1B1886D4C52B57E591C915A3A674A6E9E0F7B3E33".to_string(),
            test_hash
        )
    }
}
