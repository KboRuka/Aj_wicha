use std::io::{self, Write};

struct PasswordValidationResult {
    is_valid: bool,
    char_length: usize,
    reasons: Vec<String>,
}

fn validate_email(email: &str) -> bool {
    let parts: Vec<&str> = email.split('@').collect();
    parts.len() == 2
        && !parts[0].is_empty()
        && parts[1].contains('.')
        && !parts[1].starts_with('.')
        && !parts[1].ends_with('.')
}

fn validate_password(password: &str) -> PasswordValidationResult {
    let mut reasons = Vec::new();

    // นับจำนวนอักขระ Unicode จริงด้วย .chars().count() (ไม่นับเป็นจำนวน Byte)
    let char_length = password.chars().count();

    // ตรวจสอบความยาวขั้นต่ำ 15 ตัวอักษร (ไม่บังคับชุดอักขระ และรองรับความยาว 64+ ตัวอักษร)
    if char_length < 15 {
        reasons.push(format!(
            "รหัสผ่านมีความยาว {} ตัวอักษร ซึ่งน้อยกว่าเกณฑ์ขั้นต่ำ (ต้องมีอย่างน้อย 15 ตัวอักษร)",
            char_length
        ));
    }

    PasswordValidationResult {
        is_valid: reasons.is_empty(),
        char_length,
        reasons,
    }
}

fn main() {
    let mut username_input = String::new();
    let mut password_input = String::new();

    print!("กรุณากรอก Username (Email): ");
    io::stdout().flush().expect("ไม่สามารถแสดงผลข้อความได้");
    io::stdin()
        .read_line(&mut username_input)
        .expect("เกิดข้อผิดพลาดในการรับค่า Username");
    let username = username_input.trim();

    print!("กรุณากรอก Password: ");
    io::stdout().flush().expect("ไม่สามารถแสดงผลข้อความได้");
    io::stdin()
        .read_line(&mut password_input)
        .expect("เกิดข้อผิดพลาดในการรับค่า Password");

    // ตัดเฉพาะอักขระขึ้นบรรทัดใหม่ (\r และ \n) จากการกด Enter โดยไม่ตัดช่องว่างหัวท้าย
    let password = password_input.trim_end_matches(&['\r', '\n'][..]);

    let is_email_valid = validate_email(username);
    let result = validate_password(password);

    println!("\n--- ผลการตรวจสอบ ---");
    println!(
        "บัญชีผู้ใช้ (Email): {} ({})",
        username,
        if is_email_valid { "รูปแบบอีเมลถูกต้อง" } else { "รูปแบบอีเมลไม่ถูกต้อง" }
    );
    println!("ความยาวรหัสผ่านที่ตรวจพบ: {} ตัวอักษร", result.char_length);

    if result.is_valid {
        println!("ผลการตรวจสอบรหัสผ่าน: ผ่าน (PASS)");
        println!("เหตุผล: รหัสผ่านมีความยาวตั้งแต่ 15 ตัวอักษรขึ้นไป และตรงตามข้อกำหนด");
    } else {
        println!("ผลการตรวจสอบรหัสผ่าน: ไม่ผ่าน (FAIL)");
        println!("เหตุผล:");
        for (index, reason) in result.reasons.iter().enumerate() {
            println!("  {}. {}", index + 1, reason);
        }
    }
    // ไม่พิมพ์ค่ารหัสผ่านซ้ำ และไม่มีการเขียนข้อมูลลงไฟล์
}
