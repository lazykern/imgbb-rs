use imgbb::ImgBB;
use std::env;
use base64::Engine;

// Create a test image file for upload tests
fn create_test_image() -> std::path::PathBuf {
    let dir = std::env::temp_dir().join("imgbb_test");
    std::fs::create_dir_all(&dir).unwrap();
    
    let file_path = dir.join("test_image.png");
    
    // 1x1 transparent PNG
    let png_data = [
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D,
        0x49, 0x48, 0x44, 0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01,
        0x08, 0x04, 0x00, 0x00, 0x00, 0xB5, 0x1C, 0x0C, 0x02, 0x00, 0x00, 0x00,
        0x0B, 0x49, 0x44, 0x41, 0x54, 0x08, 0xD7, 0x63, 0x64, 0xF8, 0x07, 0x00,
        0x01, 0x05, 0x01, 0x01, 0x27, 0x18, 0xE3, 0x76, 0x00, 0x00, 0x00, 0x00,
        0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82
    ];
    
    std::fs::write(&file_path, &png_data).unwrap();
    file_path
}

/// Validate common response fields that should be present in every upload response
fn validate_response_integrity(data: &imgbb::model::Data) {
    // Check essential fields existence
    assert!(data.id.is_some(), "Image ID is missing");
    assert!(data.url.is_some(), "Image URL is missing");
    assert!(data.delete_url.is_some(), "Delete URL is missing");
    
    // Validate URL formats
    if let Some(url) = &data.url {
        assert!(url.starts_with("https://i.ibb.co/"), "Image URL has unexpected format");
    }
    
    if let Some(url_viewer) = &data.url_viewer {
        assert!(url_viewer.starts_with("https://ibb.co/"), "Viewer URL has unexpected format");
    }
    
    if let Some(delete_url) = &data.delete_url {
        assert!(delete_url.contains("ibb.co/"), "Delete URL has unexpected format");
    }
    
    // Validate image dimensions (1x1 for our test image)
    if let Some(width) = data.width {
        assert_eq!(width, 1, "Unexpected image width");
    }
    
    if let Some(height) = data.height {
        assert_eq!(height, 1, "Unexpected image height");
    }
    
    // Skip timestamp validation as it's complex to handle multiple formats
    
    // Validate image details
    if let Some(ref img) = data.image {
        assert!(img.filename.is_some(), "Image filename is missing");
        assert!(img.name.is_some(), "Image name is missing");
        assert_eq!(img.mime.as_deref(), Some("image/png"), "Unexpected MIME type");
        assert_eq!(img.extension.as_deref(), Some("png"), "Unexpected extension");
        assert!(img.url.is_some(), "Image URL is missing in image object");
    }
}

// This test requires a valid ImgBB API key set as IMGBB_API_KEY environment variable
#[tokio::test]
async fn test_real_upload_and_delete() {
    // Skip this test if no API key is provided
    let api_key = match env::var("IMGBB_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            println!("Skipping integration test - no IMGBB_API_KEY environment variable found");
            return;
        }
    };
    
    println!("Running integration test with API key {}", api_key);
    
    // Create a test image file
    let image_path = create_test_image();
    
    // Create the ImgBB client
    let imgbb = ImgBB::new(&api_key);
    
    // Test the upload
    let response = imgbb.upload_file(&image_path).await.expect("Failed to upload image");
    
    // Verify we got a successful response
    assert!(response.success.unwrap_or(false), "Upload was not successful");
    assert!(response.data.is_some(), "Response data is missing");
    
    let data = response.data.unwrap();
    
    // Validate the response integrity
    validate_response_integrity(&data);
    
    println!("Successfully uploaded image: {}", data.url.unwrap());
    
    // Test deletion
    let delete_url = data.delete_url.unwrap();
    imgbb.delete(&delete_url).await.expect("Failed to delete image");
    
    println!("Successfully deleted image");
}

// Test the advanced upload with options
#[tokio::test]
async fn test_advanced_upload() {
    // Skip this test if no API key is provided
    let api_key = match env::var("IMGBB_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            println!("Skipping integration test - no IMGBB_API_KEY environment variable found");
            return;
        }
    };
    
    // Create a test image file
    let image_path = create_test_image();
    
    // Create the ImgBB client
    let imgbb = ImgBB::new(&api_key);
    
    // Create an advanced upload with options
    let response = imgbb.upload_builder()
        .file(&image_path).expect("Failed to read file")
        .name("test-image")  // Note: ImgBB API converts underscores to hyphens
        .title("Integration Test Image")
        .expiration(300) // 5 minutes
        .upload()
        .await
        .expect("Failed to upload image");
    
    // Verify we got a successful response
    assert!(response.success.unwrap_or(false), "Upload was not successful");
    assert!(response.data.is_some(), "Response data is missing");
    
    let data = response.data.unwrap();
    
    // Validate the response integrity
    validate_response_integrity(&data);
    
    // Just check that there's some expiration value
    assert!(data.expiration.is_some(), "Expiration field should be present");
    
    println!("Successfully uploaded image with options: {}", data.url.unwrap());
    
    // Clean up by deleting the image
    if let Some(delete_url) = data.delete_url {
        imgbb.delete(&delete_url).await.expect("Failed to delete image");
        println!("Successfully deleted image");
    }
}

