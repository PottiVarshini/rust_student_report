# 📝 Student Report Card Generator (Rust + PDF)

A simple CLI tool written in Rust that:
- Accepts a student's name, total marks, and number of subjects
- Calculates the average
- Assigns a grade based on the average
- Generates a clean PDF report card

---

## 🚀 Features

✅ Terminal-based input  
✅ Grade assignment using match expressions  
✅ Cleanly formatted **PDF output** using `printpdf` crate  
✅ Automatically saved as `report_card.pdf`

---

## 📊 Grade Scheme

| Average Score | Grade |
|---------------|-------|
| 90+           | A     |
| 75 - 89       | B     |
| 60 - 74       | C     |
| Below 60      | D     |

---

## 🛠 Technologies Used

- [Rust](https://www.rust-lang.org/)
- [`printpdf`](https://crates.io/crates/printpdf) — to generate PDF files

---

