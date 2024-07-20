use std::process::Command;
use std::io;
use std::io::Write;

/// Lists all current IP addresses on the system.
fn list_ip_addresses() -> io::Result<()> {
    // Execute the `ipconfig /all` command to get all IP configurations.
    let output = Command::new("ipconfig")
        .args(&["/all"])
        .output()?;

    // Check if the command executed successfully.
    if output.status.success() {
        // Print the command output to the console.
        println!("{}", String::from_utf8_lossy(&output.stdout));
        Ok(())
    } else {
        // Print an error message if the command failed.
        eprintln!("Failed to retrieve IP addresses.");
        eprintln!("Standard Error: {}", String::from_utf8_lossy(&output.stderr));
        Err(io::Error::new(io::ErrorKind::Other, "Failed to retrieve IP addresses"))
    }
}

/// Enables DHCP on a specified network interface.
fn enable_dhcp(interface_name: &str) -> io::Result<()> {
    // Validate that the interface name is not empty.
    if interface_name.trim().is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Interface name cannot be empty"));
    }

    // Execute the `netsh` command to enable DHCP.
    let output = Command::new("netsh")
        .args(&[
            "interface",
            "ip",
            "set",
            "address",
            interface_name,
            "dhcp"
        ])
        .output()?;

    // Check if the command executed successfully.
    if output.status.success() {
        // Check if DHCP was already enabled on the interface.
        let output_str = String::from_utf8_lossy(&output.stdout);
        if output_str.contains("DHCP is already enabled") {
            println!("DHCP was already enabled on interface: {}", interface_name);
        } else {
            println!("Successfully enabled DHCP on interface: {}", interface_name);
        }
        Ok(())
    } else {
        // Print an error message if the command failed.
        let error_msg = String::from_utf8_lossy(&output.stderr);
        eprintln!("Failed to enable DHCP on interface: {}", interface_name);
        eprintln!("Error: {}", error_msg);
        Err(io::Error::new(io::ErrorKind::Other, "Failed to enable DHCP"))
    }
}

/// Sets a static IP address for a specified network interface.
fn set_static_ip(interface_name: &str, ip_address: &str, subnet_mask: &str, gateway: &str) -> io::Result<()> {
    // Validate that all required parameters are provided and not empty.
    if interface_name.trim().is_empty() || ip_address.trim().is_empty() || subnet_mask.trim().is_empty() || gateway.trim().is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "All parameters must be provided"));
    }

    // Execute the `netsh` command to set the static IP address.
    let output = Command::new("netsh")
        .args(&[
            "interface",
            "ip",
            "set",
            "address",
            interface_name,
            "static",
            ip_address,
            subnet_mask,
            gateway
        ])
        .output()?;

    // Check if the command executed successfully.
    if output.status.success() {
        println!("Successfully set static IP address on interface: {}", interface_name);
        Ok(())
    } else {
        // Print an error message if the command failed.
        let error_msg = String::from_utf8_lossy(&output.stderr);
        eprintln!("Failed to set static IP address on interface: {}", interface_name);
        eprintln!("Error: {}", error_msg);
        Err(io::Error::new(io::ErrorKind::Other, "Failed to set static IP address"))
    }
}

/// Main function to handle user interaction and execute the chosen command.
fn main() -> io::Result<()> {
    // Display the menu options to the user.
    println!("Choose an option:");
    println!("1. List all IP addresses");
    println!("2. Enable DHCP on an interface");
    println!("3. Set a static IP address on an interface");

    // Read the user's choice.
    let mut choice = String::new();
    io::stdin().read_line(&mut choice)?;
    let choice = choice.trim();

    // Execute the corresponding function based on the user's choice.
    match choice {
        "1" => {
            println!("Listing all current IP addresses:");
            list_ip_addresses()?;
        }
        "2" => {
            println!("Enter the interface name:");
            let mut interface_name = String::new();
            io::stdin().read_line(&mut interface_name)?;
            let interface_name = interface_name.trim();
            enable_dhcp(interface_name)?;
        }
        "3" => {
            println!("Enter the interface name:");
            let mut interface_name = String::new();
            io::stdin().read_line(&mut interface_name)?;
            let interface_name = interface_name.trim();

            println!("Enter the IP address:");
            let mut ip_address = String::new();
            io::stdin().read_line(&mut ip_address)?;
            let ip_address = ip_address.trim();

            println!("Enter the subnet mask:");
            let mut subnet_mask = String::new();
            io::stdin().read_line(&mut subnet_mask)?;
            let subnet_mask = subnet_mask.trim();

            println!("Enter the gateway:");
            let mut gateway = String::new();
            io::stdin().read_line(&mut gateway)?;
            let gateway = gateway.trim();

            set_static_ip(interface_name, ip_address, subnet_mask, gateway)?;
        }
        _ => eprintln!("Invalid choice. Please select 1, 2, or 3."),
    }

    Ok(())
}