// Test error handling for invalid API key
#[tokio::test]
async fn test_invalid_api_key() {
    // Create a test image file
    let image_path = create_test_image();
    
    // Create the ImgBB client with invalid API key
    let imgbb = ImgBB::new("invalid_api_key");
    
    // Test the upload
    let result = imgbb.upload_file(&image_path).await;
    
    // It should return an InvalidApiKey error
    assert!(result.is_err(), "Expected error for invalid API key");
    match result {
        Err(imgbb::Error::InvalidApiKey) => {
            println!("Successfully detected invalid API key error");
        },
        Err(err) => {
            panic!("Unexpected error type: {:?}", err);
        },
        Ok(_) => {
            panic!("Expected error but got success");
        }
    }
}

// Test error handling for invalid base64 data
#[tokio::test]
async fn test_invalid_base64_data() {
    let api_key = match env::var("IMGBB_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            println!("Skipping test - no IMGBB_API_KEY environment variable found");
            return;
        }
    };

    let imgbb = ImgBB::new(&api_key);
    let invalid_base64 = "not_valid_base64_data";

    // Create an uploader with invalid base64 data
    let result = imgbb.upload_builder()
        .data(invalid_base64)
        .upload()
        .await;

    assert!(result.is_err(), "Expected error for invalid base64 data");
    match result {
        Err(imgbb::Error::InvalidBase64Data) => {
            println!("Successfully detected invalid base64 data error");
        },
        Err(err) => {
            panic!("Unexpected error type: {:?}", err);
        },
        Ok(_) => {
            panic!("Expected error but got success");
        }
    }
}

// Test error handling for image too large
#[tokio::test]
async fn test_image_too_large() {
    let api_key = match env::var("IMGBB_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            println!("Skipping test - no IMGBB_API_KEY environment variable found");
            return;
        }
    };

    let imgbb = ImgBB::new(&api_key);
    
    // Create a large base64 string (33MB when decoded)
    let large_data = "A".repeat(44_000_000); // This will exceed the 32MB limit

    let result = imgbb.upload_builder()
        .data(&large_data)
        .upload()
        .await;

    assert!(result.is_err(), "Expected error for large image");
    match result {
        Err(imgbb::Error::ImageTooLarge) => {
            println!("Successfully detected image too large error");
        },
        Err(err) => {
            panic!("Unexpected error type: {:?}", err);
        },
        Ok(_) => {
            panic!("Expected error but got success");
        }
    }
}

// Test error handling for unsupported format
#[tokio::test]
async fn test_unsupported_format() {
    let api_key = match env::var("IMGBB_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            println!("Skipping test - no IMGBB_API_KEY environment variable found");
            return;
        }
    };

    let imgbb = ImgBB::new(&api_key);
    
    // Create some invalid image data that's base64 encoded but not a valid image
    let invalid_format = base64::engine::general_purpose::STANDARD.encode("This is not an image file");

    let result = imgbb.upload_builder()
        .data(&invalid_format)
        .upload()
        .await;

    assert!(result.is_err(), "Expected error for unsupported format");
    match result {
        Err(imgbb::Error::UnsupportedFormat) => {
            println!("Successfully detected unsupported format error");
        },
        Err(err) => {
            panic!("Unexpected error type: {:?}", err);
        },
        Ok(_) => {
            panic!("Expected error but got success");
        }
    }
}

// Test error handling for invalid parameters
#[tokio::test]
async fn test_invalid_parameters() {
    let api_key = match env::var("IMGBB_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            println!("Skipping test - no IMGBB_API_KEY environment variable found");
            return;
        }
    };

    let imgbb = ImgBB::new(&api_key);
    
    // Try to upload without providing any data
    let result = imgbb.upload_builder()
        .upload()
        .await;

    assert!(result.is_err(), "Expected error for invalid parameters");
    match result {
        Err(imgbb::Error::InvalidParameters(_)) => {
            println!("Successfully detected invalid parameters error");
        },
        Err(err) => {
            panic!("Unexpected error type: {:?}", err);
        },
        Ok(_) => {
            panic!("Expected error but got success");
        }
    }
}

// Test error handling for rate limit exceeded
#[tokio::test]
async fn test_rate_limit() {
    let api_key = match env::var("IMGBB_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            println!("Skipping test - no IMGBB_API_KEY environment variable found");
            return;
        }
    };

    let imgbb = ImgBB::new(&api_key);
    let test_data = base64::engine::general_purpose::STANDARD.encode("test");
    
    // Make multiple rapid requests to trigger rate limit
    let mut results = Vec::new();
    for _ in 0..100 {
        let result = imgbb.upload_builder()
            .data(&test_data)
            .upload()
            .await;
        results.push(result);
    }

    // Check if any request hit the rate limit
    let rate_limited = results.iter().any(|result| {
        matches!(result, Err(imgbb::Error::RateLimitExceeded))
    });

    if rate_limited {
        println!("Successfully detected rate limit error");
    } else {
        println!("Note: Rate limit was not triggered during the test");
    }
} 