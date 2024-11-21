# ⚠️ Caution
**Tested on Windows 10, Steam version**

**Made for educational purposes**

# 💉 How to use it?
You will need the steam version of the game and unbanned account before you can use it

1. Unpack the injector.exe and logger.dll into the same folder
2. Start the injector.exe
3. Start the game

Once you have done these steps, all information about outgoing requests will be stored in `..AppData/sarnetlogger/[timestamp]`.

# 💡 Features
* **[+]** Built-in simple injector
* **[-]** Logging to a file
* **[-]** Logging to the console

# 🛠️ Compilation guide
[Install](https://www.rust-lang.org/learn/get-started) Rust language on your computer

Download this repository or clone it using git:
```git
git clone https://github.com/Drumkq/sarnetlog.git sarnetlog/
```
Open terminal in `sarnetlog` folder and type this command:
```
# build injector
cd injector
cargo build
# build logger
cd ../logger
cargo build

```
