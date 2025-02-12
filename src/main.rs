use clap::{App, Arg, SubCommand};
use std::fs;
use std::io::{self, Write};
use std::process::Command;

fn main() {
    let matches = App::new("VirtualHost Manager")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Manages Apache virtual hosts")
        .subcommand(
            SubCommand::with_name("create")
                .about("Creates a new virtual host")
                .arg(
                    Arg::with_name("domain")
                        .help("The domain name for the virtual host")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("document_root")
                        .help("The document root for the virtual host")
                        .required(true)
                        .index(2),
                ),
        )
        .subcommand(
            SubCommand::with_name("delete")
                .about("Deletes an existing virtual host")
                .arg(
                    Arg::with_name("domain")
                        .help("The domain name for the virtual host")
                        .required(true)
                        .index(1),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("create") {
        let domain = matches.value_of("domain").unwrap();
        let doc_root = matches.value_of("document_root").unwrap();
        create_virtual_host(domain, doc_root);
    } else if let Some(matches) = matches.subcommand_matches("delete") {
        let domain = matches.value_of("domain").unwrap();
        delete_virtual_host(domain);
    }
}

fn create_virtual_host(domain: &str, doc_root: &str) {
    let conf_file = format!("/etc/apache2/sites-available/{}.conf", domain);

    // Create document root
    if let Err(e) = fs::create_dir_all(doc_root) {
        eprintln!("Failed to create document root: {}", e);
        return;
    }
    println!("Created document root at: {}", doc_root);

    // Set up virtual host configuration
    let vhost_config = format!(
        "<VirtualHost *:80>\n    ServerName {}\n    DocumentRoot {}\n    ErrorLog /var/log/apache2/{}_error.log\n    CustomLog /var/log/apache2/{}_access.log combined\n</VirtualHost>\n",
        domain, doc_root, domain, domain
    );
    if let Err(e) = fs::write(&conf_file, vhost_config) {
        eprintln!("Failed to write config file: {}", e);
        return;
    }
    println!("Created config file at: {}", conf_file);

    // Enable site
    if let Err(e) = run_command("sudo", &["a2ensite", domain]) {
        eprintln!("Failed to enable site: {}", e);
        return;
    }

    // Restart Apache
    if let Err(e) = run_command("sudo", &["systemctl", "restart", "apache2"]) {
        eprintln!("Failed to restart Apache: {}", e);
    } else {
        println!(
            "Apache restarted successfully. Virtual host {} is now active.",
            domain
        );
    }
}

fn delete_virtual_host(domain: &str) {
    let conf_file = format!("/etc/apache2/sites-available/{}.conf", domain);

    // Prompt for confirmation
    println!(
        "Are you sure you want to delete the virtual host for {}? (y/N)",
        domain
    );
    let mut confirmation = String::new();
    io::stdin()
        .read_line(&mut confirmation)
        .expect("Failed to read input");
    if confirmation.trim().to_lowercase() != "y" {
        println!("Operation cancelled.");
        return;
    }

    // Disable site
    if let Err(e) = run_command("sudo", &["a2dissite", domain]) {
        eprintln!("Failed to disable site: {}", e);
        return;
    }

    // Remove config file
    if let Err(e) = fs::remove_file(&conf_file) {
        eprintln!("Failed to remove config file: {}", e);
        return;
    }
    println!("Removed config file at: {}", conf_file);

    // Restart Apache
    if let Err(e) = run_command("sudo", &["systemctl", "restart", "apache2"]) {
        eprintln!("Failed to restart Apache: {}", e);
    } else {
        println!(
            "Apache restarted successfully. Virtual host {} has been deleted.",
            domain
        );
    }
}

fn run_command(cmd: &str, args: &[&str]) -> io::Result<()> {
    let status = Command::new(cmd).args(args).status()?;
    if status.success() {
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "Command failed"))
    }
}
