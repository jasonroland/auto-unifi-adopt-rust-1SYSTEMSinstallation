use ssh2::Session;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

fn strip_ansi_codes(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\x1b' {
            // Skip ANSI escape sequence
            if chars.peek() == Some(&'[') {
                chars.next(); // consume '['
                // Skip until we hit a letter (end of escape sequence)
                while let Some(&next) = chars.peek() {
                    chars.next();
                    if next.is_ascii_alphabetic() {
                        break;
                    }
                }
            }
        } else if ch == '\r' {
            // Skip carriage returns
            continue;
        } else {
            result.push(ch);
        }
    }

    result
}

fn send_output(
    output_buffer: &mut String,
    chunk: &str,
    tx: &Option<tokio::sync::mpsc::UnboundedSender<String>>,
) {
    let cleaned = strip_ansi_codes(chunk);
    print!("{}", cleaned);
    std::io::stdout().flush().ok();
    output_buffer.push_str(&cleaned);
    if let Some(sender) = tx {
        sender.send(cleaned).ok();
    }
}

pub fn execute_adoption(
    ip: &str,
    username: &str,
    password: &str,
    controller_url: &str,
    progress_tx: Option<tokio::sync::mpsc::UnboundedSender<String>>,
) -> Result<String, String> {
    let mut output = String::new();

    // Helper to format errors with the output so far
    let make_error = |output: &str, error: &str| -> String {
        format!("{}\n{}", output, error)
    };

    // Show the SSH connection attempt
    let connection_msg = format!("{}@{}\n", username, ip);
    send_output(&mut output, &connection_msg, &progress_tx);

    // 1. Connect to device
    let tcp = match TcpStream::connect_timeout(
        &format!("{}:22", ip).parse().unwrap(),
        Duration::from_secs(10)
    ) {
        Ok(tcp) => tcp,
        Err(e) => return Err(make_error(&output, &format!("Connection failed: {}", e))),
    };

    if let Err(e) = tcp.set_read_timeout(Some(Duration::from_secs(30))) {
        return Err(make_error(&output, &format!("Failed to set timeout: {}", e)));
    }

    let mut sess = Session::new().map_err(|e| format!("Failed to create session: {}", e))?;
    sess.set_tcp_stream(tcp);

    if let Err(e) = sess.handshake() {
        return Err(make_error(&output, &format!("SSH handshake failed: {}", e)));
    }

    // 2. Authenticate
    if let Err(e) = sess.userauth_password(username, password) {
        return Err(make_error(&output, &format!("Authentication failed: {}", e)));
    }

    if !sess.authenticated() {
        return Err(make_error(&output, "Authentication failed: Invalid credentials"));
    }

    // 3. Open an interactive shell
    let mut channel = match sess.channel_session() {
        Ok(ch) => ch,
        Err(e) => return Err(make_error(&output, &format!("Failed to open channel: {}", e))),
    };

    // Request a PTY for interactive shell
    if let Err(e) = channel.request_pty("xterm", None, None) {
        return Err(make_error(&output, &format!("Failed to request PTY: {}", e)));
    }

    // Start the shell
    if let Err(e) = channel.shell() {
        return Err(make_error(&output, &format!("Failed to start shell: {}", e)));
    }

    // Give shell time to initialize
    std::thread::sleep(Duration::from_millis(1000));

    // Read initial shell output (welcome message, prompt, etc)
    let mut initial_buf = vec![0u8; 4096];
    sess.set_blocking(false);
    std::thread::sleep(Duration::from_millis(500));

    match channel.read(&mut initial_buf) {
        Ok(n) => {
            let initial_output = String::from_utf8_lossy(&initial_buf[0..n]);
            let cleaned = strip_ansi_codes(&initial_output);

            // Extract only the prompt (last line that ends with #)
            if let Some(prompt_line) = cleaned.lines().filter(|line| line.trim().ends_with('#')).last() {
                let prompt_output = format!("{}\n", prompt_line.trim());
                output.push_str(&prompt_output);
                if let Some(sender) = &progress_tx {
                    sender.send(prompt_output).ok();
                }
            }
        }
        _ => {}
    }

    // Now send the set-inform command
    let command = format!("set-inform {}/inform\n", controller_url);
    if let Err(e) = channel.write_all(command.as_bytes()) {
        return Err(make_error(&output, &format!("Failed to send command: {}", e)));
    }
    channel.flush().ok();

    // Wait a bit for command to execute
    std::thread::sleep(Duration::from_millis(500));

    // Read command output
    let mut output_buf = vec![0u8; 4096];
    let start = std::time::Instant::now();
    while start.elapsed() < Duration::from_secs(5) {
        match channel.read(&mut output_buf) {
            Ok(n) if n > 0 => {
                let cmd_output = String::from_utf8_lossy(&output_buf[0..n]);
                send_output(&mut output, &cmd_output, &progress_tx);
            }
            _ => {}
        }
        std::thread::sleep(Duration::from_millis(100));

        if channel.eof() {
            break;
        }
    }

    // Close the session
    channel.send_eof().ok();
    channel.wait_eof().ok();
    channel.close().ok();
    channel.wait_close().ok();

    Ok(output)
}
