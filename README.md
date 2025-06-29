# 📜 Rust Report Card Generator

A simple, interactive **Rust console application** that collects student details, subject-wise marks (with total marks per subject), calculates total and average, assigns a grade, and generates a **beautiful PDF report card** using the `printpdf` crate.

---

## 📌 Features
- Clean ASCII art header on program start 🖥️
- Accepts:
  - Student name
  - Number of subjects
  - Subject names
  - Marks obtained and marks out of for each subject
- Calculates:
  - Total marks
  - Average marks
  - Grade based on percentage average:
    - **A:** 90%+
    - **B:** 75–89%
    - **C:** 60–74%
    - **D:** Below 60%
- Displays report card on the console 📊
- Generates a professional-looking **PDF report card** with:
  - Subject-wise marks (obtained / out of)
  - Total, average, and grade
  - Decorative border and icons

---

## 📦 Dependencies

Add this to your `Cargo.toml`:
```toml
[dependencies]
printpdf = "0.4.1"
