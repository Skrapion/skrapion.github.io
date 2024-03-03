pub fn credits(credit: &str) -> (&str, &str) {
    match credit {
        "caitlyn" => ("Caitlyn MacMaster", ""),
        "sean" => ("Sean Montgomery", "https://artofsean.threadless.com/"),
        "jon" => ("Jonathan Stainton", "https://www.instagram.com/jonathanstainton/"),
        _ => ("", "")
    }
}
