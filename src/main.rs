use std::error::Error;
use std::{env, fs};
use headless_chrome::{Browser, types::PrintToPdfOptions};
use std::io;


fn pdf_to_url(url: &str,name: &str,size_str: &str ) -> Result<(), Box<dyn Error>> {

    enum PaperSize {
        A4,
        A5,
        A6,
        A7,
        Letter,
        Legal,
        Tabloid,
        Ledger,
    }
    
    impl PaperSize {
        pub fn width(&self) -> f64 {
            match self {
                PaperSize::A4 => 8.27,
                PaperSize::A5 => 5.83,
                PaperSize::A6 => 4.13,
                PaperSize::A7 => 2.91,
                PaperSize::Letter => 8.5,
                PaperSize::Legal => 8.5,
                PaperSize::Tabloid => 11.0,
                PaperSize::Ledger => 17.0,
            }
        }
    
        pub fn height(&self) -> f64 {
            match self {
                PaperSize::A4 => 11.69,
                PaperSize::A5 => 8.27,
                PaperSize::A6 => 5.83,
                PaperSize::A7 => 4.13,
                PaperSize::Letter =>  11.0,
                PaperSize::Legal =>  14.0,
                PaperSize::Tabloid =>  17.0,
                PaperSize::Ledger =>  11.0,
            }
        }
    }
    
    // let paper_size = PaperSize::A5;
    impl PaperSize {
        pub fn from_string(size_str: &str) -> Option<Self> {
            match size_str {
                "A4" => Some(PaperSize::A4),
                "A5" => Some(PaperSize::A5),
                "A6" => Some(PaperSize::A6),
                "A7" => Some(PaperSize::A7),
                "LETTER" => Some(PaperSize::Letter),
                "LEGAL" => Some(PaperSize::Legal),
                "TABLOID" => Some(PaperSize::Tabloid),
                "LEDGER" => Some(PaperSize::Ledger),
                _ => None,
            }
        }
    }
    let paper_size = PaperSize::from_string(size_str).unwrap_or(PaperSize::A4);
    let width = paper_size.width();
    let height = paper_size.height();


    let pdf_options = PrintToPdfOptions {
        display_header_footer: None,
        print_background: Some(true),
        landscape: None,
        scale: None,
        paper_width: Some(width),
        paper_height: Some(height),
        margin_top: None,
        margin_bottom: None,
        margin_left: None,
        margin_right: None,
        page_ranges: None,
        ignore_invalid_page_ranges: None,
        header_template: None,
        footer_template: None,
        prefer_css_page_size: None,
        transfer_mode: None,
    };
    let browser = Browser::default()?;
    let tab = browser.new_tab()?;

    let pdf = tab
        .navigate_to(url)?
        .wait_until_navigated()?
        .print_to_pdf(Some(pdf_options))?;
    let newpath = "./pdf_folder/".to_string() + &name + ".pdf";
    fs::write(newpath, pdf)?;
    println!("PDF successfully created from internet web page.");

    Ok(())
}

fn main() {

    let mut url = String::new();
    // url = "https://en.wikipedia.org/wiki/WebKit".to_string();
    println!("enter url");
    io::stdin().read_line(&mut url).expect("Failed to read line");
    url = url.trim().to_string();

    let mut name = String::new();
    // name = "test".to_string();
    println!("enter name");
    io::stdin().read_line(&mut name).expect("Failed to read line");
    name = name.trim().to_string();

    let mut size_str = String::new();
    // size_str = "A5".to_string();
    println!("enter print_size. Options: A4, A5, A6, A7, Letter, Legal, Tabloid, Ledger,");
    io::stdin().read_line(&mut size_str).expect("Failed to read line");
    size_str = size_str.trim().to_string().to_uppercase();


    // test site
    // https://en.wikipedia.org/wiki/WebKit 

    
    pdf_to_url(&url,&name,&size_str);
}


