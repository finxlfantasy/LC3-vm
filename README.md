# LC3-vm

# Abstract
Wanted to improve my understanding of assembly and assemblers. What better way than writing a LC3-VM? I followed Rodrigo Araujo guide. 

# Components
This program consists of two things: registers and memory. 

* **Memory** is where we will store a programs binaries (our array of bytes). These will hold the instrution and indices of given address. 

* **Registers** will store data that our opcode (Operation Code) will operate on. In total we have 7 general-purpose registers, whilst on top of that we have 2 special purpose registers. 

![image](https://github.com/finxlfantasy/LC3-vm/assets/113019900/b63dba2b-bc92-4db9-95a6-abe0e6f1b35e)


To run an LC-3 program: cargo run -- examples/<program_name>.obj
