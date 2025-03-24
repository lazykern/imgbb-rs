use imgbb::ImgBB;
use std::time::Duration;

// Unit tests that don't require an API key
// These test the core client builder functionality

#[tokio::test]
async fn test_builder_with_timeout() {
    // Test that the builder with timeout completes without error
    let imgbb = ImgBB::builder("test_key")
        .timeout(Duration::from_secs(10))
        .build();
    
    assert!(imgbb.is_ok());
}

#[tokio::test]
async fn test_builder_with_user_agent() {
    // Test that the builder with user agent completes without error
    let imgbb = ImgBB::builder("test_key")
        .user_agent("test-agent")
        .build();
    
    assert!(imgbb.is_ok());
}

#[test]
fn test_custom_client() {
    // Create a custom reqwest client with specific configuration
    let custom_client = reqwest::Client::builder()
        .user_agent("CustomAgent/1.0")
        .build()
        .unwrap();
    
    // Test direct constructor with custom client
    let _imgbb_direct = ImgBB::new_with_client("test_key", custom_client.clone());
    
    // Test builder with custom client
    let imgbb_builder = ImgBB::builder("test_key")
        .client(custom_client)
        .build();
    
    assert!(imgbb_builder.is_ok());
}

#[tokio::test]
async fn test_new_method() {
    // Test that the new method works
    let _imgbb = ImgBB::new("test_key");
    // We can't test much else without making actual API calls
}

// Rest of the file... 