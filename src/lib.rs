pub mod blockchain;
pub mod crypto;
pub mod data;
pub mod interactors;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[tokio::test]
    async fn main() -> Result<(), Box<dyn std::error::Error>> {
        let resp = reqwest::get("https://httpbin.org/ip")
            .await?
            .json::<HashMap<String, String>>()
            .await?;
        println!("{:#?}", resp);
        Ok(())
    }
}
