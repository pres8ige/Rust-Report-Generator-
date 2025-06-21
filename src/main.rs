use std::io;
use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

fn main() {
    // Display REPORT ASCII banner
    println!();
    println!(" ____                       _   ");
    println!("|  _ \\ ___ _ __   ___  _ __| |_ ");
    println!("| |_) / _ \\ '_ \\ / _ \\| '__| __|");
    println!("|  _ <  __/ |_) | (_) | |  | |_ ");
    println!("|_| \\_\\___| .__/ \\___/|_|   \\__|");
    println!("          |_|                   ");
    println!();

    // Take student name
    let mut name = String::new();
    println!("Enter student name:");
    io::stdin().read_line(&mut name).unwrap();
    let name = name.trim().to_string();

    // Number of subjects
    let mut num_subjects_input = String::new();
    println!("Enter number of subjects:");
    io::stdin().read_line(&mut num_subjects_input).unwrap();
    let num_subjects: usize = num_subjects_input.trim().parse().unwrap();

    // Subject names, marks obtained and out of
    let mut subjects: Vec<String> = Vec::new();
    let mut marks: Vec<f64> = Vec::new();
    let mut out_of: Vec<f64> = Vec::new();

    for i in 0..num_subjects {
        let mut subject_name = String::new();
        println!("Enter name of subject {}:", i + 1);
        io::stdin().read_line(&mut subject_name).unwrap();
        let subject_name = subject_name.trim().to_string();
        subjects.push(subject_name);

        let mut mark_input = String::new();
        println!("Enter marks obtained in {}:", subjects[i]);
        io::stdin().read_line(&mut mark_input).unwrap();
        let mark: f64 = mark_input.trim().parse().unwrap();
        marks.push(mark);

        let mut out_of_input = String::new();
        println!("Enter total marks (out of) for {}:", subjects[i]);
        io::stdin().read_line(&mut out_of_input).unwrap();
        let out: f64 = out_of_input.trim().parse().unwrap();
        out_of.push(out);
    }

    // Calculate total and average (considering obtained marks)
    let total_marks: f64 = marks.iter().sum();
    let total_out_of: f64 = out_of.iter().sum();
    let average = total_marks / num_subjects as f64;
    let grade = assign_grade(average / (total_out_of / num_subjects as f64) * 100.0); // percentage-based grade

    // Print report card on console
    println!("\n===== Report Card =====");
    println!("Name: {}", name);
    for i in 0..num_subjects {
        println!("{}: {:.2} / {:.2}", subjects[i], marks[i], out_of[i]);
    }
    println!("Total Marks: {:.2} / {:.2}", total_marks, total_out_of);
    println!("Average Marks: {:.2}", average);
    println!("Grade: {}", grade);

    // Generate PDF report card
    generate_pdf(&name, &subjects, &marks, &out_of, total_marks, total_out_of, average, &grade);
    println!("📄 Report card PDF generated successfully!");
}

// Function to assign grade based on percentage average
fn assign_grade(percentage_avg: f64) -> String {
    if percentage_avg >= 90.0 {
        "A".to_string()
    } else if percentage_avg >= 75.0 {
        "B".to_string()
    } else if percentage_avg >= 60.0 {
        "C".to_string()
    } else {
        "D".to_string()
    }
}

// Function to generate a styled PDF report card
fn generate_pdf(name: &str, subjects: &Vec<String>, marks: &Vec<f64>, out_of: &Vec<f64>,
                total: f64, total_out_of: f64, average: f64, grade: &str) {
    let (doc, page1, layer1) = PdfDocument::new("Report Card", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    let title_font = doc.add_builtin_font(BuiltinFont::HelveticaBold).unwrap();
    let content_font = doc.add_builtin_font(BuiltinFont::TimesRoman).unwrap();

    // Draw border rectangle
    let border_shape = Line {
        points: vec![
            (Point::new(Mm(15.0), Mm(280.0)), false),
            (Point::new(Mm(195.0), Mm(280.0)), false),
            (Point::new(Mm(195.0), Mm(50.0)), false),
            (Point::new(Mm(15.0), Mm(50.0)), false),
            (Point::new(Mm(15.0), Mm(280.0)), false),
        ],
        is_closed: true,
        has_fill: false,
        has_stroke: true,
        is_clipping_path: false,
    };
    current_layer.add_shape(border_shape);

    // Title — centered at top
    current_layer.use_text("📜 Student Report Card 📜", 24.0, Mm(40.0), Mm(270.0), &title_font);

    let mut y_position = 250.0;

    // Student name
    current_layer.use_text(&format!("👤 Name: {}", name), 16.0, Mm(30.0), Mm(y_position), &content_font);
    y_position -= 15.0;

    // Subject-wise marks
    current_layer.use_text("📚 Subject-wise Marks:", 16.0, Mm(30.0), Mm(y_position), &content_font);
    y_position -= 12.0;

    for i in 0..subjects.len() {
        current_layer.use_text(
            &format!("   📝 {}: {:.2} / {:.2}", subjects[i], marks[i], out_of[i]),
            14.0,
            Mm(35.0),
            Mm(y_position),
            &content_font,
        );
        y_position -= 12.0;
    }

    y_position -= 5.0;

    // Total, average, grade
    current_layer.use_text(&format!("📊 Total: {:.2} / {:.2}", total, total_out_of), 16.0, Mm(30.0), Mm(y_position), &content_font);
    y_position -= 15.0;
    current_layer.use_text(&format!("📈 Average Marks: {:.2}", average), 16.0, Mm(30.0), Mm(y_position), &content_font);
    y_position -= 15.0;
    current_layer.use_text(&format!("🏆 Grade: {}", grade), 16.0, Mm(30.0), Mm(y_position), &content_font);

    // Footer
    current_layer.use_text("Generated by Rust Report System 📑", 10.0, Mm(50.0), Mm(55.0), &content_font);

    // Save the PDF
    let file = File::create("report_card.pdf").unwrap();
    doc.save(&mut BufWriter::new(file)).unwrap();
}
