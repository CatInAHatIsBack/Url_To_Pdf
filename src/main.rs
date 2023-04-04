use std::error::Error;
use std::{env, fs};
use headless_chrome::Browser;
use std::io;

fn pdf_to_url(url: &str,name: &str) -> Result<(), Box<dyn Error>> {
    let browser = Browser::default()?;

    let tab = browser.new_tab()?;

    let pdf = tab
        .navigate_to(url)?
        .wait_until_navigated()?
        .print_to_pdf(None)?;
    let newpath = "./pdf_folder/".to_string() + &name + ".pdf";
    fs::write(newpath, pdf)?;
    println!("PDF successfully created from internet web page.");

    
    Ok(())
}
fn main() {
    
    let mut url = String::new();
    println!("enter url");
    io::stdin().read_line(&mut url).expect("Failed to read line");
    url = url.trim().to_string();

    let mut name = String::new();
    println!("enter name");
    io::stdin().read_line(&mut name).expect("Failed to read line");
    name = name.trim().to_string();

    // test site
    // https://en.wikipedia.org/wiki/WebKit 

    
    pdf_to_url(&url,&name);
}
