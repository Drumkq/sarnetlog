# ⚠️ Caution
**Tested on Windows 10, Steam version**

**Made for** *<s>educational</s>* **purposes :)**

# ⁉️ How to
You will need the **steam** version of the game and **unbanned** account before you can use it

1. Unpack the injector.exe and logger.dll into the same folder
2. Start the injector.exe
3. Start the game

*Once you have done these steps, all information about outgoing requests will be stored in `steamapps\common\Super Animal Royale\sarnetlog`*

### HTTP
> `output_jsons.txt` for **outgoing** HTTP requests
>
> `incoming_jsons.txt` for **incoming** HTTP requests

### WebSockets
> `output_lidgren.txt` for **outgoing** WebSockets stream
>
> `incoming_lidgren.txt` for **incoming** WebSockets stream

# 💡 Features
* ✅ Built-in simple injector
* ✅ Logging to a file
* ✅ Logging to the console
* ✅ Outgoing HTTP requests logging
* ❌ Incoming HTTP requests logging
* ❌ WebSockets outgoing stream logging
* ❌ WebSockets incoming stream logging

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
