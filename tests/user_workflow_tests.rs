use quantum_forge_secure_comms::{
    StreamlinedSecureClient,
    SecureCommsError,
};
use tokio::time::{sleep, Duration, Instant};

/// Comprehensive user workflow tests covering real-world usage scenarios
/// 
/// This test suite validates various user interaction patterns and business workflows.

#[tokio::test]
async fn test_new_user_onboarding_workflow() -> Result<(), SecureCommsError> {
    println!("🧪 Testing New User Onboarding Workflow");
    
    // Step 1: New user creates account (simulated by client creation)
    println!("👤 Step 1: New user Alice creates account");
    let mut alice = StreamlinedSecureClient::new().await?;
    let alice_id = alice.get_client_id();
    println!("✅ Alice account created with ID: {alice_id}");
    
    // Step 2: Alice performs initial system health check
    println!("🏥 Step 2: Alice performs initial health check");
    let alice_health = alice.health_check().await?;
    println!("✅ Alice's system health check completed");
    println!("   System status: {}", if alice_health { "Healthy" } else { "Issues detected" });
    
    // Step 3: Alice gets performance metrics
    println!("📊 Step 3: Alice reviews system performance");
    let alice_metrics = alice.get_performance_metrics();
    println!("✅ Alice's performance metrics:");
    println!("   Setup time: {}ms", alice_metrics.total_setup_ms);
    println!("   Foundation: {}ms", alice_metrics.foundation_setup_ms);
    println!("   Crypto: {}ms", alice_metrics.crypto_init_ms);
    
    // Step 4: Alice attempts to send message before establishing channel (should fail)
    println!("❌ Step 4: Alice tries to send message without channel (expected to fail)");
    let result = alice.send_secure_message("unknown_user", b"Hello!").await;
    match result {
        Err(e) => println!("✅ Correctly failed with error: {e}"),
        Ok(_) => panic!("Should have failed for non-existent channel"),
    }
    
    // Step 5: Another user Bob joins
    println!("👤 Step 5: New user Bob joins the system");
    let mut bob = StreamlinedSecureClient::new().await?;
    let bob_id = bob.get_client_id();
    println!("✅ Bob account created with ID: {bob_id}");
    
    // Step 6: Alice establishes secure channel with Bob
    println!("🔗 Step 6: Alice establishes secure channel with Bob");
    let alice_to_bob = alice.establish_secure_channel("bob").await?;
    println!("✅ Alice → Bob channel established:");
    println!("   Channel ID: {}", alice_to_bob.channel_id);
    println!("   Security Level: {}", alice_to_bob.security_level);
    println!("   QKD Fidelity: {:.2}%", alice_to_bob.qkd_fidelity * 100.0);
    
    // Step 7: Bob establishes return channel
    println!("🔗 Step 7: Bob establishes return channel to Alice");
    let _bob_to_alice = bob.establish_secure_channel("alice").await?;
    println!("✅ Bob → Alice channel established");
    
    // Step 8: First successful message exchange
    println!("💬 Step 8: First message exchange");
    let alice_msg = "Hello Bob! Welcome to the secure communications system!";
    let sent_msg = alice.send_secure_message("bob", alice_msg.as_bytes()).await?;
    println!("✅ Alice sent welcome message: {}", &sent_msg.message_id[..8]);
    
    let bob_reply = "Hi Alice! Thanks for the welcome. This system is amazing!";
    let reply_msg = bob.send_secure_message("alice", bob_reply.as_bytes()).await?;
    println!("✅ Bob sent reply: {}", &reply_msg.message_id[..8]);
    
    // Step 9: Users check their active channels
    println!("📋 Step 9: Users review their active channels");
    let alice_channels = alice.list_secure_channels();
    let bob_channels = bob.list_secure_channels();
    
    println!("✅ Alice has {} active channel(s)", alice_channels.len());
    println!("✅ Bob has {} active channel(s)", bob_channels.len());
    
    // Verify the onboarding was successful
    assert_eq!(alice_channels.len(), 1);
    assert_eq!(bob_channels.len(), 1);
    assert_eq!(sent_msg.payload, alice_msg.as_bytes());
    assert_eq!(reply_msg.payload, bob_reply.as_bytes());
    
    println!("✅ New user onboarding workflow completed successfully");
    Ok(())
}

