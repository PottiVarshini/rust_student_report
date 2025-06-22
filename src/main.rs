use std::io;
use std::fs::File;
use printpdf::*;
use std::io::BufWriter;
fn cal_avg(total_marks:f64, num_subjects:u32) -> f64 {
    total_marks / num_subjects as f64
}
fn assign_grade(average: f64) -> &'static str {
    match average {
        avg if avg >= 90.0 => "A",
        avg if avg >= 75.0 => "B",
        avg if avg >= 60.0 => "C",
        _ => "D",
    }
}
fn generate_pdf(name: &str, total: f64, subjects: u32, average: f64, grade: &str) {
    let (doc, page1, layer1) = PdfDocument::new("Student Report", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);
    let font_bold = doc.add_builtin_font(BuiltinFont::HelveticaBold).unwrap();
    let font_regular = doc.add_builtin_font(BuiltinFont::Helvetica).unwrap();

    let start_x = Mm(20.0);
    let mut start_y = Mm(250.0);
    let line_spacing = Mm(10.0);

    current_layer.use_text("Student Report Card", 20.0, start_x, start_y, &font_bold);
    start_y -= line_spacing + Mm(5.0);

    let lines: Vec<String> = vec![
        format!("Name           : {}", name),
        format!("Total Marks    : {}", total),
        format!("No. of Subjects: {}", subjects),
        format!("Average        : {:.2}", average),
        format!("Grade          : {}", grade),
    ];

    for line in lines {
        current_layer.use_text(line, 14.0, start_x, start_y, &font_regular);
        start_y -= line_spacing;
    }
    doc.save(&mut BufWriter::new(File::create("report_card.pdf").unwrap())).unwrap();
}
fn main() {
    let mut name = String::new();
    let mut total_marks = String::new();
    let mut subjects = String::new();

    println!("Enter Student Name:");
    io::stdin()
        .read_line(&mut name).unwrap();
    name = name.trim().to_string();

    println!("Enter Total Marks:");
    io::stdin()
        .read_line(&mut total_marks).unwrap();
    let total_marks: f64 = total_marks.trim().parse().expect("Enter valid number");

    println!("Enter Number of Subjects:");
    io::stdin()
        .read_line(&mut subjects).unwrap();
    let subjects: u32 = subjects.trim().parse().expect("Enter valid number");

    let average = cal_avg(total_marks,subjects);
    let grade = assign_grade(average);

    println!("\n--- Report Card ---");
    println!("Name           : {}", name);
    println!("Total Marks   : {}", total_marks);
    println!("No:Of Subjects: {}", subjects);
    println!("Average        : {:.2}", average);
    println!("Grade          : {}", grade);

    generate_pdf(&name, total_marks, subjects, average, grade);
    println!("\nPDF Of the Student Report Card is saved as 'report_card.pdf'");
}
