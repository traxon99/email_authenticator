mod password;

fn main() {
    use lettre::transport::smtp::authentication::Credentials; 
    use lettre::{Message, SmtpTransport, Transport}; 
    
    let email = Message::builder() 
      .from("Jackson's Email Authenticator <jacksony.testauth@gmail.com>".parse().unwrap()) 
      .to("Receiver <yanekjackson@gmail.com>".parse().unwrap()) 
      .subject("Email Authenticator: Verify Your Login") 
      .body(String::from("Please verify your login using this link: https://traxon99.github.io/"))
      .unwrap(); 
    
    let creds = Credentials::new("jacksony.testauth@gmail.com".to_string(), password::get_pass_from_file("src/password.txt")); 
    
    // Open a remote connection to gmail 
    let mailer = SmtpTransport::relay("smtp.gmail.com") 
      .unwrap() 
      .credentials(creds) 
      .build(); 
    
    // Send the email 
    match mailer.send(&email) { 
      Ok(_) => println!("Email sent successfully!"), 
      Err(e) => panic!("Could not send email: {:?}", e), 
    }
}
