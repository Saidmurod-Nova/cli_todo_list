use std::io::{self, Write};

// 1. Command enum'ini yarating
enum Command {
    Add(String),
    List,
    Remove(usize),
    Quit,
}

// 2. Foydalanuvchi kiritgan matnni Command'ga aylantiruvchi funksiya
fn parse_command(input: &str) -> Result<Command, &'static str> {
    let parts: Vec<&str> = input.trim().splitn(2, ' ').collect();
    match parts[0].to_lowercase().as_str() {
        "quit" => Ok(Command::Quit),
        "list" => Ok(Command::List),
        "add" => {
            if parts.len() > 1 {
                Ok(Command::Add(parts[1].to_string()))
            } else {
                Err("Iltimos, vazifa matnini kiriting. Masalan: add Non sotib olish")
            }
        }
        "remove" => {
            if parts.len() > 1 {
                // Bu yerda parts[1] ni usize ga o'girish kerak
                // .parse::<usize>() ishlatib ko'r. Agar xato bo'lsa, Err qaytar
                // Masalan: parts[1].parse::<usize>().map_err(|_| "Noto'g'ri raqam")
                parts[1].parse::<usize>().map_err(|_| "Noto'g'ri raqam").map(|i| Command::Remove(i))
            } else {
                Err("Iltimos, o'chiriladigan vazifa raqamini kiriting. Masalan: remove 1")
            }
        }
        _ => Err("Noma'lum buyruq. Mavjud buyruqlar: add, list, remove, quit"),
    }
}

fn main() {
    let mut todos: Vec<String> = Vec::new();
    println!("🦀 Rust Todo List'ga xush kelibsiz!");
    println!("Buyruqlar: add <vazifa>, list, remove <raqam>, quit");

    loop {
        print!("\n> ");
        io::stdout().flush().unwrap(); // Ekranga darhol chiqarish uchun

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // 3. parse_command ni chaqir va natijani match bilan tekshir
        match parse_command(&input) {
            Ok(Command::Add(task)) => {
                todos.push(task);
                println!("✅ Vazifa qo'shildi!");
            }
            Ok(Command::List) => {
                if todos.is_empty() {
                    println!("📭 Hozircha vazifalar yo'q.");
                } else {
                    println!("📋 Vazifalar ro'yxati:");
                    // Iterator ishlatib, har bir vazifani raqami bilan chiqar:
                    // masalan: (index, task) in todos.iter().enumerate()
                    for (i, task) in todos.iter().enumerate() {
                        println!("{}. {}", i + 1, task);
                    }
                }
            }
            Ok(Command::Remove(index)) => {
                // Eslatma: foydalanuvchi 1 dan boshlab kiritadi, lekin Vec indeksi 0 dan boshlanadi.
                // Shuning uchun index - 1 qilish kerak.
                // Agar index xato bo'lsa (masalan, ro'yxatda yo'q bo'lsa), xato xabarini chiqar.
                if index <= todos.len() && index > 0 {
                    todos.remove(index - 1);
                    println!("✅ Vazifa o'chirildi!");
                } else {
                    println!("❌ Xato: Bunday raqamli vazifa yo'q.");
                }
            }
            Ok(Command::Quit) => {
                println!("Xayr! 👋");
                break;
            }
            Err(e) => {
                println!("❌ Xato: {}", e);
            }
        }
    }
}