#[tokio::test]
async fn test_business_team_collaboration() -> Result<(), SecureCommsError> {
    println!("🧪 Testing Business Team Collaboration Workflow");
    
    // Create business team members
    let mut manager = StreamlinedSecureClient::new().await?;
    let mut developer = StreamlinedSecureClient::new().await?;
    let mut designer = StreamlinedSecureClient::new().await?;
    let mut qa_engineer = StreamlinedSecureClient::new().await?;
    
    println!("✅ Business team created: Manager, Developer, Designer, QA Engineer");
    
    // Establish team communication network
    println!("🔗 Establishing team communication network");
    let mut connection_count = 0;
    
    // Manager establishes channels to all team members
    let _manager_to_dev = manager.establish_secure_channel("developer").await?;
    let _manager_to_designer = manager.establish_secure_channel("designer").await?;
    let _manager_to_qa = manager.establish_secure_channel("qa_engineer").await?;
    connection_count += 3;
    
    // Developer establishes channels
    let _dev_to_manager = developer.establish_secure_channel("manager").await?;
    let _dev_to_designer = developer.establish_secure_channel("designer").await?;
    let _dev_to_qa = developer.establish_secure_channel("qa_engineer").await?;
    connection_count += 3;
    
    // Designer establishes channels
    let _designer_to_manager = designer.establish_secure_channel("manager").await?;
    let _designer_to_dev = designer.establish_secure_channel("developer").await?;
    let _designer_to_qa = designer.establish_secure_channel("qa_engineer").await?;
    connection_count += 3;
    
    // QA Engineer establishes channels
    let _qa_to_manager = qa_engineer.establish_secure_channel("manager").await?;
    let _qa_to_dev = qa_engineer.establish_secure_channel("developer").await?;
    let _qa_to_designer = qa_engineer.establish_secure_channel("designer").await?;
    connection_count += 3;
    
    println!("✅ Team network established: {connection_count} secure channels");
    
    // Simulate business workflow: Project planning meeting
    println!("📋 Business Workflow: Project Planning Meeting");
    
    // Sequential message sending to avoid borrowing issues
    let msg1 = manager.send_secure_message("developer", "Please provide estimates for the new feature implementation.".as_bytes()).await?;
    println!("💼 Manager → Developer: {} ({})", 
        &"Please provide estimates for the new feature implementation."[..40], &msg1.message_id[..8]);
    assert_eq!(msg1.payload, "Please provide estimates for the new feature implementation.".as_bytes());
    sleep(Duration::from_millis(100)).await;
    
    let msg2 = developer.send_secure_message("manager", "Initial estimate: 2 weeks for backend, 1 week for API integration.".as_bytes()).await?;
    println!("💼 Developer → Manager: {} ({})", 
        &"Initial estimate: 2 weeks for backend, 1 week for API integration."[..40], &msg2.message_id[..8]);
    assert_eq!(msg2.payload, "Initial estimate: 2 weeks for backend, 1 week for API integration.".as_bytes());
    sleep(Duration::from_millis(100)).await;
    
    let msg3 = manager.send_secure_message("designer", "Can you create mockups for the new user interface?".as_bytes()).await?;
    println!("💼 Manager → Designer: {} ({})", 
        &"Can you create mockups for the new user interface?"[..40], &msg3.message_id[..8]);
    assert_eq!(msg3.payload, "Can you create mockups for the new user interface?".as_bytes());
    sleep(Duration::from_millis(100)).await;
    
    let msg4 = designer.send_secure_message("manager", "I'll have the UI mockups ready by tomorrow morning.".as_bytes()).await?;
    println!("💼 Designer → Manager: {} ({})", 
        &"I'll have the UI mockups ready by tomorrow morning."[..40], &msg4.message_id[..8]);
    assert_eq!(msg4.payload, "I'll have the UI mockups ready by tomorrow morning.".as_bytes());
    sleep(Duration::from_millis(100)).await;
    
    let msg5 = manager.send_secure_message("qa_engineer", "Please prepare test cases for the new feature.".as_bytes()).await?;
    println!("💼 Manager → QA Engineer: {} ({})", 
        &"Please prepare test cases for the new feature."[..40], &msg5.message_id[..8]);
    assert_eq!(msg5.payload, "Please prepare test cases for the new feature.".as_bytes());
    sleep(Duration::from_millis(100)).await;
    
    let msg6 = qa_engineer.send_secure_message("manager", "Test plan will be ready by Friday. Need feature specs first.".as_bytes()).await?;
    println!("💼 QA Engineer → Manager: {} ({})", 
        &"Test plan will be ready by Friday. Need feature specs first."[..40], &msg6.message_id[..8]);
    assert_eq!(msg6.payload, "Test plan will be ready by Friday. Need feature specs first.".as_bytes());
    sleep(Duration::from_millis(100)).await;
    
    let msg7 = developer.send_secure_message("designer", "Need to coordinate on the API data structure for UI.".as_bytes()).await?;
    println!("💼 Developer → Designer: {} ({})", 
        &"Need to coordinate on the API data structure for UI."[..40], &msg7.message_id[..8]);
    assert_eq!(msg7.payload, "Need to coordinate on the API data structure for UI.".as_bytes());
    sleep(Duration::from_millis(100)).await;
    
    let msg8 = designer.send_secure_message("developer", "Let's schedule a sync meeting for API-UI alignment.".as_bytes()).await?;
    println!("💼 Designer → Developer: {} ({})", 
        &"Let's schedule a sync meeting for API-UI alignment."[..40], &msg8.message_id[..8]);
    assert_eq!(msg8.payload, "Let's schedule a sync meeting for API-UI alignment.".as_bytes());
    sleep(Duration::from_millis(100)).await;
    
    // Document sharing simulation
    println!("📄 Document Sharing Workflow");
    
    let documents = vec![
        ("Technical Specification", "This document outlines the technical requirements...".repeat(100)),
        ("UI Design Guidelines", "Design system and component specifications...".repeat(80)),
        ("Test Plan Document", "Comprehensive testing strategy and test cases...".repeat(90)),
        ("Project Timeline", "Milestone dates and delivery schedule...".repeat(60)),
    ];
    
    for (doc_name, doc_content) in &documents {
        let document_data = format!("DOCUMENT: {doc_name}\n\nCONTENT:\n{doc_content}");
        
        // Manager shares document with all team members
        let recipients = vec!["developer", "designer", "qa_engineer"];
        for recipient_role in recipients {
            let sent_doc = manager.send_secure_message(recipient_role, document_data.as_bytes()).await?;
            println!("📄 Manager shared '{}' with {} ({} bytes)", 
                doc_name, recipient_role, sent_doc.payload.len());
            
            // Verify document integrity
            assert_eq!(sent_doc.payload, document_data.as_bytes());
        }
    }
    
    // Team status check
    println!("📊 Team Communication Status Check");
    
    // Check each team member individually to avoid borrowing conflicts
    let manager_channels = manager.list_secure_channels();
    let manager_channel_count = manager_channels.len();
    let manager_health = manager.health_check().await?;
    println!("✅ manager: {} channels, system health: {}", 
        manager_channel_count, if manager_health { "Good" } else { "Issues" });
    
    let developer_channels = developer.list_secure_channels();
    let developer_channel_count = developer_channels.len();
    let developer_health = developer.health_check().await?;
    println!("✅ developer: {} channels, system health: {}", 
        developer_channel_count, if developer_health { "Good" } else { "Issues" });
    
    let designer_channels = designer.list_secure_channels();
    let designer_channel_count = designer_channels.len();
    let designer_health = designer.health_check().await?;
    println!("✅ designer: {} channels, system health: {}", 
        designer_channel_count, if designer_health { "Good" } else { "Issues" });
    
    let qa_channels = qa_engineer.list_secure_channels();
    let qa_channel_count = qa_channels.len();
    let qa_health = qa_engineer.health_check().await?;
    println!("✅ qa_engineer: {} channels, system health: {}", 
        qa_channel_count, if qa_health { "Good" } else { "Issues" });
    
    println!("✅ Business team collaboration workflow completed successfully");
    Ok(())
}

