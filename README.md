# Solana Counter Program (Rust)

A simple Solana smart contract (program) written in **Rust** that stores a counter value inside a Solana account and allows users to:

- Increment the counter by a custom value
- Decrement the counter by a custom value (never goes below 0)
- Update the counter to a specific value
- Reset the counter to 0

This project is built as part of my learning journey in **Solana development + Rust**.

---

## 📌 Features

✅ Counter stored on-chain (inside account data)  
✅ Custom increment and decrement values (u32 input)  
✅ Safe decrement (if decrement > counter → counter becomes 0)  
✅ Uses **Borsh** serialization for account state  

---

## 🧠 How It Works

### Counter State (Stored in Account)
The counter is stored in a struct:

```rust
pub struct CounterAccount {
    pub counter: u32,
}

Instructions Supported
Tag (first byte)	Instruction	Data
0	Increment	+ 4 bytes (u32 value)
1	Decrement	+ 4 bytes (u32 value)
2	Update	+ 4 bytes (u32 value)
3	Reset	no extra bytes
🧾 Instruction Data Examples
Increment by 10
[0, 10, 0, 0, 0]

Decrement by 3
[1, 3, 0, 0, 0]

Update to 50
[2, 50, 0, 0, 0]

Reset
[3]

🧪 Run Tests

Make sure you're inside the project folder, then run:

cargo test

🛠️ Build
cargo build


If you're compiling for Solana BPF, you may need Solana toolchain installed depending on your setup.

📚 Learning Notes

This project helped me learn:

Solana program entrypoints

Account data storage

Instruction decoding (byte parsing)

Rust enums, structs, and pattern matching

Safe math operations in Rust

👨‍💻 Author

Saleh
Mechatronics Engineer | Data Science & Analytics | FinTech Master’s Student
Interested in Machine Learning, Crypto, Blockchain, and Prediction Models 🚀
