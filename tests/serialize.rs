use serde_json;
use vsr_rust::rpc::RPC;

#[cfg(test)]
mod tests {
    use vsr_rust::rpc::Request;

    use super::*;

    #[test]
    fn test_rpc_serialization() {
      let payload_str = String::from("test");
        // Create an instance of RPC enum
        let rpc = RPC::Request(Request {
            payload: payload_str.clone().into_bytes(),
            client_id: 0,
            request_number: 0,
        });

        // Serialize the RPC instance to JSON
        let serialized = serde_json::to_string(&rpc).expect("Failed to serialize RPC");
        println!("serialized = {}", serialized);

        // Define the expected JSON output
        let deserialized: RPC = serde_json::from_str(&serialized).expect("Failed to deserialize RPC");
        println!("deserialized = {:?}", deserialized);
        if let RPC::Request(ref request) = rpc {
            println!("payload = {}", String::from_utf8_lossy(&request.payload));
            // Assert that the serialized output matches the expected JSON
            assert_eq!(String::from_utf8_lossy(&request.payload), payload_str);
        } else {
            panic!("RPC is not a Request");
        }

    }
}