#[tokio::test]
async fn test_customer_support_scenario() -> Result<(), SecureCommsError> {
    println!("🧪 Testing Customer Support Scenario");
    
    // Create customer support environment
    let mut customer = StreamlinedSecureClient::new().await?;
    let mut support_agent = StreamlinedSecureClient::new().await?;
    let mut technical_specialist = StreamlinedSecureClient::new().await?;
    
    println!("✅ Support environment created: Customer, Support Agent, Technical Specialist");
    
    // Establish secure channels between all parties
    println!("🔗 Establishing support channels");
    let _customer_to_support = customer.establish_secure_channel("support_agent").await?;
    let _support_to_customer = support_agent.establish_secure_channel("customer").await?;
    
    println!("✅ Customer support channels established");
    
    // Initial support conversation - sequential to avoid borrowing issues
    println!("💬 Customer support conversation");
    
    let msg1 = customer.send_secure_message("support_agent", "Hello, I'm having trouble with secure file transfers. The system keeps failing.".as_bytes()).await?;
    println!("🎧 Customer → Support: {} ({})", 
        &"Hello, I'm having trouble with secure file transfers. The system keeps failing."[..50], &msg1.message_id[..8]);
    
    let msg2 = support_agent.send_secure_message("customer", "Hi! I'm here to help. Can you tell me what error message you're seeing?".as_bytes()).await?;
    println!("🎧 Support → Customer: {} ({})", 
        &"Hi! I'm here to help. Can you tell me what error message you're seeing?"[..50], &msg2.message_id[..8]);
    
    let msg3 = customer.send_secure_message("support_agent", "The error says 'Channel not established' but I'm sure I set up the connection correctly.".as_bytes()).await?;
    println!("🎧 Customer → Support: {} ({})", 
        &"The error says 'Channel not established' but I'm sure I set up the connection correctly."[..50], &msg3.message_id[..8]);
    
    let msg4 = support_agent.send_secure_message("customer", "Let me check your account settings. Can you try sending a small test file?".as_bytes()).await?;
    println!("🎧 Support → Customer: {} ({})", 
        &"Let me check your account settings. Can you try sending a small test file?"[..50], &msg4.message_id[..8]);
    
    // Escalation to technical specialist
    println!("🔧 Escalating to technical specialist");
    
    let _support_to_tech = support_agent.establish_secure_channel("technical_specialist").await?;
    let _tech_to_support = technical_specialist.establish_secure_channel("support_agent").await?;
    let _tech_to_customer = technical_specialist.establish_secure_channel("customer").await?;
    let _customer_to_tech = customer.establish_secure_channel("technical_specialist").await?;
    
    // Technical specialist conversation - sequential
    let tech_msg1 = support_agent.send_secure_message("technical_specialist", "Customer having issues with large file transfers. Need diagnostic assistance.".as_bytes()).await?;
    println!("🔧 Support → Tech: {} ({})", 
        &"Customer having issues with large file transfers. Need diagnostic assistance."[..50], &tech_msg1.message_id[..8]);
    
    let tech_msg2 = technical_specialist.send_secure_message("support_agent", "I'll handle this. Let me run some diagnostics and test the customer's setup.".as_bytes()).await?;
    println!("🔧 Tech → Support: {} ({})", 
        &"I'll handle this. Let me run some diagnostics and test the customer's setup."[..50], &tech_msg2.message_id[..8]);
    
    let tech_msg3 = technical_specialist.send_secure_message("customer", "Hi! I'm a technical specialist. Let's test your file transfer capability step by step.".as_bytes()).await?;
    println!("🔧 Tech → Customer: {} ({})", 
        &"Hi! I'm a technical specialist. Let's test your file transfer capability step by step."[..50], &tech_msg3.message_id[..8]);
    
    let tech_msg4 = customer.send_secure_message("technical_specialist", "Great! I'm ready to test. What should I try first?".as_bytes()).await?;
    println!("🔧 Customer → Tech: {} ({})", 
        &"Great! I'm ready to test. What should I try first?"[..50], &tech_msg4.message_id[..8]);
    
    // File transfer testing simulation
    println!("📁 Testing file transfer capabilities");
    
    let test_files = vec![
        ("small_test.txt", vec![0x54, 0x65, 0x73, 0x74], 1024),      // 1KB test file
        ("medium_doc.pdf", vec![0x25, 0x50, 0x44, 0x46], 100 * 1024), // 100KB PDF
        ("large_image.jpg", vec![0xFF, 0xD8, 0xFF, 0xE0], 1024 * 1024), // 1MB JPEG
    ];
    
    for (filename, header, size) in test_files {
        let mut test_data = header.clone();
        test_data.resize(size, 0x42); // Fill with test pattern
        
        let start_time = Instant::now();
        let test_transfer = customer.send_secure_message("technical_specialist", &test_data).await?;
        let transfer_time = start_time.elapsed();
        
        assert_eq!(test_transfer.payload.len(), size);
        assert_eq!(test_transfer.payload[..header.len()], header);
        
        let throughput = (size as f64 / (1024.0 * 1024.0)) / transfer_time.as_secs_f64();
        
        println!("📁 Test transfer {}: {} KB, {:.2} MB/s", 
            filename, size / 1024, throughput);
        
        // Technical specialist confirms receipt
        let confirmation = format!("Received {filename} successfully. Transfer rate: {throughput:.2} MB/s");
        let _tech_response = technical_specialist.send_secure_message("customer", confirmation.as_bytes()).await?;
    }
    
    // Issue resolution
    println!("✅ Issue resolution");
    
    let resolution_msg1 = technical_specialist.send_secure_message("customer", "All file transfers are working correctly. The issue was with your initial channel setup.".as_bytes()).await?;
    println!("✅ Tech → Customer: {} ({})", 
        &"All file transfers are working correctly. The issue was with your initial channel setup."[..50], &resolution_msg1.message_id[..8]);
    
    let resolution_msg2 = customer.send_secure_message("technical_specialist", "Thank you! Everything is working perfectly now. I really appreciate the help.".as_bytes()).await?;
    println!("✅ Customer → Tech: {} ({})", 
        &"Thank you! Everything is working perfectly now. I really appreciate the help."[..50], &resolution_msg2.message_id[..8]);
    
    let resolution_msg3 = technical_specialist.send_secure_message("support_agent", "Issue resolved. Customer's file transfer capability is fully functional.".as_bytes()).await?;
    println!("✅ Tech → Support: {} ({})", 
        &"Issue resolved. Customer's file transfer capability is fully functional."[..50], &resolution_msg3.message_id[..8]);
    
    let resolution_msg4 = support_agent.send_secure_message("customer", "Glad we could help! Please don't hesitate to contact us if you need further assistance.".as_bytes()).await?;
    println!("✅ Support → Customer: {} ({})", 
        &"Glad we could help! Please don't hesitate to contact us if you need further assistance."[..50], &resolution_msg4.message_id[..8]);
    
    // Final status check
    println!("📊 Final support session status");
    
    let customer_channels = customer.list_secure_channels();
    println!("✅ Customer: {} active channels", customer_channels.len());
    
    let support_channels = support_agent.list_secure_channels();
    println!("✅ Support Agent: {} active channels", support_channels.len());
    
    let tech_channels = technical_specialist.list_secure_channels();
    println!("✅ Technical Specialist: {} active channels", tech_channels.len());
    
    println!("✅ Customer support scenario completed successfully");
    Ok(())
}

