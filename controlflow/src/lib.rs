pub fn status_match() {
    let status_code = 200;

    let semantic_status = match status_code {
        200 => "OK",
        404 => "Not found",
        500..=511 => "Error",
        _ => "Not implemented"
    };

    println!("Current status: {} {}", status_code, semantic_status);
}
