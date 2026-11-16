2. README.md (วิธีติดตั้ง วิธีรัน และวิธีสแกน Semgrep)
คัดลอกข้อความด้านล่างไปบันทึกเป็นไฟล์ README.md ในโฟลเดอร์โปรเจกต์ได้ทันที
# โปรแกรมตรวจสอบรหัสผ่านตามแนวทาง OWASP (Node.js & Rust)

โปรแกรมตรวจสอบความถูกต้องและความปลอดภัยของรหัสผ่านพัฒนาขึ้น 2 เวอร์ชัน ได้แก่ **JavaScript (Node.js)** และ **Rust** โดยอ้างอิงแนวทางความปลอดภัยของ OWASP (เน้นความยาวขั้นต่ำ 15 ตัวอักษร, ไม่บังคับชุดอักขระ, รองรับช่องว่างและอักขระ Unicode โดยไม่ตัดช่องว่างหัวท้าย, รองรับความยาว 64 ตัวอักษรขึ้นไป และไม่พิมพ์หรือบันทึกรหัสผ่านซ้ำ)

---

## 1. ข้อกำหนดของระบบและการติดตั้ง (Installation)

### 1.1 ติดตั้ง Node.js (สำหรับเวอร์ชัน JavaScript)
* ดาวน์โหลดและติดตั้ง **Node.js** (แนะนำเวอร์ชัน LTS) ได้จาก [nodejs.org](https://nodejs.org/)
* ตรวจสอบการติดตั้งผ่าน Terminal / Command Prompt:
  
bash
  node -v
  

* หมายเหตุ: โค้ด password_validator.js ใช้โมดูลมาตรฐาน (`readline`) ของ Node.js จึงไม่ต้องติดตั้งแพ็กเกจเพิ่มเติมด้วย npm

### 1.2 ติดตั้ง Rust (สำหรับเวอร์ชัน Rust)
* ดาวน์โหลดและติดตั้ง **Rust** ผ่าน rustup ได้จาก [rust-lang.org](https://www.rust-lang.org/tools/install)
* ตรวจสอบการติดตั้งคอมไพเลอร์ผ่าน Terminal / Command Prompt:
  
bash
  rustc --version
  

* หมายเหตุ: โค้ด main.rs ใช้ไลบรารีมาตรฐาน (`std::io`) จึงสามารถคอมไพล์ได้ทันทีโดยไม่ต้องติดตั้ง Crate ภายนอก

### 1.3 ติดตั้ง Semgrep (สำหรับสแกนความปลอดภัยของโค้ด)
เลือกติดตั้งผ่านตัวจัดการแพ็กเกจวิธีใดวิธีหนึ่ง:
* **ติดตั้งผ่าน Python (pip):**
  
bash
  pip install semgrep
  

* **ติดตั้งผ่าน Homebrew (macOS / Linux):**
  
bash
  brew install semgrep
  

* ตรวจสอบการติดตั้ง:
  
bash
  semgrep --version
  


---

## 2. วิธีการรันโปรแกรม (How to Run)

### 2.1 รันเวอร์ชัน JavaScript (Node.js)
เปิด Terminal ในโฟลเดอร์ที่เก็บไฟล์ password_validator.js แล้วรันคำสั่ง:
bash
node password_validator.js

### 2.2 รันเวอร์ชัน Rust
เปิด Terminal ในโฟลเดอร์ที่เก็บไฟล์ main.rs คอมไพล์โค้ดและรันไฟล์ผลลัพธ์:
* **บน macOS / Linux:**
  
bash
  rustc main.rs
  ./main
  

* **บน Windows (Command Prompt / PowerShell):**
  
bash
  rustc main.rs
  .\main.exe
  


---