#[tokio::test]
async fn test_remote_work_collaboration() -> Result<(), SecureCommsError> {
    println!("🧪 Testing Remote Work Collaboration");
    
    // Create remote team
    let mut team_lead = StreamlinedSecureClient::new().await?;
    let mut remote_dev1 = StreamlinedSecureClient::new().await?;
    let mut remote_dev2 = StreamlinedSecureClient::new().await?;
    let mut project_manager = StreamlinedSecureClient::new().await?;
    
    println!("✅ Remote team assembled: Team Lead, Remote Dev 1, Remote Dev 2, Project Manager");
    
    // Establish team communication mesh
    println!("🌐 Setting up remote collaboration network");
    
    // Team lead establishes channels
    let _tl_to_dev1 = team_lead.establish_secure_channel("remote_dev1").await?;
    let _tl_to_dev2 = team_lead.establish_secure_channel("remote_dev2").await?;
    let _tl_to_pm = team_lead.establish_secure_channel("project_manager").await?;
    
    // Remote devs establish channels
    let _dev1_to_tl = remote_dev1.establish_secure_channel("team_lead").await?;
    let _dev1_to_dev2 = remote_dev1.establish_secure_channel("remote_dev2").await?;
    let _dev1_to_pm = remote_dev1.establish_secure_channel("project_manager").await?;
    
    let _dev2_to_tl = remote_dev2.establish_secure_channel("team_lead").await?;
    let _dev2_to_dev1 = remote_dev2.establish_secure_channel("remote_dev1").await?;
    let _dev2_to_pm = remote_dev2.establish_secure_channel("project_manager").await?;
    
    // Project manager establishes channels
    let _pm_to_tl = project_manager.establish_secure_channel("team_lead").await?;
    let _pm_to_dev1 = project_manager.establish_secure_channel("remote_dev1").await?;
    let _pm_to_dev2 = project_manager.establish_secure_channel("remote_dev2").await?;
    
    println!("✅ Remote collaboration network established");
    
    // Daily standup simulation - sequential to avoid borrowing issues
    println!("🗣️ Daily Standup Meeting (Async Messages)");
    
    let update1 = remote_dev1.send_secure_message("team_lead", "Yesterday: Completed user authentication module. Today: Working on API integration. Blockers: None.".as_bytes()).await?;
    println!("🗣️ Remote Dev 1 → Team Lead: {} ({})", 
        &"Yesterday: Completed user authentication module. Today: Working on API integration. Blockers: None."[..50], &update1.message_id[..8]);
    sleep(Duration::from_millis(100)).await;
    
    let update2 = remote_dev2.send_secure_message("team_lead", "Yesterday: Fixed database connection issues. Today: Implementing data validation. Blockers: Need schema review.".as_bytes()).await?;
    println!("🗣️ Remote Dev 2 → Team Lead: {} ({})", 
        &"Yesterday: Fixed database connection issues. Today: Implementing data validation. Blockers: Need schema review."[..50], &update2.message_id[..8]);
    sleep(Duration::from_millis(100)).await;
    
    let update3 = project_manager.send_secure_message("team_lead", "Yesterday: Updated project timeline. Today: Client meeting at 2 PM. Blockers: Waiting for QA feedback.".as_bytes()).await?;
    println!("🗣️ Project Manager → Team Lead: {} ({})", 
        &"Yesterday: Updated project timeline. Today: Client meeting at 2 PM. Blockers: Waiting for QA feedback."[..50], &update3.message_id[..8]);
    sleep(Duration::from_millis(100)).await;
    
    let update4 = team_lead.send_secure_message("remote_dev1", "Great progress on auth! Let me know if you need any help with API integration.".as_bytes()).await?;
    println!("🗣️ Team Lead → Remote Dev 1: {} ({})", 
        &"Great progress on auth! Let me know if you need any help with API integration."[..50], &update4.message_id[..8]);
    sleep(Duration::from_millis(100)).await;
    
    let update5 = team_lead.send_secure_message("remote_dev2", "I'll review the schema today. Can you send me the current draft?".as_bytes()).await?;
    println!("🗣️ Team Lead → Remote Dev 2: {} ({})", 
        &"I'll review the schema today. Can you send me the current draft?"[..50], &update5.message_id[..8]);
    sleep(Duration::from_millis(100)).await;
    
    let update6 = team_lead.send_secure_message("project_manager", "I'll prepare QA feedback before your client meeting.".as_bytes()).await?;
    println!("🗣️ Team Lead → Project Manager: {} ({})", 
        &"I'll prepare QA feedback before your client meeting."[..50], &update6.message_id[..8]);
    sleep(Duration::from_millis(100)).await;
    
    // Code review process
    println!("👨‍💻 Code Review Process");
    
    let code_review_data = r#"
    PULL REQUEST #123: Implement User Authentication
    
    Files changed:
    - src/auth/login.rs (new)
    - src/auth/session.rs (modified)
    - tests/auth_tests.rs (new)
    
    Changes:
    + Added secure login functionality
    + Implemented session management
    + Added comprehensive test coverage
    
    Please review and provide feedback.
    "#;
    
    // Developer submits code for review
    let code_submission = remote_dev1.send_secure_message("team_lead", code_review_data.as_bytes()).await?;
    println!("📝 Remote Dev 1 submitted code review: {} bytes", code_submission.payload.len());
    
    // Team lead reviews and provides feedback
    let review_feedback = r#"
    CODE REVIEW FEEDBACK:
    
    Overall: Excellent work! Clean implementation.
    
    Suggestions:
    1. Add input validation on line 45
    2. Consider using constant for session timeout
    3. Add error logging in catch blocks
    
    Approved with minor changes.
    "#;
    
    let feedback_msg = team_lead.send_secure_message("remote_dev1", review_feedback.as_bytes()).await?;
    println!("✅ Team Lead provided review feedback: {} bytes", feedback_msg.payload.len());
    
    // File sharing simulation
    println!("📂 Secure File Sharing");
    
    let shared_files = vec![
        ("project_spec.pdf", vec![0x25, 0x50, 0x44, 0x46], 256 * 1024), // PDF
        ("design_mockup.png", vec![0x89, 0x50, 0x4E, 0x47], 512 * 1024), // PNG
        ("database_schema.sql", b"CREATE TABLE users (".to_vec(), 32 * 1024), // SQL
    ];
    
    for (filename, header, size) in shared_files {
        let mut file_data = header.clone();
        file_data.resize(size, 0x20); // Fill with spaces
        
        let start_time = Instant::now();
        let shared_file = project_manager.send_secure_message("team_lead", &file_data).await?;
        let transfer_time = start_time.elapsed();
        
        let throughput = (size as f64 / (1024.0 * 1024.0)) / transfer_time.as_secs_f64();
        
        println!("📁 Shared {}: {} KB, {:.2} MB/s", filename, size / 1024, throughput);
        
        // Verify file integrity
        assert_eq!(shared_file.payload.len(), size);
        assert_eq!(shared_file.payload[..header.len()], header);
    }
    
    // End-of-day summary
    println!("📊 End-of-Day Team Summary");
    
    let team_members = vec![
        ("team_lead", &team_lead),
        ("remote_dev1", &remote_dev1),
        ("remote_dev2", &remote_dev2),
        ("project_manager", &project_manager),
    ];
    
    for (role, team_member) in &team_members {
        let channels = team_member.list_secure_channels();
        let metrics = team_member.get_performance_metrics();
        
        println!("✅ {}: {} channels, setup: {}ms", 
            role, channels.len(), metrics.total_setup_ms);
    }
    
    println!("✅ Remote work collaboration completed successfully");
    Ok(())
}

