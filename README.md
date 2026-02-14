# **Hyperkit-toolbox**
**HyperKit Toolbox** is an all-in-one toolkit containing every tool 
you'll ever need, for any day of your life — from cryptography 
and networking utilities to much more, all integrated within a 
single, unified interface featuring both a CLI and an interactive REPL.

It was built out of a simple frustration: why should you have to 
install dozens of separate tools just to get through your day? 
With HyperKit, you install once and get everything — no more 
hunting down utilities, no more dependency headaches, just open 
it and get to work.

**HyperKit** is for everyone — **everyday users**, **crypto professionals**, 
**networking engineers**, **security researchers**, and anyone who values 
having the right tool at the right moment.

What sets **HyperKit** apart is its **simplicity** and **raw performance** 
— every tool is built-in, written in **Rust**, giving you **speed** and 
**reliability** out of the box.

## What areas does HyperKit cover?
- [X] cryptography   "hashing and encryption and different types of encoding".
- [X] networking     "port scaning , and much more in the way".
- [X] system utilities   "extracting information about the device and the operating system , and much more".
- [X] files utilities   "extracting zip and tar archives , and making them , with alot more in the way". 
- [X] science utilities   "calculator , with plans to add more in the near future"

## cryptography
>**Cryptography** is the science of securing information by transforming it into a form that only intended recipients can read and understand.
---
| tool | Description | supported types | modes|
| ----|------------- | ----------------| ------|
| **seal**| encrypting and hashing data| Aes256Gcm/ChaCha20Poly1305/MD5/Sha256/Sha512|--seal to hash and encrypt /--unseal to decrypt|                
| **transmute** | encoding and decoding| Base64-ST/Base64-PD/Base64-Url/hex/HEX/url|--enc to encode /--dec to decode |
## networking
>**Networking** is the practice of connecting computers and devices together so they can communicate and share data.
---
| tool | Description| supported types|modes|
|------|------------| --------------|-------|
|**fang**| port scaning tool | TCP/UPD| --most-popular-ports— scans the most commonly used TCP/UDP ports/ --top100 —scans the top 100 TCP ports/ --top1000 —scans the top 1000 TCP ports/ --full-tcp —scans all TCP ports| Results can be filtered to show open ports only using --open-only.|

## system utilities
>**System utilities** are tools that help you monitor, manage, and interact with your operating system and hardware.
---
|tool|Description| supported types|
|---|------------|----------------|
|**tree**| Recursively walks a directory and builds a visual tree structure of its contents, printing it to the console |all|
|**yank**| extracting information about the device and the software| mem/temp/cpu/bios/os/battery/disk|

## File utilities
>**File utilities** are tools that help you work with files and directories on your system.
---
|tool|Description| modes|
|----|-----------|-----|
|**tar**|Archives and extracts TAR files.| --load —packs a file or directory into a .tar archive / --Unload — unpacks a .tar archive into a destination directory|
|**zip**|A full-featured ZIP utility.| --New-Zip  — creates a new ZIP and adds up to three files, either as empty placeholders (-N) or with their actual contents included (-E)/ --Zip-All — recursively compresses an entire directory into a ZIP, preserving structure /--extract — unpacks a ZIP archive into a destination directory, restoring files and Unix permissions|
|**indicate**|Inspects a file and displays detailed metadata about it, including the file name, type, MIME type, permissions (read-only or editable), full path, Unix file mode, last modified timestamp, and size in KiB. Useful for quickly understanding what a file is and how it's configured without opening it.|

## science utilities
>**Science utilities** are tools that help you perform mathematical, statistical, and scientific calculations and conversions.
---

|tool|Description|
|----|-----------|
|**calc**| Evaluates a mathematical expression and prints the result to the console. It accepts any valid math expression as a string — such as 2 + 2 or (10 * 5) / 2 — computes it using an expression evaluator, and displays the result styled with the user's configured username and current path. If the expression is invalid or can't be evaluated, it prints a descriptive error instead.

## Basic utilities
>**Basic utilities** are the foundational, everyday tools that don't fit neatly into a specific category but are essential for general use.
---
|tool|Description|modes|
|----|-----------|-----|
|**help**|Displays help information based on the flag provided.|--commands — lists all available REPL commands with their syntax and usage / --built-in-apps — lists all built-in tools like calc, tar, zip, tree, yank, and more / --about — prints a short description of what HyperKit is|
|**clean**|Clears the terminal screen by sending ANSI escape codes, giving you a fresh, empty console.|
|**go**| Changes the current working directory to the given path, effectively navigating to a new location in the filesystem.|
|**wh**|Prints the current working directory, similar to pwd, displayed alongside the configured username.|
|**see**|Lists all files and directories inside the current working directory, similar to ls.|
|**peek**|Opens and prints the contents of a file to the console. If the file doesn't exist, it prompts the user asking whether they want to create it.|
|**mk**|Creates a new directory at the given path, similar to mkdir.|
|**burn**|Deletes a file or directory. If the target is a non-empty directory, it asks for confirmation before recursively deleting everything inside it.|
|**rn**|Renames a file or directory from one name to another.|
|**clone**|Copies a file from one path to another, similar to cp.|
|**forge**|Creates a new empty file at the given path, similar to touch.|
|**run**|Executes an external program and prints its output to the console.|
|**mv**|Moves a file or directory to a new location by copying it to the destination and then deleting the original.|
|**find**|Recursively searches the entire filesystem starting from root / for a file matching the given name, and prints its full path if found.|
|**ps**| Lists and inspects running processes.| -SF— searches for a specific process by PID and shows its name, disk usage, and memory usage / -A — lists all currently running processes with their PIDs and names|
|**stop**|Forcefully terminates a process by its PID using the SIGKILL signal.|
|**hostname**|Views or changes the system hostname| --show — displays the current hostname/--set — sets a new hostname|
|**history**|Manages the command history file.|--search — searches the history file for a keyword and prints every matching line with its line number|
