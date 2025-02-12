# VirtualHost Manager

This Rust project automates the process of creating and deleting virtual hosts on an Apache web server. It provides a command-line interface to manage virtual hosts by enabling, disabling, and restarting the Apache service.

## Features

- **Create Virtual Host**: Creates a new virtual host configuration, enables the site, and restarts Apache.
- **Delete Virtual Host**: Disables an existing virtual host, removes the configuration file, and restarts Apache.

## Installation

1. **Clone the Repository**:
    ```sh
    git clone https://github.com/naggix/virtualhosts.git
    cd virtualhosts
    ```

2. **Build the Program**:
    ```sh
    cargo build --release
    ```

3. **Move the Binary to a Directory in Your PATH**:
    ```sh
    sudo mv target/release/virtualhost /usr/local/bin/
    ```

## Usage

### Create a Virtual Host

To create a new virtual host, use the `create` subcommand with the `-c` flag, providing the domain name and the document root directory:

```sh
virtualhost create example.com /var/www/example.com
```

### Delete a Virtual Host

To delete an existing virtual host, use the `delete` subcommand with the domain name:

```sh
virtualhost delete example.com
```