#[tokio::test]
async fn test_emergency_communication_protocol() -> Result<(), SecureCommsError> {
    println!("🧪 Testing Emergency Communication Protocol");
    
    // Create emergency response team
    let mut incident_commander = StreamlinedSecureClient::new().await?;
    let mut security_team = StreamlinedSecureClient::new().await?;
    let mut technical_team = StreamlinedSecureClient::new().await?;
    let mut management = StreamlinedSecureClient::new().await?;
    
    println!("🚨 Emergency response team assembled");
    
    // Rapid channel establishment for emergency - sequential to avoid borrowing issues
    println!("⚡ Rapid emergency channel establishment");
    
    let start_time = Instant::now();
    
    // Incident commander to teams
    let _ic_to_security = incident_commander.establish_secure_channel("security_team").await?;
    let _ic_to_technical = incident_commander.establish_secure_channel("technical_team").await?;
    let _ic_to_management = incident_commander.establish_secure_channel("management").await?;
    
    // Teams back to incident commander (for responses)
    let _security_to_ic = security_team.establish_secure_channel("incident_commander").await?;
    let _technical_to_ic = technical_team.establish_secure_channel("incident_commander").await?;
    let _management_to_ic = management.establish_secure_channel("incident_commander").await?;
    
    // Inter-team communication
    let _security_to_technical = security_team.establish_secure_channel("technical_team").await?;
    
    let setup_time = start_time.elapsed();
    
    println!("✅ Emergency channels established in {setup_time:?}");
    
    // Emergency alert broadcast
    println!("📢 Emergency alert broadcast");
    
    let emergency_alert = "SECURITY INCIDENT DETECTED: Unauthorized access attempt on production servers. All teams respond immediately.";
    
    let alert_start = Instant::now();
    let alert_to_security = incident_commander.send_secure_message("security_team", emergency_alert.as_bytes()).await?;
    let alert_to_technical = incident_commander.send_secure_message("technical_team", emergency_alert.as_bytes()).await?;
    let alert_to_management = incident_commander.send_secure_message("management", emergency_alert.as_bytes()).await?;
    let alert_time = alert_start.elapsed();
    
    println!("🚨 Emergency alerts sent in {alert_time:?}");
    println!("   Security team: {}", &alert_to_security.message_id[..8]);
    println!("   Technical team: {}", &alert_to_technical.message_id[..8]);
    println!("   Management: {}", &alert_to_management.message_id[..8]);
    
    // Rapid response coordination - sequential to avoid borrowing issues
    println!("🔄 Rapid response coordination");
    
    let response1 = security_team.send_secure_message("incident_commander", "Security team responding. Initiating threat analysis and access review.".as_bytes()).await?;
    println!("🚨 Security → Commander: {} ({})", 
        &"Security team responding. Initiating threat analysis and access review."[..50], &response1.message_id[..8]);
    sleep(Duration::from_millis(50)).await;
    
    let response2 = technical_team.send_secure_message("incident_commander", "Technical team online. Beginning system integrity check and log analysis.".as_bytes()).await?;
    println!("🚨 Technical → Commander: {} ({})", 
        &"Technical team online. Beginning system integrity check and log analysis."[..50], &response2.message_id[..8]);
    sleep(Duration::from_millis(50)).await;
    
    let response3 = management.send_secure_message("incident_commander", "Management notified. Preparing stakeholder communication. What's the severity?".as_bytes()).await?;
    println!("🚨 Management → Commander: {} ({})", 
        &"Management notified. Preparing stakeholder communication. What's the severity?"[..50], &response3.message_id[..8]);
    sleep(Duration::from_millis(50)).await;
    
    let response4 = incident_commander.send_secure_message("security_team", "Priority 1 incident. Isolate affected systems immediately.".as_bytes()).await?;
    println!("🚨 Commander → Security: {} ({})", 
        &"Priority 1 incident. Isolate affected systems immediately."[..50], &response4.message_id[..8]);
    sleep(Duration::from_millis(50)).await;
    
    let response5 = incident_commander.send_secure_message("technical_team", "Run full security scan and provide status in 5 minutes.".as_bytes()).await?;
    println!("🚨 Commander → Technical: {} ({})", 
        &"Run full security scan and provide status in 5 minutes."[..50], &response5.message_id[..8]);
    sleep(Duration::from_millis(50)).await;
    
    let response6 = incident_commander.send_secure_message("management", "Severity: High. Potential data exposure. Recommend immediate action.".as_bytes()).await?;
    println!("🚨 Commander → Management: {} ({})", 
        &"Severity: High. Potential data exposure. Recommend immediate action."[..50], &response6.message_id[..8]);
    sleep(Duration::from_millis(50)).await;
    
    // Critical data transmission
    println!("📊 Critical data transmission");
    
    let incident_report = format!(
        "INCIDENT REPORT #{}\n\nTimestamp: {}\nSeverity: HIGH\nAffected Systems: Production Web Servers\nThreat Vector: Unauthorized SSH Access\nImmediate Actions Taken:\n- Blocked suspicious IP addresses\n- Revoked compromised credentials\n- Enabled enhanced monitoring\n\nOngoing Investigation:\n- Forensic analysis in progress\n- System integrity verification\n- Data access audit\n\nRecommendations:\n- Implement additional access controls\n- Update security policies\n- Conduct team security training\n\nStatus: CONTAINED\nNext Update: 30 minutes",
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
        "2024-01-15 14:30:00 UTC"
    );
    
    let report_data = incident_report.as_bytes();
    let report_start = Instant::now();
    
    let report_to_management = incident_commander.send_secure_message("management", report_data).await?;
    let report_transmission_time = report_start.elapsed();
    
    println!("📋 Incident report transmitted: {} bytes in {:?}", 
        report_data.len(), report_transmission_time);
    
    // Verify report integrity
    assert_eq!(report_to_management.payload, report_data);
    
    // Emergency status verification - check each participant individually
    println!("✅ Emergency communication status verification");
    
    let ic_channels = incident_commander.list_secure_channels();
    let ic_channel_count = ic_channels.len();
    let ic_health = incident_commander.health_check().await?;
    println!("✅ Incident Commander: {} channels, system: {}", 
        ic_channel_count, if ic_health { "OPERATIONAL" } else { "DEGRADED" });
    
    let security_channels = security_team.list_secure_channels();
    let security_channel_count = security_channels.len();
    let security_health = security_team.health_check().await?;
    println!("✅ Security Team: {} channels, system: {}", 
        security_channel_count, if security_health { "OPERATIONAL" } else { "DEGRADED" });
    
    let tech_channels = technical_team.list_secure_channels();
    let tech_channel_count = tech_channels.len();
    let tech_health = technical_team.health_check().await?;
    println!("✅ Technical Team: {} channels, system: {}", 
        tech_channel_count, if tech_health { "OPERATIONAL" } else { "DEGRADED" });
    
    let mgmt_channels = management.list_secure_channels();
    let mgmt_channel_count = mgmt_channels.len();
    let mgmt_health = management.health_check().await?;
    println!("✅ Management: {} channels, system: {}", 
        mgmt_channel_count, if mgmt_health { "OPERATIONAL" } else { "DEGRADED" });
    
    // Emergency protocol completion
    let total_emergency_time = start_time.elapsed();
    println!("🎯 Emergency protocol completed in {total_emergency_time:?}");
    println!("   Channel setup: {setup_time:?}");
    println!("   Alert broadcast: {alert_time:?}");
    println!("   Report transmission: {report_transmission_time:?}");
    
    println!("✅ Emergency communication protocol completed successfully");
    Ok(())
} 