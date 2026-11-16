const readline = require('readline');

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout
});

function validateEmail(email) {
  return /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email);
}

function validatePassword(password) {
  const reasons = [];

  // นับจำนวนอักขระ Unicode จริง (รองรับภาษาไทยและอีโมจิอย่างแม่นยำ)
  const charLength = Array.from(password).length;

  // ตรวจสอบความยาวขั้นต่ำ 15 ตัวอักษร (ไม่บังคับชุดอักขระ และรองรับความยาว 64+ ตัวอักษร)
  if (charLength < 15) {
    reasons.push(
      `รหัสผ่านมีความยาว ${charLength} ตัวอักษร ซึ่งน้อยกว่าเกณฑ์ขั้นต่ำ (ต้องมีอย่างน้อย 15 ตัวอักษร)`
    );
  }

  return {
    isValid: reasons.length === 0,
    charLength,
    reasons
  };
}

rl.question('กรุณากรอก Username (Email): ', (usernameInput) => {
  const username = usernameInput.trim();

  rl.question('กรุณากรอก Password: ', (passwordInput) => {
    // ตัดเฉพาะอักขระขึ้นบรรทัดใหม่ (\r หรือ \n) จากการกด Enter โดยไม่ตัดช่องว่างหัวท้าย
    const password = passwordInput.replace(/\r?\n$/, '');

    const isEmailValid = validateEmail(username);
    const result = validatePassword(password);

    console.log('\n--- ผลการตรวจสอบ ---');
    console.log(`บัญชีผู้ใช้ (Email): ${username} (${isEmailValid ? 'รูปแบบอีเมลถูกต้อง' : 'รูปแบบอีเมลไม่ถูกต้อง'})`);
    console.log(`ความยาวรหัสผ่านที่ตรวจพบ: ${result.charLength} ตัวอักษร`);

    if (result.isValid) {
      console.log('ผลการตรวจสอบรหัสผ่าน: ผ่าน (PASS)');
      console.log('เหตุผล: รหัสผ่านมีความยาวตั้งแต่ 15 ตัวอักษรขึ้นไป และตรงตามข้อกำหนด');
    } else {
      console.log('ผลการตรวจสอบรหัสผ่าน: ไม่ผ่าน (FAIL)');
      console.log('เหตุผล:');
      result.reasons.forEach((reason, index) => {
        console.log(`  ${index + 1}. ${reason}`);
      });
    }

    // ไม่พิมพ์ค่ารหัสผ่านซ้ำ และไม่มีการเขียนข้อมูลลงไฟล์
    rl.close();
  });
});
