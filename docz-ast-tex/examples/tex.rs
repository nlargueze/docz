//! Example for tex

fn main() {
    let latex = r#"
\documentclass{article}
\begin{document}
Hello, world!
\end{document}
"#;

    let pdf_data: Vec<u8> = tectonic::latex_to_pdf(latex).expect("processing failed");
    println!("Output PDF size is {} bytes", pdf_data.len());
}
